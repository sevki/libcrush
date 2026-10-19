use crate::crush::types::*;
use std::ptr::NonNull;
use std::slice;

/// Views a C array as a slice, treating a null pointer as empty.
///
/// # Safety
/// A non-null `ptr` must point to `len` initialized, live elements.
pub(crate) unsafe fn c_slice<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    match NonNull::new(ptr.cast_mut()) {
        // Safety: guaranteed by the caller.
        Some(ptr) => unsafe { slice::from_raw_parts(ptr.as_ptr(), len) },
        None => &[],
    }
}

/// Find root buckets in the CRUSH map (buckets that are not children of any other bucket)
/// Returns a Vec of root bucket IDs (as negative indices)
pub fn crush_find_roots_safe(map: &CrushMap) -> Result<Vec<i32>, CrushError> {
    let max_buckets = usize::try_from(map.max_buckets).unwrap_or(0);
    // Safety: a CrushMap's `buckets` array holds `max_buckets` bucket pointers,
    // each null or pointing to a live bucket whose `items` holds `size` ids.
    let buckets = unsafe { c_slice(map.buckets, max_buckets) };
    let buckets: Vec<Option<&CrushBucket>> = buckets
        .iter()
        .map(|&b| NonNull::new(b).map(|b| unsafe { b.as_ref() }))
        .collect();

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
mod tests;
