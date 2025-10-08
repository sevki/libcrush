#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::crush::types::*;

/// Find root buckets in the CRUSH map (buckets that are not children of any other bucket)
/// Returns a Vec of root bucket IDs (as negative indices)
pub fn crush_find_roots_safe(map: &CrushMap) -> Result<Vec<i32>, CrushError> {
    let max_buckets = map.max_buckets as usize;
    let mut ref_counts: Vec<i32> = vec![0; max_buckets];
    let mut root_count = map.max_buckets;
    
    // Count references to each bucket
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
    unsafe {
        for pos in 0..map.max_buckets {
            if !(*(map.buckets).offset(pos as isize)).is_null()
                && ref_counts[pos as usize] == 0
            {
                roots.push(-1 - pos);
            }
        }
    }
    
    assert_eq!(roots.len(), root_count as usize, 
        "Root count mismatch: found {} roots, expected {}", roots.len(), root_count);

    Ok(roots)
}
