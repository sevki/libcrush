// Allow access to mycrush crate's public items
use mycrush::{
    builder::*,
    mapper::*,
    CrushRuleStep,
    CrushOpcode,
    CrushRuleMask,
    CrushMap as MyCrushMap,
    CrushBucket,
    CrushBucketUniform,
    CrushBucketCommon,
    CrushRule as MyCrushRule,
    CrushAlgorithm, // Added for alg constants
};
use crush_sys;

use std::collections::HashSet;
// use std::ffi::CString; // Not used in this version

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
        hash: crush_sys::CRUSH_HASH_RJENKINS1 as u8,
        weight: 0x10000, // 1.0
        size: 1,
        items: vec![device_id],
    };
    // In mycrush, CrushBucketUniform holds CrushBucketCommon directly, not 'h' field.
    let my_uniform_bucket_params = CrushBucketUniform {
        common: my_bucket_common, // field name is 'common'
        item_weight: 0x10000,
    };
    let my_bucket = CrushBucket::Uniform(my_uniform_bucket_params);
    let assigned_my_bucket_id = add_bucket(&mut my_map, my_bucket).expect("Failed to add bucket to my_map");
    assert_eq!(assigned_my_bucket_id, bucket_id_internal);
    my_map.max_devices = 1;

    let my_rule_steps = vec![
        CrushRuleStep { op: CrushOpcode::Take as u32, arg1: assigned_my_bucket_id, arg2: 0 }, // op is u32
        CrushRuleStep { op: CrushOpcode::Emit as u32, arg1: 0, arg2: 0 },
    ];
    let my_rule = MyCrushRule {
        mask: CrushRuleMask { ruleset: 0, r#type: 0, min_size: 1, max_size: 1 },
        steps: my_rule_steps.clone(), // steps field in MyCrushRule
        len: my_rule_steps.len() as u32,
    };
    add_rule(&mut my_map, my_rule, Some(rule_id)).expect("Failed to add rule to my_map");

    // 3. Build an equivalent CrushMap using `crush-sys` (C API)
    let c_map_ptr: *mut crush_sys::crush_map;
    unsafe {
        c_map_ptr = crush_sys::crush_create();
        assert!(!c_map_ptr.is_null(), "Failed to create C crush_map");

        // Match tunables from my_map to c_map if they are used by crush_do_rule implicitly
        // For this simple test, default C tunables might be okay, or set them:
        (*c_map_ptr).choose_local_tries = my_map.choose_local_tries;
        (*c_map_ptr).choose_local_fallback_tries = my_map.choose_local_fallback_tries;
        (*c_map_ptr).choose_total_tries = my_map.choose_total_tries;
        (*c_map_ptr).chooseleaf_descend_once = my_map.chooseleaf_descend_once;
        // (*c_map_ptr).chooseleaf_vary_r = my_map.chooseleaf_vary_r; // chooseleaf_vary_r is u8
        // (*c_map_ptr).chooseleaf_stable = my_map.chooseleaf_stable; // chooseleaf_stable is u8
        // Note: Direct field access for chooseleaf_vary_r and chooseleaf_stable might be tricky if they are bitfields or part of a packed struct in C.
        // Using provided functions if available is safer. For now, this is a direct attempt.
        // crush_sys::crush_set_chooseleaf_vary_r(c_map_ptr, my_map.chooseleaf_vary_r as ::std::os::raw::c_int);
        // crush_sys::crush_set_chooseleaf_stable(c_map_ptr, my_map.chooseleaf_stable as ::std::os::raw::c_int);


        let c_items = vec![device_id];
        // For uniform bucket, crush_make_bucket expects an array for weights where the first element is the item_weight.
        let c_bucket_item_weights = vec![0x10000_u32];

        let c_bucket_id: ::std::os::raw::c_int = bucket_id_internal;
        let c_bucket_ptr = crush_sys::crush_make_bucket(
            c_map_ptr,
            crush_sys::CRUSH_BUCKET_UNIFORM as ::std::os::raw::c_int,
            crush_sys::CRUSH_HASH_RJENKINS1 as ::std::os::raw::c_int,
            0, // type (c_ushort / u16)
            c_items.len() as ::std::os::raw::c_int,
            c_items.as_ptr() as *mut ::std::os::raw::c_int,
            c_bucket_item_weights.as_ptr() as *mut u32, // Corrected to u32
        );
        assert!(!c_bucket_ptr.is_null(), "Failed to create C uniform bucket");

        let mut id_out: ::std::os::raw::c_int = 0;
        // crush_add_bucket in newer Ceph takes bucket_id as the *index* if positive, or 0 for next.
        // To force a specific negative ID, one might need to use crush_add_bucket_legacy or manipulate map->buckets directly (unsafe).
        // For simplicity, let's try with bucket_no = 0 (next available), then verify if it matches our expected index logic.
        // Ceph assigns IDs like -1, -2, ... which map to bucket indices 0, 1, ...
        // So if we add the first bucket, its ID should be -1.
        let ret = crush_sys::crush_add_bucket(c_map_ptr, 0, c_bucket_ptr, &mut id_out);
        assert!(ret == 0, "crush_add_bucket failed, ret: {}", ret);
        assert_eq!(id_out, c_bucket_id, "C bucket ID mismatch. Expected {}, got {}", c_bucket_id, id_out);

        let c_rule_len = 2;
        let c_rule_ptr = crush_sys::crush_make_rule(
            c_rule_len,
            0, // ruleset
            0, // type
            1, // minsize
            1, // maxsize
        );
        assert!(!c_rule_ptr.is_null(), "Failed to create C rule");
        // Use id_out which is the actual ID assigned by C library for the bucket.
        crush_sys::crush_rule_set_step(c_rule_ptr, 0, crush_sys::CRUSH_RULE_TAKE as i32, id_out, 0);
        crush_sys::crush_rule_set_step(c_rule_ptr, 1, crush_sys::CRUSH_RULE_EMIT as i32, 0, 0);

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
        let c_workspace_size = crush_sys::crush_work_size(c_map_ptr, result_max as ::std::os::raw::c_int);
        let mut c_workspace: Vec<u8> = vec![0; c_workspace_size as usize];
        crush_sys::crush_init_workspace(c_map_ptr, c_workspace.as_mut_ptr() as *mut ::std::os::raw::c_void);

        let c_selection_count = crush_sys::crush_do_rule(
            c_map_ptr,
            rule_id as ::std::os::raw::c_int,
            x as ::std::os::raw::c_int,
            c_result_vec.as_mut_ptr(),
            result_max as ::std::os::raw::c_int,
            std::ptr::null_mut(), // osd_weight
            0,
            c_workspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
            std::ptr::null_mut(),
        );
        c_result_vec.truncate(c_selection_count as usize);

        assert_eq!(my_selection_count, c_selection_count as usize, "Selection counts differ");
        assert_eq!(my_result_vec, c_result_vec, "Selection results differ");

        crush_sys::crush_destroy(c_map_ptr);
    }
}
