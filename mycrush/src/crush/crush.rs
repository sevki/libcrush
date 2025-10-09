#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::crush::types::ffi;
use crate::crush::types::*;

unsafe extern "C" {
    fn free(_: *mut ffi::c_void);
}
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
pub unsafe fn crush_get_bucket_item_weight(b: *const CrushBucket, p: ffi::c_int) -> ffi::c_int {
    unsafe {
        if p as U32 >= (*b).size {
            return 0;
        }
        match (*b).alg as ffi::c_int {
            1 => (*(b as *mut CrushBucketUniform)).item_weight as ffi::c_int,
            2 => *((*(b as *mut CrushBucketList)).item_weights).offset(p as isize) as ffi::c_int,
            3 => {
                *((*(b as *mut CrushBucketTree)).node_weights).offset(crush_calc_tree_node(p) as isize)
                    as ffi::c_int
            }
            4 => *((*(b as *mut CrushBucketStraw)).item_weights).offset(p as isize) as ffi::c_int,
            5 => *((*(b as *mut CrushBucketStraw2)).item_weights).offset(p as isize) as ffi::c_int,
            _ => 0,
        }
    }
}
pub unsafe fn crush_destroy_bucket_uniform(b: *mut CrushBucketUniform) {
    unsafe {
        if !((*b).h.items).is_null() {
            free((*b).h.items as *mut ffi::c_void);
        }
        if !b.is_null() {
            free(b as *mut ffi::c_void);
        }
    }
}
pub unsafe fn crush_destroy_bucket_list(b: *mut CrushBucketList) {
    unsafe {
        if !((*b).item_weights).is_null() {
            free((*b).item_weights as *mut ffi::c_void);
        }
        if !((*b).sum_weights).is_null() {
            free((*b).sum_weights as *mut ffi::c_void);
        }
        if !((*b).h.items).is_null() {
            free((*b).h.items as *mut ffi::c_void);
        }
        if !b.is_null() {
            free(b as *mut ffi::c_void);
        }
    }
}
pub unsafe fn crush_destroy_bucket_tree(b: *mut CrushBucketTree) {
    unsafe {
        if !((*b).h.items).is_null() {
            free((*b).h.items as *mut ffi::c_void);
        }
        if !((*b).node_weights).is_null() {
            free((*b).node_weights as *mut ffi::c_void);
        }
        if !b.is_null() {
            free(b as *mut ffi::c_void);
        }
    }
}
pub unsafe fn crush_destroy_bucket_straw(b: *mut CrushBucketStraw) {
    unsafe {
        if !((*b).straws).is_null() {
            free((*b).straws as *mut ffi::c_void);
        }
        if !((*b).item_weights).is_null() {
            free((*b).item_weights as *mut ffi::c_void);
        }
        if !((*b).h.items).is_null() {
            free((*b).h.items as *mut ffi::c_void);
        }
        if !b.is_null() {
            free(b as *mut ffi::c_void);
        }
    }
}
pub unsafe fn crush_destroy_bucket_straw2(b: *mut CrushBucketStraw2) {
    unsafe {
        if !((*b).item_weights).is_null() {
            free((*b).item_weights as *mut ffi::c_void);
        }
        if !((*b).h.items).is_null() {
            free((*b).h.items as *mut ffi::c_void);
        }
        if !b.is_null() {
            free(b as *mut ffi::c_void);
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
            if !((*map).buckets).is_null() {
                free((*map).buckets as *mut ffi::c_void);
            }
        }
        if !((*map).rules).is_null() {
            for b in 0..(*map).max_rules {
                crush_destroy_rule(*((*map).rules).offset(b as isize));
            }
            if !((*map).rules).is_null() {
                free((*map).rules as *mut ffi::c_void);
            }
        }
        if !((*map).choose_tries).is_null() {
            free((*map).choose_tries as *mut ffi::c_void);
        }
        if !map.is_null() {
            free(map as *mut ffi::c_void);
        }
    }
}
pub unsafe fn crush_destroy_rule(rule: *mut CrushRule) {
    unsafe {
        if !rule.is_null() {
            free(rule as *mut ffi::c_void);
        }
    }
}
