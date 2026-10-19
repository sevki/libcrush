use crate::crush::types::*;
use std::slice;

/// Views a C array as a slice, treating a null pointer as empty.
///
/// # Safety
/// A non-null `ptr` must point to `len` initialized, live elements.
unsafe fn c_slice<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    if ptr.is_null() {
        &[]
    } else {
        // Safety: guaranteed by the caller.
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

/// Find root buckets in the CRUSH map (buckets that are not children of any other bucket)
/// Returns a Vec of root bucket IDs (as negative indices)
pub fn crush_find_roots_safe(map: &CrushMap) -> Result<Vec<i32>, CrushError> {
    let max_buckets = usize::try_from(map.max_buckets).unwrap_or(0);
    // Safety: a CrushMap's `buckets` array holds `max_buckets` bucket pointers,
    // each null or pointing to a live bucket whose `items` holds `size` ids.
    let buckets = unsafe { c_slice(map.buckets, max_buckets) };
    let buckets: Vec<Option<&CrushBucket>> =
        buckets.iter().map(|&b| unsafe { b.as_ref() }).collect();

    let mut referenced = vec![false; max_buckets];
    for b in buckets.iter().flatten() {
        // Safety: see above.
        let items = unsafe { c_slice(b.items, b.size as usize) };
        for &item_id in items.iter().filter(|&&id| id < 0) {
            let slot = usize::try_from(-1 - item_id).map_err(|_| CrushError::InvalidArgument)?;
            *referenced
                .get_mut(slot)
                .ok_or(CrushError::InvalidArgument)? = true;
        }
    }

    Ok(buckets
        .iter()
        .zip(&referenced)
        .enumerate()
        .filter(|(_, (b, referenced))| b.is_some() && !**referenced)
        .map(|(pos, _)| -1 - pos as i32)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wrapper::{BucketAlgorithm, Map};

    use crate::crush::builder::crush_bucket_add_item;

    /// Original c2rust-shaped implementation, kept as a test oracle.
    /// Find root buckets in the CRUSH map (buckets that are not children of any other bucket)
    /// Returns a Vec of root bucket IDs (as negative indices)
    fn crush_find_roots_unsafe_oracle(map: &CrushMap) -> Result<Vec<i32>, CrushError> {
        let max_buckets = map.max_buckets as usize;
        let mut ref_counts: Vec<i32> = vec![0; max_buckets];
        let mut root_count = map.max_buckets;

        // Count references to each bucket
        // Safety: CrushMap contains raw pointers from C FFI. We must use unsafe to dereference them.
        // The map structure guarantees buckets array has max_buckets elements.
        unsafe {
            for pos in 0..map.max_buckets {
                let b: *mut CrushBucket = *(map.buckets).offset(pos as isize);
                if b.is_null() {
                    root_count -= 1;
                } else {
                    for i in 0..(*b).size {
                        let item_id = *((*b).items).offset(i as isize);
                        if item_id < 0 {
                            let item: i32 = -1 - item_id;
                            if item >= map.max_buckets {
                                return Err(CrushError::InvalidArgument);
                            }
                            if ref_counts[item as usize] == 0 {
                                root_count -= 1;
                            }
                            ref_counts[item as usize] += 1;
                        }
                    }
                }
            }
        }

        // Collect root buckets (those with zero references)
        let mut roots = Vec::with_capacity(root_count as usize);
        // Safety: Same as above - accessing raw pointers from C FFI
        unsafe {
            for pos in 0..map.max_buckets {
                if !(*(map.buckets).offset(pos as isize)).is_null() && ref_counts[pos as usize] == 0
                {
                    roots.push(-1 - pos);
                }
            }
        }

        assert_eq!(
            roots.len(),
            root_count as usize,
            "Root count mismatch: found {} roots, expected {}",
            roots.len(),
            root_count
        );

        Ok(roots)
    }

    /// The in-tree C implementation.
    fn c_find_roots(m: &Map) -> Result<Vec<i32>, i32> {
        let mut buckets: *mut i32 = std::ptr::null_mut();
        let n = unsafe {
            crush_sys::crush_find_roots(m.ptr as *mut crush_sys::crush_map, &mut buckets)
        };
        if n < 0 {
            return Err(n);
        }
        let roots = unsafe { slice::from_raw_parts(buckets, n as usize) }.to_vec();
        unsafe { libc::free(buckets as *mut libc::c_void) };
        Ok(roots)
    }

    fn check(m: &Map) -> Result<Vec<i32>, CrushError> {
        let safe = crush_find_roots_safe(unsafe { &*m.ptr });
        let oracle = unsafe { crush_find_roots_unsafe_oracle(&*m.ptr) };
        assert_eq!(safe, oracle);
        assert_eq!(safe.clone().map_err(|e| e.to_errno()), c_find_roots(m));
        safe
    }

    fn add_bucket(m: &mut Map) -> i32 {
        let b = m
            .make_bucket(BucketAlgorithm::Straw2, 0, 1, &[], &[])
            .unwrap();
        m.add_bucket(0, b).unwrap()
    }

    #[test]
    fn matches_oracle_and_c() {
        let mut m = Map::new();
        assert_eq!(check(&m), Ok(vec![]));

        let first = add_bucket(&mut m);
        let second = add_bucket(&mut m);
        let third = add_bucket(&mut m);
        assert_eq!(check(&m), Ok(vec![first, second, third]));

        // Nest `second` under `first`, then `third` under `second`.
        unsafe {
            let b = *(*m.ptr).buckets.offset(-1 - first as isize);
            crush_bucket_add_item(m.ptr, b, second, 0x1000);
        }
        assert_eq!(check(&m), Ok(vec![first, third]));
        unsafe {
            let b = *(*m.ptr).buckets.offset(-1 - second as isize);
            crush_bucket_add_item(m.ptr, b, third, 0x1000);
        }
        assert_eq!(check(&m), Ok(vec![first]));

        // An item referencing a bucket slot past max_buckets is invalid.
        unsafe {
            let b = *(*m.ptr).buckets.offset(-1 - first as isize);
            crush_bucket_add_item(m.ptr, b, -200, 0x1000);
        }
        assert_eq!(check(&m), Err(CrushError::InvalidArgument));
    }
}
