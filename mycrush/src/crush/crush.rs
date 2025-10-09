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
use crate::crush::types::ffi;

#[inline]
fn crush_calc_tree_node(i: i32) -> i32 {
    ((i + 1) << 1) - 1
}
pub fn crush_bucket_alg_name(alg: ffi::c_int) -> *const ffi::c_char {
    match alg {
        1 => b"uniform\0" as *const u8 as *const ffi::c_char,
        2 => b"list\0" as *const u8 as *const ffi::c_char,
        3 => b"tree\0" as *const u8 as *const ffi::c_char,
        4 => b"straw\0" as *const u8 as *const ffi::c_char,
        5 => b"straw2\0" as *const u8 as *const ffi::c_char,
        _ => b"unknown\0" as *const u8 as *const ffi::c_char,
    }
}
pub unsafe fn crush_get_bucket_item_weight(
    b: *const CrushBucket,
    p: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        if p as U32 >= (*b).size {
            return 0;
        }
        match (*b).alg as ffi::c_int {
            1 => (*(b as *mut CrushBucketUniform)).item_weight as ffi::c_int,
            2 => *((*(b as *mut CrushBucketList)).item_weights).offset(p as isize) as ffi::c_int,
            3 => *((*(b as *mut CrushBucketTree)).node_weights)
                .offset(crush_calc_tree_node(p) as isize) as ffi::c_int,
            4 => *((*(b as *mut CrushBucketStraw)).item_weights).offset(p as isize) as ffi::c_int,
            5 => *((*(b as *mut CrushBucketStraw2)).item_weights).offset(p as isize) as ffi::c_int,
            _ => 0,
        }
    }
}
pub unsafe fn crush_destroy_bucket_uniform(b: *mut CrushBucketUniform) {
    unsafe {
        if !((*b).h.items).is_null() {
            // Reconstruct Vec to properly deallocate using correct capacity
            let _ = Vec::from_raw_parts((*b).h.items, (*b).h.size as usize, (*b).h.items_capacity as usize);
        }
        if !b.is_null() {
            let _ = Box::from_raw(b);
        }
    }
}
pub unsafe fn crush_destroy_bucket_list(b: *mut CrushBucketList) {
    unsafe {
        if !((*b).item_weights).is_null() {
            let _ = Vec::from_raw_parts((*b).item_weights, (*b).h.size as usize, (*b).item_weights_capacity as usize);
        }
        if !((*b).sum_weights).is_null() {
            let _ = Vec::from_raw_parts((*b).sum_weights, (*b).h.size as usize, (*b).sum_weights_capacity as usize);
        }
        if !((*b).h.items).is_null() {
            let _ = Vec::from_raw_parts((*b).h.items, (*b).h.size as usize, (*b).h.items_capacity as usize);
        }
        if !b.is_null() {
            let _ = Box::from_raw(b);
        }
    }
}
pub unsafe fn crush_destroy_bucket_tree(b: *mut CrushBucketTree) {
    unsafe {
        if !((*b).h.items).is_null() {
            let _ = Vec::from_raw_parts((*b).h.items, (*b).h.size as usize, (*b).h.items_capacity as usize);
        }
        if !((*b).node_weights).is_null() {
            let _ = Vec::from_raw_parts((*b).node_weights, (*b).num_nodes as usize, (*b).node_weights_capacity as usize);
        }
        if !b.is_null() {
            let _ = Box::from_raw(b);
        }
    }
}
pub unsafe fn crush_destroy_bucket_straw(b: *mut CrushBucketStraw) {
    unsafe {
        if !((*b).straws).is_null() {
            let _ = Vec::from_raw_parts((*b).straws, (*b).h.size as usize, (*b).straws_capacity as usize);
        }
        if !((*b).item_weights).is_null() {
            let _ = Vec::from_raw_parts((*b).item_weights, (*b).h.size as usize, (*b).item_weights_capacity as usize);
        }
        if !((*b).h.items).is_null() {
            let _ = Vec::from_raw_parts((*b).h.items, (*b).h.size as usize, (*b).h.items_capacity as usize);
        }
        if !b.is_null() {
            let _ = Box::from_raw(b);
        }
    }
}
pub unsafe fn crush_destroy_bucket_straw2(b: *mut CrushBucketStraw2) {
    unsafe {
        if !((*b).item_weights).is_null() {
            let _ = Vec::from_raw_parts((*b).item_weights, (*b).h.size as usize, (*b).item_weights_capacity as usize);
        }
        if !((*b).h.items).is_null() {
            let _ = Vec::from_raw_parts((*b).h.items, (*b).h.size as usize, (*b).h.items_capacity as usize);
        }
        if !b.is_null() {
            let _ = Box::from_raw(b);
        }
    }
}
pub unsafe fn crush_destroy_bucket(b: *mut CrushBucket) {
    unsafe {
        match (*b).alg as ffi::c_int {
            1 => crush_destroy_bucket_uniform(b as *mut CrushBucketUniform),
            2 => crush_destroy_bucket_list(b as *mut CrushBucketList),
            3 => crush_destroy_bucket_tree(b as *mut CrushBucketTree),
            4 => crush_destroy_bucket_straw(b as *mut CrushBucketStraw),
            5 => crush_destroy_bucket_straw2(b as *mut CrushBucketStraw2),
            _ => {}
        }
    }
}
pub unsafe fn crush_destroy(map: *mut CrushMap) {
    unsafe {
        if !((*map).buckets).is_null() {
            for b in 0..(*map).max_buckets {
                if !(*((*map).buckets).offset(b as isize)).is_null() {
                    crush_destroy_bucket(*((*map).buckets).offset(b as isize));
                }
            }
            // Reconstruct Vec to properly deallocate
            let _ = Vec::from_raw_parts((*map).buckets, (*map).max_buckets as usize, (*map).max_buckets as usize);
        }
        if !((*map).rules).is_null() {
            for b in 0..(*map).max_rules {
                crush_destroy_rule(*((*map).rules).offset(b as isize));
            }
            // Reconstruct Vec to properly deallocate
            let _ = Vec::from_raw_parts((*map).rules, (*map).max_rules as usize, (*map).max_rules as usize);
        }
        if !((*map).choose_tries).is_null() {
            // choose_tries is allocated elsewhere, we need to check how it's allocated
            // For now, use Box::from_raw assuming it was allocated with Box
            let _ = Box::from_raw((*map).choose_tries);
        }
        if !map.is_null() {
            let _ = Box::from_raw(map);
        }
    }
}
pub unsafe fn crush_destroy_rule(rule: *mut CrushRule) {
    unsafe {
        if !rule.is_null() {
            let layout = std::alloc::Layout::from_size_align_unchecked(
                ::core::mem::size_of::<CrushRule>()
                    + ((*rule).len as usize) * ::core::mem::size_of::<CrushRuleStep>(),
                ::core::mem::align_of::<CrushRule>(),
            );
            std::alloc::dealloc(rule as *mut u8, layout);
        }
    }
}
