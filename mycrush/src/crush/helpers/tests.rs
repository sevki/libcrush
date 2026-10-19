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
            if !(*(map.buckets).offset(pos as isize)).is_null() && ref_counts[pos as usize] == 0 {
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
    let n =
        unsafe { crush_sys::crush_find_roots(m.ptr as *mut crush_sys::crush_map, &mut buckets) };
    if n < 0 {
        return Err(n);
    }
    let roots = unsafe { slice::from_raw_parts(buckets, n as usize) }.to_vec();
    unsafe { libc::free(buckets as *mut libc::c_void) };
    Ok(roots)
}

fn check(m: &Map) -> Result<Vec<i32>, CrushError> {
    if m.ptr.is_null() {
        return Err(CrushError::InvalidArgument);
    }
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
        assert!(!m.ptr.is_null(), "Map pointer must not be null");
        assert!(!(*m.ptr).buckets.is_null(), "Map buckets pointer must not be null");
        let b = *(*m.ptr).buckets.offset(-1 - first as isize);
        assert!(!b.is_null(), "Selected bucket pointer must not be null");
        crush_bucket_add_item(m.ptr, b, second, 0x1000);
    }
    assert_eq!(check(&m), Ok(vec![first, third]));
    unsafe {
        assert!(!m.ptr.is_null(), "Map pointer must not be null");
        assert!(!(*m.ptr).buckets.is_null(), "Map buckets pointer must not be null");
        let b = *(*m.ptr).buckets.offset(-1 - second as isize);
        assert!(!b.is_null(), "Selected bucket pointer must not be null");
        crush_bucket_add_item(m.ptr, b, third, 0x1000);
    }
    assert_eq!(check(&m), Ok(vec![first]));

    // An item referencing a bucket slot past max_buckets is invalid.
    unsafe {
        assert!(!m.ptr.is_null(), "Map pointer must not be null");
        assert!(!(*m.ptr).buckets.is_null(), "Map buckets pointer must not be null");
        let b = *(*m.ptr).buckets.offset(-1 - first as isize);
        assert!(!b.is_null(), "Selected bucket pointer must not be null");
        crush_bucket_add_item(m.ptr, b, -200, 0x1000);
    }
    assert_eq!(check(&m), Err(CrushError::InvalidArgument));
}
