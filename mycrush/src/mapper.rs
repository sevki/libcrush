use crate::{
    CrushMap, CrushOpcode, CrushBucket, // Removed CrushRule, CrushRuleStep
    CrushBucketUniform, CrushBucketList, CrushBucketStraw2,
    hash::crush_hash32_rjenkins1,
};
use std::collections::HashSet;
// use crate::CrushOpcode; // This was a duplicate import, CrushOpcode is already imported above.
use num_traits::FromPrimitive; // Keep this as it's the standard way for the trait to be in scope.
                               // The compiler error for E0599 (no from_u32 on CrushOpcode) suggested a path like
                               // use crate::_IMPL_NUM_FromPrimitive_FOR_CrushOpcode::_num_traits::FromPrimitive;
                               // This implies the derive macro is working but the method isn't found without specific help.
                               // However, a direct call `CrushOpcode::from_u32` should work if derived correctly
                               // and the `FromPrimitive` trait itself is in scope for the method call.
                               // Let's ensure `FromPrimitive` is broadly available.

// Helper function to get bucket by ID from the map
// This function would be more robust in a real scenario, handling missing buckets etc.
fn get_bucket_by_id(map: &CrushMap, id: i32) -> Option<&CrushBucket> {
    // In Ceph, bucket IDs are negative and map to positive indices.
    // e.g., id -1 -> buckets[0], id -2 -> buckets[1]
    if id >= 0 { return None; } // Not a bucket ID
    let idx = (-id - 1) as usize;
    if idx < map.buckets.len() {
        map.buckets[idx].as_ref()
    } else {
        None
    }
}


fn crush_bucket_choose_uniform(
    bucket: &CrushBucketUniform,
    x: i32,
    r: i32
) -> Option<i32> {
    if bucket.common.items.is_empty() || bucket.common.size == 0 {
        return None;
    }
    // Use the globally defined hash function for this crate
    let h = crush_hash32_rjenkins1(x as u32, bucket.common.id as u32, r as u32);
    let item_index = (h % bucket.common.size as u32) as usize;

    bucket.common.items.get(item_index).copied()
}

fn crush_bucket_choose_list(
    bucket: &CrushBucketList,
    x: i32,
    r: i32
) -> Option<i32> {
    if bucket.common.items.is_empty() || bucket.common.size == 0 {
        return None;
    }

    let mut current_r = r;
    // The logic for list buckets can be complex, involving retries and adjustments to 'r'.
    // This is a simplified initial version.
    // The original C code might iterate multiple times or adjust 'r' based on previous failures.

    for i in 0..bucket.common.size as usize {
        if i >= bucket.item_weights.len() || i >= bucket.sum_weights.len() || i >= bucket.common.items.len() {
            // Data inconsistency
            return None;
        }

        let item_id = bucket.common.items[i];
        let item_weight = bucket.item_weights[i];

        // Simplified sum_weight logic: sum of weights of items *after* current one.
        // The sum_weights array in Ceph's crush_bucket_list is typically:
        // sum_weights[i] = sum of item_weights[0]...item_weights[i]
        // So, weight of items after i would be total_weight - sum_weights[i]
        // And weight of items before or at i is sum_weights[i]
        // For this version, let's assume sum_weights[j] is sum of weights from 0 to j.
        // Total weight is sum_weights[size-1].
        // Sum of weights of items *after* current item `i`:
        // `total_sum = sum_weights[bucket.common.size as usize -1]`
        // `current_item_and_previous_sum = sum_weights[i]`
        // `sum_after = total_sum - current_item_and_previous_sum` (This is not quite right)

        // Let's use the logic described in the prompt:
        // s = sum of weights of *remaining* items (items from i+1 to end)
        // This requires a bit of careful handling of sum_weights.
        // If sum_weights[k] = sum(w_0 .. w_k), then sum(w_{i+1} .. w_{size-1}) = sum_weights[size-1] - sum_weights[i]

        let total_weight_for_bucket = if !bucket.sum_weights.is_empty() {
            bucket.sum_weights[bucket.sum_weights.len()-1]
        } else {
            0
        };

        let sum_up_to_and_including_current = bucket.sum_weights[i];
        // The sum of weights for items *strictly after* the current item `i`.
        // This seems to be what `s` should represent in the formula `(h & 0xffff) * w > (s & 0xffff) * 0x10000`
        // where `w` is current item's weight and `s` is for "other" items.
        // However, the Ceph paper describes choosing item `j` if hash is in `(sum_weights[j-1], sum_weights[j]]`.
        // The provided formula `(h & 0xffff) * w > (s & 0xffff) * 0x10000` seems more like a straw-like comparison.

        // Let's try to match the formula:
        // w = weight of current item
        // s = sum of weights of items *before* the current one in this iteration (or some other interpretation of 'remaining')
        // This is a common probabilistic selection: pick item `i` with probability `w_i / sum_total_weights`.
        // The C code's `crush_bucket_list_choose` iterates. If an item is out, it subtracts its weight from `s`
        // and continues. `s` starts as the sum of weights of all items *not yet rejected*.

        // Simplified approach for now, closer to a direct probabilistic pick from remaining:
        // For each item, calculate a score and pick the one that passes a threshold.
        // This isn't how list usually works (it's typically iterative selection based on hash falling into a weight range).
        // The prompt's formula: `(h & 0xffff) * w > (s & 0xffff) * 0x10000`
        // Let `s` be the sum of weights of items *not including the current one*.
        // This is not standard list logic. Standard list logic is:
        // `r' = hash(x, r) % total_weight`. Find item `j` such that `sum_weight[j-1] < r' <= sum_weight[j]`.

        // Given the prompt's specific formula, let's try to interpret `s`.
        // If `s` is sum of *other* items:
        // `s = total_weight_for_bucket - item_weight`. If `item_weight` is 0, this also needs care.

        let s = if total_weight_for_bucket > item_weight { // sum of other weights
            total_weight_for_bucket - item_weight
        } else { // current item is the only one with weight or all weights are zero
            0
        };

        if s == 0 { // If current item is the only one with weight, or all weights are zero.
                    // If item_weight > 0, it should be chosen. If all are zero, first one?
            if item_weight > 0 { return Some(item_id); }
            // If item_weight is also 0, and s is 0, this means all weights are 0.
            // The original CRUSH would likely pick the first item in such a degenerate case if r=0.
            // For simplicity here, if all other weights are 0, and current is 0, we might not pick.
            // Or, if it's the last item and s=0, it must be picked.
            if i == (bucket.common.size as usize) -1 { return Some(item_id); }
            continue; // try next if not last and current weight is 0
        }

        let h = crush_hash32_rjenkins1(x as u32, item_id as u32, current_r as u32);
        // The comparison `(h & 0xffff) * item_weight > (s & 0xffff) * 0x10000` is problematic.
        // `s & 0xffff` truncates `s`. `0x10000` is `1<<16`.
        // This is `(h_low_16_bits / 2^16) * item_weight > (s_low_16_bits / 2^16) * 2^16`
        // Simplified: `h_low_16_bits * item_weight > s_low_16_bits * 0x10000`
        // This is equivalent to `h_low_16_bits * item_weight / 0x10000 > s_low_16_bits`
        // Or `(h & 0xffff) * item_weight > s * 0x10000` if s is already scaled or if `s & 0xffff` was a typo for `s`.
        // Assuming item_weight and s are 16.16 fixed point, then `w = item_weight >> 16` and `s_val = s >> 16`.
        // The C code uses `(crush_hash32_rjenkins1(x, items[i], r) & 0xffff) * w > s * 0x10000`
        // where `w` is `item_weight` and `s` is `sum_of_weights_of_items_not_yet_tried_and_failed`.
        // Let's assume weights are u32 representing fixed point 16.16.

        // Simplified: if ((h & 0xffff) as u64 * item_weight as u64) > ((s & 0xffff) as u64 * 0x10000u64) {
        // Using a more standard interpretation for list buckets:
        // Iterate, on each failure, subtract weight from total and retry for next replica (r++).
        // This is for one selection (one r value).
        // For a single r, the standard way:
        // `target_hash_val = crush_hash32_rjenkins1(x, bucket_id, r) % total_sum_of_weights`
        // Then find item `j` such that `sum_weights[j-1] < target_hash_val <= sum_weights[j]`
        // (using sum_weights[k] as sum up to k).

        // Re-interpreting the prompt's formula for list bucket:
        // This formula is more akin to how straw competes. Let's assume it's a probabilistic step.
        // The original Ceph code's list bucket logic is:
        // `r'" = crush_hash32_rjenkins1(x, b->h.id, r);`
        // `first_try = true;`
        // `do { ... `
        // `  s = 0;`
        // `  for (item = 0...size-1) { `
        // `    if (skip[item]) continue;`
        // `    s += item_weight;`
        // `    if (s > (r'" & 0xffff) * W / 0x10000) { chosen = item; break; } `
        // `  }`
        // `  if chosen is out { skip[chosen]=true; r'" = crush_hash32_rjenkins1(x, b->h.id, ++r); first_try=false; } `
        // `} while (chosen is out)`
        // This is too complex for this stage.
        // Let's stick to the simple formula from the prompt, understanding it might be inaccurate for true list behavior.
        // `h = hash_fn(x as u32, item_id as u32, r as u32)`
        // `if (h & 0xffff) * item_weight > (s & 0xffff) * 0x10000`
        // This condition means: (h_low_bits / 2^16) * item_weight > (s_low_bits / 2^16) * 2^16
        // Or: (h_low_bits * item_weight) > (s_low_bits * 2^16)
        // Let's use u64 to avoid overflow during multiplication.
        if item_weight == 0 { continue; } // Cannot be chosen if weight is 0 with this formula
        let h_val = crush_hash32_rjenkins1(x as u32, item_id as u32, current_r as u32) & 0xffff;
        if (h_val as u64 * item_weight as u64) > ((s & 0xffff) as u64 * 0x10000u64) {
             return Some(item_id);
        }
        // This simplified version will tend to pick items at the beginning of the list
        // more often if their weights are high, which is not quite right for list.
        // A true list bucket gives precedence to items based on hash falling in their segment of total weight.
        // For now, we proceed with this interpretation.
    }
    // If loop finishes, means no item was chosen based on the formula.
    // Fallback: could pick last valid item or none. Original list would always pick one if total weight > 0.
    // If any item has weight, one should be chosen.
    // If all weights are 0, original CRUSH might pick items[r % size].
    // Fallback if the loop fails to select (e.g. due to simplified logic or all-zero weights with the formula).
    // Only attempt fallback if there are items.
    if !bucket.common.items.is_empty() {
        // The original condition `total_weight_for_bucket > 0` is problematic due to scope.
        // The fallback itself doesn't use total_weight_for_bucket.
        // This fallback ensures *something* is picked if items exist, which is closer to CRUSH behavior
        // than returning None when a bucket has items.
        // println!("crush_bucket_choose_list: Loop failed, attempting fallback selection for bucket {}", bucket.common.id);
        let fallback_idx = (crush_hash32_rjenkins1(x as u32, bucket.common.id as u32, current_r as u32) % bucket.common.size as u32) as usize;
        return bucket.common.items.get(fallback_idx).copied();
    }

    None // Should ideally always pick one if bucket not empty and total weight > 0
}


fn crush_bucket_choose_straw2(
    bucket: &CrushBucketStraw2,
    x: i32,
    r: i32
) -> Option<i32> {
    if bucket.common.items.is_empty() || bucket.common.size == 0 {
        return None;
    }

    let mut max_straw_val: i64 = -1; // Use i64 because straws can be large, init with a value lower than any possible straw
    let mut selected_item: Option<i32> = None;

    for i in 0..bucket.common.size as usize {
        if i >= bucket.item_weights.len() || i >= bucket.common.items.len() {
            return None; // Data inconsistency
        }
        let item_id = bucket.common.items[i];
        let item_weight = bucket.item_weights[i];

        if item_weight == 0 { // Some versions of straw might skip 0-weight items
            // continue;
        }

        // Simplified straw calculation. Ceph has straw_calc_version for different calculations.
        // Basic idea: hash draws a "length" for the straw, then scale by weight.
        // H = hash(x, item_id, r)
        // straw = f(H) * weight_factor(item_weight)
        // The reference C code uses:
        //   unsigned long long h = crush_hash32_rjenkins1(x, items[i], r);
        //   h &= 0xffff;
        //   h *= weight;  (where weight is u32 for item_weight)
        // So, straw is (hash_low_16_bits * item_weight)

        let h = crush_hash32_rjenkins1(x as u32, item_id as u32, r as u32);
        let h_low_16bits = h & 0xffff;

        // Weight factor for straw2. Ceph uses:
        // static inline __u32 crush_straw_weight_factor(__u32 w)
        // {
        //   return (unsigned)(ln((double)w / 0x10000.0) * 0x10000.0);
        // }
        // This is complex. Let's use the simpler (h_low_16bits * item_weight) for now,
        // which is actually from the original "straw" algorithm, not "straw2".
        // Straw2 uses `ln(weight/max_weight) * max_weight_ln_units`.
        // The prompt says: `straw = (h & 0xffff) * bucket.item_weights[i]`
        let current_straw_val = h_low_16bits as u64 * item_weight as u64;

        if selected_item.is_none() || (current_straw_val as i64) > max_straw_val {
            max_straw_val = current_straw_val as i64;
            selected_item = Some(item_id);
        }
    }
    selected_item
}

fn dispatch_bucket_choose(
    bucket: &CrushBucket,
    x: i32,
    r: i32,
    // hash_fn: fn(u32, u32, u32) -> u32, // Using crate's hash function directly for now
) -> Option<i32> {
    match bucket {
        CrushBucket::Uniform(b) => crush_bucket_choose_uniform(b, x, r),
        CrushBucket::List(b) => crush_bucket_choose_list(b, x, r),
        CrushBucket::Straw2(b) => crush_bucket_choose_straw2(b, x, r),
        // CrushBucket::Tree(_) => ..., // Example if other types were added
        // CrushBucket::Straw(_) => ...,
    }
}

// Simplified recursive helper for ChooseLeaf operations
fn crush_select_leaf(
    map: &CrushMap,
    item_id: i32,      // The item currently being considered (could be bucket or device)
    x: i32,            // The input value for hashing
    r_attempt: i32,    // Current attempt/replica number for selection from this path
    local_reject_list: &HashSet<i32>, // Items already selected or rejected
    target_bucket_type_filter: i32, // Type of bucket to allow descent into (0 means no filter)
) -> Option<i32> {
    if item_id >= 0 { // Item is a device
        if !local_reject_list.contains(&item_id) {
            return Some(item_id);
        } else {
            // println!("crush_select_leaf: Device {} is in reject list.", item_id);
            return None; // Collision with already selected item
        }
    }

    // Item is a bucket
    let current_bucket_id = item_id;
    // println!("crush_select_leaf: Descending into bucket {}. Target type filter: {}", current_bucket_id, target_bucket_type_filter);

    if let Some(bucket_obj) = get_bucket_by_id(map, current_bucket_id) {
        // Apply bucket type filter for descent
        if target_bucket_type_filter != 0 && bucket_obj.common().r#type as i32 != target_bucket_type_filter {
            // println!("crush_select_leaf: Bucket {} type {} does not match target type filter {}. Stopping descent.",
            //          current_bucket_id, bucket_obj.common().r#type, target_bucket_type_filter);
            return None;
        }

        // Try to select an item from this bucket.
        // The 'r_attempt' here is crucial. For true CRUSH behavior, how 'r' (replica index)
        // and internal retries are managed is complex. This is a simplification.
        // We might need separate counters for "outer r" (which replica we are trying to find for the rule)
        // and "inner r" (attempts within a bucket or recursive descent).
        // The r_attempt is incremented for each *level* of recursion here, which is one way to vary selection.
        if let Some(chosen_from_sub_bucket) = dispatch_bucket_choose(bucket_obj, x, r_attempt) {
            // Recursively try to find a leaf from the item chosen from the sub-bucket.
            // Pass the original target_bucket_type_filter down.
            // Increment r_attempt for the next level of recursion to ensure selection varies if this path is tried again due to outer retries.
            return crush_select_leaf(map, chosen_from_sub_bucket, x, r_attempt + 1, local_reject_list, target_bucket_type_filter);
        } else {
            // println!("crush_select_leaf: dispatch_bucket_choose returned None from bucket {}", current_bucket_id);
            return None;
        }
    } else {
        // println!("crush_select_leaf: Bucket {} (id {}) not found in map.", current_bucket_id, item_id);
        return None;
    }
}


pub fn crush_do_rule(
    map: &CrushMap,
    rule_no: i32,
    _x: i32, // x is not used yet, but part of the signature
    result: &mut Vec<i32>,
    result_max: usize,
    // Omit weights, weight_max, cwin, choose_args for now
) -> usize {
    let original_result_len = result.len();

    if rule_no < 0 || rule_no as usize >= map.rules.len() {
        eprintln!("Rule {} not found or invalid (max rules {})", rule_no, map.rules.len());
        return 0;
    }

    let rule = match map.rules.get(rule_no as usize) {
        Some(Some(r)) => r,
        _ => {
            eprintln!("Rule {} not found or invalid (is None).", rule_no);
            return 0;
        }
    };

    if rule.steps.is_empty() && rule.len > 0 {
        eprintln!("Rule {} has len {} but no steps defined.", rule_no, rule.len);
        return 0;
    }
     if rule.len == 0 && !rule.steps.is_empty() {
        eprintln!("Rule {} has len 0 but steps are defined.", rule_no);
        // This might be acceptable if len is dynamically determined, but typically len matches steps.len().
    }


    let mut current_selection: Vec<i32> = Vec::new();
    // let mut items_added = 0; // Replaced by (result.len() - original_result_len)

    // Ensure rule.len is consistent with rule.steps.len()
    // The C struct has `steps[0]` which is a flexible array member, len is the count.
    // In Rust, Vec<CrushRuleStep> has its own length. We should use steps.len().
    let num_steps = rule.steps.len();
    if rule.len as usize != num_steps {
        // This could be a warning or an error depending on how strictly we interpret `rule.len`
        // For now, let's prefer `rule.steps.len()` as the source of truth for iteration.
        println!(
            "Warning: Rule {} has len field {} but actual steps count is {}. Using actual count.",
            rule_no, rule.len, num_steps
        );
    }


    for step_idx in 0..num_steps {
        // No need to check step_idx against rule.steps.len() due to loop bounds.
        let step = &rule.steps[step_idx];

        // Check if we can add more items *before* processing most steps.
        // Emit is special as it might just clear current_selection even if result is full.
        if result.len() >= result_max && step.op != CrushOpcode::Emit as u32 {
            println!("Result vector full (current: {}, max: {}). Stopping rule processing before step {}.", result.len(), result_max, step_idx);
            break;
        }

        match CrushOpcode::from_u32(step.op) { // Changed to call from_u32 directly on the enum
            Some(CrushOpcode::Take) => {
                println!("Executing Take: arg1 = {}", step.arg1);
                current_selection.clear();
                current_selection.push(step.arg1);
            }
            Some(CrushOpcode::Emit) => {
                println!("Executing Emit. Current selection: {:?}, result space: {}/{}", current_selection, result.len(), result_max);
                for &item in &current_selection {
                    if result.len() < result_max {
                        result.push(item);
                        // items_added += 1; // Will be calculated at the end
                    } else {
                        println!("Result vector full during Emit. Cannot add item: {}. Current selection will be cleared.", item);
                        break;
                    }
                }
                current_selection.clear();
                if result.len() >= result_max {
                    println!("Result vector full after Emit. Stopping rule processing.");
                    break;
                }
            }
            Some(op @ CrushOpcode::ChooseFirstN) |
            Some(op @ CrushOpcode::ChooseLeafFirstN) => {
                // This block is for ChooseFirstN and ChooseLeafFirstN
                // It was implemented in a previous subtask. Assuming its current state from the file read is acceptable
                // or will be refined separately. The current file content has a simplified placeholder here.
                // For this subtask, we are focusing on INDEP.
                // To make this runnable, I'll copy the simplified placeholder from the read file for this section.
                 let op_name = match op {
                    CrushOpcode::ChooseFirstN => "ChooseFirstN",
                    CrushOpcode::ChooseLeafFirstN => "ChooseLeafFirstN",
                    _ => "Unknown FirstN Op",
                };
                let requested_count = step.arg1;
                let choose_type = step.arg2;
                println!(
                    "Executing (simplified) {} for input items: {:?}, target_type: {}, requested_count: {}",
                    op_name, current_selection, choose_type, requested_count
                );
                let mut new_selection: Vec<i32> = Vec::new();
                if let Some(input_bucket_id) = current_selection.first() {
                    if let Some(bucket_obj) = get_bucket_by_id(map, *input_bucket_id) {
                        // Simplified: use r=0, and if leaf, try to get leaf from chosen item.
                        let r_val = 0;
                        let initial_chosen = dispatch_bucket_choose(bucket_obj, _x, r_val);

                        if let Some(init_item) = initial_chosen {
                            if op == CrushOpcode::ChooseLeafFirstN {
                                let mut reject_list_for_leaf: HashSet<i32> = result.iter().copied().collect();
                                if let Some(leaf_item) = crush_select_leaf(map, init_item, _x, r_val, &reject_list_for_leaf, choose_type) {
                                     if !reject_list_for_leaf.contains(&leaf_item) { // ensure not already in result
                                        new_selection.push(leaf_item);
                                     }
                                }
                            } else { // ChooseFirstN
                                if !result.contains(&init_item) { // basic check against current result
                                    new_selection.push(init_item);
                                }
                            }
                        }
                    }
                }
                current_selection = new_selection;
                println!("Simplified {} selection: {:?}", op_name, current_selection);
            }
            Some(op @ CrushOpcode::ChooseIndep) |
            Some(op @ CrushOpcode::ChooseLeafIndep) => {
                let op_name = match op {
                    CrushOpcode::ChooseIndep => "ChooseIndep",
                    CrushOpcode::ChooseLeafIndep => "ChooseLeafIndep",
                    _ => "Unknown Indep Op", // Should not happen
                };

                let num_items_to_select_for_rule = if step.arg1 == 0 { // Number of distinct items this step should output
                     result_max.saturating_sub(result.len())
                } else {
                    (step.arg1 as usize).min(result_max.saturating_sub(result.len()))
                };
                // For ChooseLeafIndep, target_type filters intermediate buckets.
                // For ChooseIndep, target_type filters the bucket chosen from current_selection.
                let target_type = step.arg2;

                println!(
                    "Executing {}: num_to_select_for_rule={}, target_type={}, current_selection_size={}",
                    op_name, num_items_to_select_for_rule, target_type, current_selection.len()
                );

                let mut next_selection: Vec<i32> = Vec::new();
                // In CHOOSEINDEP, the local_reject_list prevents selecting the same item multiple times *for different output slots*.
                let mut local_reject_list: HashSet<i32> = result.iter().copied().collect();

                if current_selection.is_empty() && (op == CrushOpcode::ChooseIndep || (op == CrushOpcode::ChooseLeafIndep && target_type == 0 /* e.g. no specific root type given */)) {
                    // CHOOSELEAFINDEP can start from map root (if current_selection is empty and rule implies it by e.g. target_type being a root type)
                    // CHOOSEINDEP usually requires some input buckets from current_selection.
                    println!("Warning: {} called with empty current_selection. Specific behavior for this case (e.g. starting from map root) is not fully implemented. Selection may be empty.", op_name);
                }


                for i in 0..num_items_to_select_for_rule { // For each output slot
                    if result.len() + next_selection.len() >= result_max { break; }

                    let r_for_this_item_slot = i as i32; // Base 'r' for this specific item/slot.
                    let mut item_found_for_this_slot = false;

                    // Determine retry attempts for this specific slot
                    let retries_for_slot = map.choose_local_tries.saturating_add(map.choose_local_fallback_tries).max(1);


                    for try_num in 0..retries_for_slot { // Retries for the current slot `i`
                        // Iterate through each input_item_id in the current_selection
                        // (these are the buckets/items passed from the previous step)
                        // If current_selection is empty, this inner loop is skipped.
                        // This implicitly handles "start from root" if current_selection is empty and op is ChooseLeafIndep,
                        // then initial_chosen_item logic should handle picking a root of target_type. (This part is still simplified)

                        let inputs_to_iterate = if current_selection.is_empty() && op == CrushOpcode::ChooseLeafIndep {
                            // If current_selection is empty for ChooseLeafIndep, we might need to select roots of target_type.
                            // This is a complex part of CRUSH. For now, let's assume if current_selection is empty,
                            // we can't proceed easily unless we add logic to pick initial root buckets.
                            // For simplification, we'll print a warning and likely select nothing.
                            // A real implementation would iterate over map roots of 'target_type'.
                            // vec![] // Effectively skips the loop below if current_selection is empty.
                             if target_type != 0 {
                                // Placeholder: try to find a root bucket of target_type.
                                // This is a very simplified way to get a starting point.
                                // A real map would have actual root ids.
                                map.buckets.iter().enumerate()
                                    .find(|(_idx, b_opt)| b_opt.as_ref().map_or(false, |b| b.common().r#type as i32 == target_type))
                                    .map_or(vec![], |(idx, _)| vec![-1 - idx as i32]) // Get its ID
                            } else {
                                vec![] // No target type, cannot pick a root.
                            }
                        } else {
                            current_selection.clone() // Iterate over provided inputs
                        };

                        if inputs_to_iterate.is_empty() && current_selection.is_empty() {
                             println!("{}: No input items in current_selection and no root found/specified for ChooseLeafIndep.", op_name);
                             break; // Break try_num loop, nothing to select from for this slot.
                        }


                        for &input_item_id in &inputs_to_iterate {
                            // The 'r' for dispatch_bucket_choose is varied by try_num for each slot.
                            // r_for_this_item_slot provides uniqueness per slot, try_num provides retries.
                            // Multiplying try_num by a larger factor can help spread out attempts more.
                            // All parts of the calculation for current_r must be i32.
                            // try_num is u32. num_items_to_select_for_rule is usize.
                            let factor = try_num as i32 * (num_items_to_select_for_rule.max(1) as i32);
                            let current_r = r_for_this_item_slot + factor;

                            let initial_chosen_item = if op == CrushOpcode::ChooseLeafIndep {
                                // For ChooseLeafIndep, input_item_id is the starting point for descent.
                                input_item_id
                            } else { // ChooseIndep
                                let bucket_obj = match get_bucket_by_id(map, input_item_id) {
                                    Some(b) => b,
                                    None => continue, // Input item not a valid bucket
                                };
                                // Filter by bucket type if target_type is specified (for the bucket chosen *from*)
                                if target_type != 0 && bucket_obj.common().r#type as i32 != target_type {
                                    continue;
                                }
                                match dispatch_bucket_choose(bucket_obj, _x, current_r) {
                                    Some(ci) => ci,
                                    None => continue, // dispatch_bucket_choose failed for this r
                                }
                            };

                            let final_item_to_add = if op == CrushOpcode::ChooseLeafIndep {
                                // target_type for ChooseLeafIndep is the type of *intermediate* bucket to descend into.
                                crush_select_leaf(map, initial_chosen_item, _x, current_r, &local_reject_list, target_type)
                            } else { // ChooseIndep
                                // For ChooseIndep, the item from dispatch_bucket_choose is the candidate.
                                // It must not be in reject list.
                                if !local_reject_list.contains(&initial_chosen_item) {
                                    Some(initial_chosen_item)
                                } else {
                                    None // Collision
                                }
                            };

                            if let Some(selected_id) = final_item_to_add {
                                // Check reject list again (crush_select_leaf should have also checked, but good for safety)
                                if !local_reject_list.contains(&selected_id) {
                                    next_selection.push(selected_id);
                                    local_reject_list.insert(selected_id); // Add to reject list for future slots/retries
                                    item_found_for_this_slot = true;
                                    break; // Found item for this slot `i`, break from input_item_id loop
                                }
                            }
                        } // End loop through current_selection (input items)
                        if item_found_for_this_slot {
                            break; // Break from try_num loop for this slot `i`
                        }
                    } // End try_num loop

                    if !item_found_for_this_slot {
                         println!("{}: Could not find a suitable item for output slot {} after {} retries.", op_name, i, retries_for_slot);
                         // Optionally, add a placeholder like CRUSH_ITEM_NONE if defined and handled later
                    }
                } // End loop for each output slot `i`

                current_selection = next_selection;
                println!("{} completed. New current_selection: {:?} (size: {})", op_name, current_selection, current_selection.len());
            }
            Some(CrushOpcode::Noop) => {
                println!("Executing NoOp (op: {:?})", step.op);
            }
            Some(CrushOpcode::SetChooseTries) => {
                 println!("Executing SetChooseTries (op: {}, val: {})", step.op, step.arg1);
                 // In a full implementation, this would set a variable in the mapper state.
            }
            Some(CrushOpcode::SetChooseLeafTries) => {
                 println!("Executing SetChooseLeafTries (op: {}, val: {})", step.op, step.arg1);
            }
            Some(CrushOpcode::SetChooseLocalTries) => {
                println!("Executing SetChooseLocalTries (op: {}, val: {})", step.op, step.arg1);
            }
            Some(CrushOpcode::SetChooseLocalFallbackTries) => {
                println!("Executing SetChooseLocalFallbackTries (op: {}, val: {})", step.op, step.arg1);
            }
            Some(CrushOpcode::SetChooseLeafVaryR) => {
                println!("Executing SetChooseLeafVaryR (op: {}, val: {})", step.op, step.arg1);
            }
             Some(CrushOpcode::SetChooseLeafStable) => {
                println!("Executing SetChooseLeafStable (op: {}, val: {})", step.op, step.arg1);
            }
            None => {
                eprintln!("Unknown or unhandled CRUSH opcode: {}. Step details: {:?}", step.op, step);
                // Potentially stop processing or handle as an error. For now, continue.
            }
        }
    }

    // Final check: if current_selection is not empty and last op wasn't EMIT,
    // some CRUSH versions might implicitly emit.
    // However, explicit EMIT is safer. We'll rely on explicit EMITs.
    if !current_selection.is_empty() {
        println!("Warning: Rule {} finished with non-empty current_selection: {:?}. This selection was not emitted.", rule_no, current_selection);
    }

    result.len() - original_result_len
}
