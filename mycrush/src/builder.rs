use crate::{CrushMap, CrushBucket, CrushRule}; // Removed CrushBucketCommon

/// Creates a new CrushMap instance with default/empty values and sensible tunables.
pub fn create_crush_map() -> CrushMap {
    CrushMap {
        buckets: Vec::new(),
        rules: Vec::new(),
        max_buckets: 0, // Will be updated as buckets are added
        max_rules: 0,   // Will be updated as rules are added
        max_devices: 0, // Placeholder, to be updated by other means (e.g. analyzing buckets)

        // Default tunables (based on common Ceph settings or sensible starting points)
        // These are from 'set_optimal_tunables' or common defaults.
        // Actual optimal values depend on the Ceph version and cluster specifics.
        choose_local_tries: 2,
        choose_local_fallback_tries: 5,
        choose_total_tries: 50, // A common default in newer Ceph versions
        chooseleaf_descend_once: 1, // true
        chooseleaf_vary_r: 1,       // true
        chooseleaf_stable: 1,       // true
        straw_calc_version: 1,      // Use new straw calculation
        allowed_bucket_algs: (1 << crate::CrushAlgorithm::Uniform as u8) |
                               (1 << crate::CrushAlgorithm::List as u8) |
                               (1 << crate::CrushAlgorithm::Tree as u8) | // Though we haven't defined Tree bucket struct
                               (1 << crate::CrushAlgorithm::Straw as u8) | // Though we haven't defined Straw bucket struct
                               (1 << crate::CrushAlgorithm::Straw2 as u8),
        // choose_tries: Vec::new(), // This was a pointer, might not be needed directly in Rust map struct
    }
}

/// Adds a bucket to the CrushMap.
/// Assigns a new unique negative ID to the bucket and updates its common part.
pub fn add_bucket(map: &mut CrushMap, mut bucket_data: CrushBucket) -> Result<i32, &'static str> {
    // In Ceph, bucket IDs are negative, starting from -1.
    // The ID is often used as -1 - index.
    let new_id = -1 - (map.buckets.len() as i32);

    // Ensure the bucket_data itself has its ID set correctly.
    bucket_data.set_id(new_id);

    // Update max_devices if items in this bucket are devices (positive IDs)
    // This is a simple scan; a more robust way might be needed.
    for &item_id in &bucket_data.common().items {
        if item_id >= 0 && item_id + 1 > map.max_devices {
            map.max_devices = item_id + 1;
        }
    }

    map.buckets.push(Some(bucket_data));
    map.max_buckets = map.buckets.len() as i32; // max_buckets in C is count of bucket slots

    Ok(new_id)
}

/// Adds a rule to the CrushMap.
/// If rule_id_req is None, finds the next available slot.
/// If rule_id_req is Some, uses that slot if available, or extends the rule Vec.
pub fn add_rule(map: &mut CrushMap, rule_data: CrushRule, rule_id_req: Option<usize>) -> Result<usize, &'static str> {
    let rule_id_to_assign: usize;

    match rule_id_req {
        Some(req_id) => {
            rule_id_to_assign = req_id;
            if req_id >= map.rules.len() {
                map.rules.resize_with(req_id + 1, || None);
            } else if map.rules[req_id].is_some() {
                // For simplicity, error out if rule slot is taken.
                // C crush_add_rule might overwrite or have different logic.
                // return Err("Requested rule ID slot is already taken.");
                // Or, allow overwrite:
                println!("Warning: Overwriting existing rule at ID {}", req_id);
            }
        }
        None => {
            // Find next available None slot or append
            if let Some(idx) = map.rules.iter().position(|r| r.is_none()) {
                rule_id_to_assign = idx;
            } else {
                rule_id_to_assign = map.rules.len();
                map.rules.push(None); // Placeholder, will be replaced
            }
        }
    }

    map.rules[rule_id_to_assign] = Some(rule_data);

    if (rule_id_to_assign + 1) as u32 > map.max_rules {
        map.max_rules = (rule_id_to_assign + 1) as u32;
    }

    Ok(rule_id_to_assign)
}
