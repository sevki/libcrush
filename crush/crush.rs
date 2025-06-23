#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn free(_: *mut std::ffi::c_void);
}
pub type __u8 = std::ffi::c_uchar;
pub type __u16 = std::ffi::c_ushort;
pub type __s32 = std::ffi::c_int;
pub type __u32 = std::ffi::c_uint;
pub type size_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_rule_step {
    pub op: __u32,
    pub arg1: __s32,
    pub arg2: __s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_rule_mask {
    pub ruleset: __u8,
    pub type_0: __u8,
    pub min_size: __u8,
    pub max_size: __u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_rule {
    pub len: __u32,
    pub mask: crush_rule_mask,
    pub steps: [crush_rule_step; 0],
}
pub type crush_algorithm = std::ffi::c_uint;
pub const CRUSH_BUCKET_STRAW2: crush_algorithm = 5;
pub const CRUSH_BUCKET_STRAW: crush_algorithm = 4;
pub const CRUSH_BUCKET_TREE: crush_algorithm = 3;
pub const CRUSH_BUCKET_LIST: crush_algorithm = 2;
pub const CRUSH_BUCKET_UNIFORM: crush_algorithm = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket {
    pub id: __s32,
    pub type_0: __u16,
    pub alg: __u8,
    pub hash: __u8,
    pub weight: __u32,
    pub size: __u32,
    pub items: *mut __s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_uniform {
    pub h: crush_bucket,
    pub item_weight: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_list {
    pub h: crush_bucket,
    pub item_weights: *mut __u32,
    pub sum_weights: *mut __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_tree {
    pub h: crush_bucket,
    pub num_nodes: __u8,
    pub node_weights: *mut __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_straw {
    pub h: crush_bucket,
    pub item_weights: *mut __u32,
    pub straws: *mut __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_straw2 {
    pub h: crush_bucket,
    pub item_weights: *mut __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_map {
    pub buckets: *mut *mut crush_bucket,
    pub rules: *mut *mut crush_rule,
    pub max_buckets: __s32,
    pub max_rules: __u32,
    pub max_devices: __s32,
    pub choose_local_tries: __u32,
    pub choose_local_fallback_tries: __u32,
    pub choose_total_tries: __u32,
    pub chooseleaf_descend_once: __u32,
    pub chooseleaf_vary_r: __u8,
    pub chooseleaf_stable: __u8,
    pub working_size: size_t,
    pub straw_calc_version: __u8,
    pub allowed_bucket_algs: __u32,
    pub choose_tries: *mut __u32,
}
#[inline]
unsafe extern "C" fn crush_calc_tree_node(mut i: std::ffi::c_int) -> std::ffi::c_int {
    return ((i + 1 as std::ffi::c_int) << 1 as std::ffi::c_int) - 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn crush_bucket_alg_name(
    mut alg: std::ffi::c_int,
) -> *const std::ffi::c_char {
    match alg {
        1 => return b"uniform\0" as *const u8 as *const std::ffi::c_char,
        2 => return b"list\0" as *const u8 as *const std::ffi::c_char,
        3 => return b"tree\0" as *const u8 as *const std::ffi::c_char,
        4 => return b"straw\0" as *const u8 as *const std::ffi::c_char,
        5 => return b"straw2\0" as *const u8 as *const std::ffi::c_char,
        _ => return b"unknown\0" as *const u8 as *const std::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn crush_get_bucket_item_weight(
    mut b: *const crush_bucket,
    mut p: std::ffi::c_int,
) -> std::ffi::c_int {
    if p as __u32 >= (*b).size {
        return 0 as std::ffi::c_int;
    }
    match (*b).alg as std::ffi::c_int {
        1 => return (*(b as *mut crush_bucket_uniform)).item_weight as std::ffi::c_int,
        2 => {
            return *((*(b as *mut crush_bucket_list)).item_weights).offset(p as isize)
                as std::ffi::c_int;
        }
        3 => {
            return *((*(b as *mut crush_bucket_tree)).node_weights)
                .offset(crush_calc_tree_node(p) as isize) as std::ffi::c_int;
        }
        4 => {
            return *((*(b as *mut crush_bucket_straw)).item_weights).offset(p as isize)
                as std::ffi::c_int;
        }
        5 => {
            return *((*(b as *mut crush_bucket_straw2)).item_weights).offset(p as isize)
                as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_uniform(mut b: *mut crush_bucket_uniform) {
    if !((*b).h.items).is_null() {
        free((*b).h.items as *mut std::ffi::c_void);
    }
    if !b.is_null() {
        free(b as *mut std::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_list(mut b: *mut crush_bucket_list) {
    if !((*b).item_weights).is_null() {
        free((*b).item_weights as *mut std::ffi::c_void);
    }
    if !((*b).sum_weights).is_null() {
        free((*b).sum_weights as *mut std::ffi::c_void);
    }
    if !((*b).h.items).is_null() {
        free((*b).h.items as *mut std::ffi::c_void);
    }
    if !b.is_null() {
        free(b as *mut std::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_tree(mut b: *mut crush_bucket_tree) {
    if !((*b).h.items).is_null() {
        free((*b).h.items as *mut std::ffi::c_void);
    }
    if !((*b).node_weights).is_null() {
        free((*b).node_weights as *mut std::ffi::c_void);
    }
    if !b.is_null() {
        free(b as *mut std::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_straw(mut b: *mut crush_bucket_straw) {
    if !((*b).straws).is_null() {
        free((*b).straws as *mut std::ffi::c_void);
    }
    if !((*b).item_weights).is_null() {
        free((*b).item_weights as *mut std::ffi::c_void);
    }
    if !((*b).h.items).is_null() {
        free((*b).h.items as *mut std::ffi::c_void);
    }
    if !b.is_null() {
        free(b as *mut std::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_straw2(mut b: *mut crush_bucket_straw2) {
    if !((*b).item_weights).is_null() {
        free((*b).item_weights as *mut std::ffi::c_void);
    }
    if !((*b).h.items).is_null() {
        free((*b).h.items as *mut std::ffi::c_void);
    }
    if !b.is_null() {
        free(b as *mut std::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket(mut b: *mut crush_bucket) {
    match (*b).alg as std::ffi::c_int {
        1 => {
            crush_destroy_bucket_uniform(b as *mut crush_bucket_uniform);
        }
        2 => {
            crush_destroy_bucket_list(b as *mut crush_bucket_list);
        }
        3 => {
            crush_destroy_bucket_tree(b as *mut crush_bucket_tree);
        }
        4 => {
            crush_destroy_bucket_straw(b as *mut crush_bucket_straw);
        }
        5 => {
            crush_destroy_bucket_straw2(b as *mut crush_bucket_straw2);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy(mut map: *mut crush_map) {
    if !((*map).buckets).is_null() {
        let mut b: __s32 = 0;
        b = 0 as std::ffi::c_int;
        while b < (*map).max_buckets {
            if !(*((*map).buckets).offset(b as isize)).is_null() {
                crush_destroy_bucket(*((*map).buckets).offset(b as isize));
            }
            b += 1;
            b;
        }
        if !((*map).buckets).is_null() {
            free((*map).buckets as *mut std::ffi::c_void);
        }
    }
    if !((*map).rules).is_null() {
        let mut b_0: __u32 = 0;
        b_0 = 0 as std::ffi::c_int as __u32;
        while b_0 < (*map).max_rules {
            crush_destroy_rule(*((*map).rules).offset(b_0 as isize));
            b_0 = b_0.wrapping_add(1);
            b_0;
        }
        if !((*map).rules).is_null() {
            free((*map).rules as *mut std::ffi::c_void);
        }
    }
    if !((*map).choose_tries).is_null() {
        free((*map).choose_tries as *mut std::ffi::c_void);
    }
    if !map.is_null() {
        free(map as *mut std::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_rule(mut rule: *mut crush_rule) {
    if !rule.is_null() {
        free(rule as *mut std::ffi::c_void);
    }
}
