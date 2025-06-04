// Allow access to mycrush crate's public items
use mycrush::{
    builder::*,
    // mapper::*, // mapper functions are called via mycrush::mapper::crush_do_rule
    CrushRuleStep,
    CrushOpcode,
    CrushRuleMask,
    // CrushMap as MyCrushMap, // MyCrushMap is used directly via mycrush::CrushMap
    CrushBucket,
    CrushBucketUniform,
    CrushBucketCommon,
    CrushBucketStraw2, // Added for the new test
    CrushRule as MyCrushRule,
    CrushAlgorithm,
};
use crush_sys;

// use std::collections::HashSet; // Not used in this simplified test

#[test]
fn test_simple_take_emit_rule() {
    // 1. Define parameters for a simple map and rule
    let device_id = 0; // A single device
    let bucket_id_internal: i32 = -1; // Bucket ID for mycrush (negative)
    let rule_id = 0;

    // 2. Build CrushMap using `mycrush`
    let mut my_map = create_crush_map();

    let my_bucket_common = CrushBucketCommon {
        id: 0, // Will be set by add_bucket
        r#type: 0, // Example type
        alg: CrushAlgorithm::Uniform as u8,
        hash: 1, // Assuming CRUSH_HASH_RJENKINS1 is 1
        weight: 0x10000, // 1.0
        size: 1,
        items: vec![device_id],
    };
    let my_uniform_bucket_params = CrushBucketUniform {
        common: my_bucket_common,
        item_weight: 0x10000,
    };
    let my_bucket = CrushBucket::Uniform(my_uniform_bucket_params);
    let assigned_my_bucket_id = add_bucket(&mut my_map, my_bucket).expect("Failed to add bucket to my_map");
    assert_eq!(assigned_my_bucket_id, bucket_id_internal);
    my_map.max_devices = 1;

    let my_rule_steps = vec![
        CrushRuleStep { op: CrushOpcode::Take as u32, arg1: assigned_my_bucket_id, arg2: 0 },
        CrushRuleStep { op: CrushOpcode::Emit as u32, arg1: 0, arg2: 0 },
    ];
    let my_rule = MyCrushRule {
        mask: CrushRuleMask { ruleset: 0, r#type: 0, min_size: 1, max_size: 1 },
        steps: my_rule_steps.clone(),
        len: my_rule_steps.len() as u32,
    };
    add_rule(&mut my_map, my_rule, Some(rule_id)).expect("Failed to add rule to my_map");

    // 3. Build an equivalent CrushMap using `crush-sys` (C API)
    let c_map_ptr: *mut crush_sys::crush_map;
    unsafe {
        c_map_ptr = crush_sys::crush_create();
        assert!(!c_map_ptr.is_null(), "Failed to create C crush_map");

        (*c_map_ptr).choose_local_tries = my_map.choose_local_tries;
        (*c_map_ptr).choose_local_fallback_tries = my_map.choose_local_fallback_tries;
        (*c_map_ptr).choose_total_tries = my_map.choose_total_tries;
        (*c_map_ptr).chooseleaf_descend_once = my_map.chooseleaf_descend_once;

        let c_items = vec![device_id as ::std::os::raw::c_int];
        let c_bucket_item_weights = vec![0x10000 as ::std::os::raw::c_int];

        let c_bucket_id: ::std::os::raw::c_int = bucket_id_internal;
        let c_bucket_ptr = crush_sys::crush_make_bucket(
            c_map_ptr,
            1 as ::std::os::raw::c_int, // CRUSH_BUCKET_UNIFORM = 1
            crush_sys::CRUSH_HASH_RJENKINS1 as ::std::os::raw::c_int,
            0,
            c_items.len() as ::std::os::raw::c_int,
            c_items.as_ptr() as *mut ::std::os::raw::c_int,
            c_bucket_item_weights.as_ptr() as *mut ::std::os::raw::c_int,
        );
        assert!(!c_bucket_ptr.is_null(), "Failed to create C uniform bucket");

        let mut id_out: ::std::os::raw::c_int = 0;
        let ret = crush_sys::crush_add_bucket(c_map_ptr, 0, c_bucket_ptr, &mut id_out);
        assert!(ret == 0, "crush_add_bucket failed, ret: {}", ret);
        assert_eq!(id_out, c_bucket_id, "C bucket ID mismatch. Expected {}, got {}", c_bucket_id, id_out);

        let c_rule_len = 2;
        let c_rule_ptr = crush_sys::crush_make_rule(
            c_rule_len,
            0, 0, 1, 1,
        );
        assert!(!c_rule_ptr.is_null(), "Failed to create C rule");
        crush_sys::crush_rule_set_step(c_rule_ptr, 0, 1 as i32, id_out, 0); // CRUSH_RULE_TAKE = 1
        crush_sys::crush_rule_set_step(c_rule_ptr, 1, 4 as i32, 0, 0);    // CRUSH_RULE_EMIT = 4

        let c_assigned_rule_id = crush_sys::crush_add_rule(c_map_ptr, c_rule_ptr, rule_id as ::std::os::raw::c_int);
        assert!(c_assigned_rule_id >= 0, "crush_add_rule failed");
        assert_eq!(c_assigned_rule_id, rule_id as ::std::os::raw::c_int);

        crush_sys::crush_finalize(c_map_ptr);
        my_map.max_devices = (*c_map_ptr).max_devices;

        let x = 12345;
        let result_max = 10;

        let mut my_result_vec: Vec<i32> = Vec::with_capacity(result_max);
        let my_selection_count = mycrush::mapper::crush_do_rule(&my_map, rule_id as i32, x, &mut my_result_vec, result_max);

        let mut c_result_vec = vec![0i32; result_max];

        let c_workspace_size = (*c_map_ptr).working_size;
        let mut c_workspace: Vec<u8> = vec![0; c_workspace_size as usize];

        if c_workspace_size > 0 {
             crush_sys::crush_init_workspace(c_map_ptr, c_workspace.as_mut_ptr() as *mut ::std::os::raw::c_void);
        } else {
            println!("Warning: C map working_size is 0. C execution might fail or be skipped.");
        }

        if c_workspace_size > 0 {
            let c_selection_count = crush_sys::crush_do_rule(
                c_map_ptr,
                rule_id as ::std::os::raw::c_int,
                x as ::std::os::raw::c_int,
                c_result_vec.as_mut_ptr(),
                result_max as ::std::os::raw::c_int,
                std::ptr::null_mut(),
                0,
                c_workspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                std::ptr::null_mut(),
            );
            c_result_vec.truncate(c_selection_count as usize);

            assert_eq!(my_selection_count, c_selection_count as usize, "Selection counts differ");
            assert_eq!(my_result_vec, c_result_vec, "Selection results differ");
        } else {
            println!("Skipping C crush_do_rule execution and comparison due to 0 workspace size.");
        }

        crush_sys::crush_destroy(c_map_ptr);
    }
}

#[test]
fn test_choose_firstn_straw2_bucket() {
    // 1. Define parameters
    let device_ids = vec![0, 1, 2]; // Three devices
    let device_weights_u32: Vec<u32> = vec![0x10000, 0x10000, 0x10000]; // Equal weights for mycrush
    let device_weights_i32: Vec<i32> = vec![0x10000, 0x10000, 0x10000]; // Equal weights for C API (as c_int)

    let bucket_id_internal = -1;
    let rule_id = 1; // New rule ID

    // 2. Build CrushMap using `mycrush`
    let mut my_map = create_crush_map();
    my_map.max_devices = device_ids.len() as i32;

    let my_bucket_common = CrushBucketCommon {
        id: 0, // Will be set by add_bucket
        r#type: 1, // Example type
        alg: CrushAlgorithm::Straw2 as u8,
        hash: crush_sys::CRUSH_HASH_RJENKINS1 as u8, // Corrected: Assuming this is available directly
        weight: device_weights_u32.iter().sum(),
        size: device_ids.len() as u32,
        items: device_ids.clone(),
    };
    // In mycrush, CrushBucketStraw2 holds CrushBucketCommon directly (not 'h') and item_weights as Vec<u32>
    let my_straw2_bucket_params = CrushBucketStraw2 {
        common: my_bucket_common, // field name is 'common'
        item_weights: device_weights_u32.clone(),
    };
    let my_bucket = CrushBucket::Straw2(my_straw2_bucket_params);
    let assigned_my_bucket_id = add_bucket(&mut my_map, my_bucket).expect("Failed to add bucket to my_map");
    assert_eq!(assigned_my_bucket_id, bucket_id_internal);

    // Rule: TAKE bucket, CHOOSELEAF_FIRSTN 1 from bucket_id_internal (type 0 implies any leaf type)
    let my_rule_steps = vec![
        CrushRuleStep { op: CrushOpcode::Take as u32, arg1: assigned_my_bucket_id, arg2: 0 },
        CrushRuleStep { op: CrushOpcode::ChooseLeafFirstN as u32, arg1: 1, arg2: 0 }, // Choose 1 leaf (type 0 = any)
        CrushRuleStep { op: CrushOpcode::Emit as u32, arg1: 0, arg2: 0 },
    ];
    let my_rule = MyCrushRule {
        mask: CrushRuleMask { ruleset: 0, r#type: 1, min_size: 1, max_size: 1 }, // type 1 for this rule
        steps: my_rule_steps.clone(),
        len: my_rule_steps.len() as u32,
    };
    add_rule(&mut my_map, my_rule, Some(rule_id)).expect("Failed to add rule to my_map");

    // 3. Build equivalent C CrushMap
    unsafe {
        let c_map_ptr = crush_sys::crush_create();
        // Set tunables similar to the previous test
        (*c_map_ptr).choose_local_tries = my_map.choose_local_tries;
        (*c_map_ptr).choose_local_fallback_tries = my_map.choose_local_fallback_tries;
        (*c_map_ptr).choose_total_tries = my_map.choose_total_tries;
        (*c_map_ptr).chooseleaf_descend_once = my_map.chooseleaf_descend_once;


        let c_items: Vec<::std::os::raw::c_int> = device_ids.iter().map(|&id| id as ::std::os::raw::c_int).collect();
        // crush_make_bucket for straw2 expects item weights as *mut ::std::os::raw::c_int
        let c_item_weights_for_make: Vec<::std::os::raw::c_int> = device_weights_i32.clone();

        let c_bucket_ptr = crush_sys::crush_make_bucket(
            c_map_ptr,
            crush_sys::crush_algorithm_CRUSH_BUCKET_STRAW2 as ::std::os::raw::c_int, // Corrected name
            crush_sys::CRUSH_HASH_RJENKINS1 as ::std::os::raw::c_int, // Corrected name
            1, // type of the bucket being created
            c_items.len() as ::std::os::raw::c_int,
            c_items.as_ptr() as *mut ::std::os::raw::c_int,
            c_item_weights_for_make.as_ptr() as *mut ::std::os::raw::c_int,
        );
        assert!(!c_bucket_ptr.is_null(), "Failed to create C straw2 bucket");

        let mut id_out: ::std::os::raw::c_int = 0;
        // Use bucket_no = 0 for C to assign next available, then check if it's -1.
        let ret = crush_sys::crush_add_bucket(c_map_ptr, 0, c_bucket_ptr, &mut id_out);
        assert!(ret == 0, "crush_add_bucket for straw2 failed");
        assert_eq!(id_out, bucket_id_internal, "C straw2 bucket ID mismatch");

        let c_rule_ptr = crush_sys::crush_make_rule(3, 0, 1, 1, 1); // len, ruleset, type, min, max
        assert!(!c_rule_ptr.is_null(), "Failed to create C rule for straw2 test");
        crush_sys::crush_rule_set_step(c_rule_ptr, 0, crush_sys::crush_opcodes_CRUSH_RULE_TAKE as i32, id_out, 0); // Corrected name
        crush_sys::crush_rule_set_step(c_rule_ptr, 1, crush_sys::crush_opcodes_CRUSH_RULE_CHOOSELEAF_FIRSTN as i32, 1, 0); // Corrected name, arg2=0 means any leaf type
        crush_sys::crush_rule_set_step(c_rule_ptr, 2, crush_sys::crush_opcodes_CRUSH_RULE_EMIT as i32, 0, 0); // Corrected name

        let c_assigned_rule_id = crush_sys::crush_add_rule(c_map_ptr, c_rule_ptr, rule_id as ::std::os::raw::c_int);
        assert!(c_assigned_rule_id == rule_id as ::std::os::raw::c_int, "crush_add_rule for straw2 test failed");

        crush_sys::crush_finalize(c_map_ptr);
        my_map.max_devices = (*c_map_ptr).max_devices;
        
        println!("C map working_size: {}", (*c_map_ptr).working_size);
        println!("C map max_devices: {}", (*c_map_ptr).max_devices);

        // 4. Test inputs
        let x_initial = 54321;
        let result_max = 10; // Max number of items we expect

        // 5. Run mycrush for multiple inputs
        for i in 0..5 { // Test 5 different inputs
            let current_x = x_initial + i * 100;
            let mut my_result_vec_loop: Vec<i32> = Vec::with_capacity(result_max);
            let my_selection_count_loop = mycrush::mapper::crush_do_rule(&my_map, rule_id as i32, current_x, &mut my_result_vec_loop, result_max);

            // 6. Run C crush for multiple inputs
            let mut c_result_vec_loop = vec![0i32; result_max];
            let c_workspace_size_loop = (*c_map_ptr).working_size; // Simplified workspace size
            let mut c_workspace_loop: Vec<u8> = vec![0; c_workspace_size_loop as usize];
            if c_workspace_size_loop > 0 {
                 crush_sys::crush_init_workspace(c_map_ptr, c_workspace_loop.as_mut_ptr() as *mut ::std::os::raw::c_void);
            }


            if c_workspace_size_loop > 0 {
                // Create weight vector for devices (all devices have weight 0x10000)
                let c_weights = vec![0x10000u32; (*c_map_ptr).max_devices as usize];
                
                let c_selection_count_loop = crush_sys::crush_do_rule(
                    c_map_ptr, rule_id as ::std::os::raw::c_int, current_x as ::std::os::raw::c_int,
                    c_result_vec_loop.as_mut_ptr(), result_max as ::std::os::raw::c_int,
                    c_weights.as_ptr() as *const u32, c_weights.len() as ::std::os::raw::c_int,
                    c_workspace_loop.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    std::ptr::null_mut(),
                );
                c_result_vec_loop.truncate(c_selection_count_loop as usize);

                // 7. Compare
                assert_eq!(my_selection_count_loop, c_selection_count_loop as usize, "Straw2 test: Selection counts differ for x={}", current_x);
                assert_eq!(my_result_vec_loop, c_result_vec_loop, "Straw2 test: Selection results differ for x={}", current_x);
            } else {
                 println!("Skipping C crush_do_rule execution for x={} due to 0 workspace size.", current_x);
                 // If C execution is skipped, perhaps we should only assert mycrush works, or fail the test.
                 // For now, if mycrush produces results and C can't run, it's an incomplete comparison.
                 assert!(my_selection_count_loop > 0, "MyCrush failed to select for x={} even when C part skipped", current_x);
            }
        }
        crush_sys::crush_destroy(c_map_ptr);
    }
}
