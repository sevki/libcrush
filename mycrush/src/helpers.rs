use crate::CrushMap;
use std::collections::HashSet;

/// Finds all root bucket IDs in the CrushMap.
/// A root bucket is a bucket that is not an item in any other bucket.
pub fn find_roots(map: &CrushMap) -> Result<Vec<i32>, String> {
    let mut all_bucket_ids: HashSet<i32> = HashSet::new();
    let mut child_bucket_ids: HashSet<i32> = HashSet::new();

    // Populate all_bucket_ids with IDs of all valid buckets
    for bucket in map.buckets.iter().flatten() {
        all_bucket_ids.insert(bucket.common().id);
    }

    // Populate child_bucket_ids by iterating through items of each bucket
    for bucket in map.buckets.iter().flatten() {
        let common_part = bucket.common();
        for item_id in &common_part.items {
            if *item_id < 0 {
                // Indicates a bucket ID
                // It's a child, but also check if this child ID actually exists in all_bucket_ids.
                // This is a basic integrity check. Original crush_find_roots might have more.
                if !all_bucket_ids.contains(item_id) {
                    // This situation (a bucket referencing a non-existent bucket as a child)
                    // might indicate a map corruption or an issue during map building.
                    // The C version might handle this by not adding it or erroring out earlier.
                    // For find_roots, we can note it or error.
                    // Let's error for now if a child points to a non-defined bucket.
                    // However, find_roots usually just identifies roots from what's defined.
                    // A bucket can be a child and not be in map.buckets if it's a "ghost" or error.
                    // For now, let's just record all negative items as potential children.
                    // The definition of a root is something that IS a bucket and IS NOT a child.
                    // So, if item_id is not in all_bucket_ids, it cannot be a root anyway.
                }
                child_bucket_ids.insert(*item_id);
            }
        }
    }

    let mut roots_vec: Vec<i32> = all_bucket_ids
        .difference(&child_bucket_ids)
        .copied()
        .collect();

    // Sort for consistent output, though not strictly required by logic
    roots_vec.sort_unstable();

    Ok(roots_vec)
}
