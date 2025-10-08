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
use ::libc;

extern "C" {
    fn free(_: *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn crush_calc_tree_node(mut i: libc::c_int) -> libc::c_int {
    ((i + 1 as libc::c_int) << 1 as libc::c_int) - 1 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_bucket_alg_name(mut alg: libc::c_int) -> *const libc::c_char {
    match alg {
        1 => b"uniform\0" as *const u8 as *const libc::c_char,
        2 => b"list\0" as *const u8 as *const libc::c_char,
        3 => b"tree\0" as *const u8 as *const libc::c_char,
        4 => b"straw\0" as *const u8 as *const libc::c_char,
        5 => b"straw2\0" as *const u8 as *const libc::c_char,
        _ => b"unknown\0" as *const u8 as *const libc::c_char,
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_get_bucket_item_weight(
    mut b: *const crush_bucket,
    mut p: libc::c_int,
) -> libc::c_int {
    if p as __u32 >= (*b).size {
        return 0 as libc::c_int;
    }
    match (*b).alg as libc::c_int {
        1 => return (*(b as *mut crush_bucket_uniform)).item_weight as libc::c_int,
        2 => {
            return *((*(b as *mut crush_bucket_list)).item_weights).offset(p as isize)
                as libc::c_int;
        }
        3 => {
            return *((*(b as *mut crush_bucket_tree)).node_weights)
                .offset(crush_calc_tree_node(p) as isize) as libc::c_int;
        }
        4 => {
            return *((*(b as *mut crush_bucket_straw)).item_weights).offset(p as isize)
                as libc::c_int;
        }
        5 => {
            return *((*(b as *mut crush_bucket_straw2)).item_weights).offset(p as isize)
                as libc::c_int;
        }
        _ => {}
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_uniform(mut b: *mut crush_bucket_uniform) {
    if !((*b).h.items).is_null() {
        free((*b).h.items as *mut libc::c_void);
    }
    if !b.is_null() {
        free(b as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_list(mut b: *mut crush_bucket_list) {
    if !((*b).item_weights).is_null() {
        free((*b).item_weights as *mut libc::c_void);
    }
    if !((*b).sum_weights).is_null() {
        free((*b).sum_weights as *mut libc::c_void);
    }
    if !((*b).h.items).is_null() {
        free((*b).h.items as *mut libc::c_void);
    }
    if !b.is_null() {
        free(b as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_tree(mut b: *mut crush_bucket_tree) {
    if !((*b).h.items).is_null() {
        free((*b).h.items as *mut libc::c_void);
    }
    if !((*b).node_weights).is_null() {
        free((*b).node_weights as *mut libc::c_void);
    }
    if !b.is_null() {
        free(b as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_straw(mut b: *mut crush_bucket_straw) {
    if !((*b).straws).is_null() {
        free((*b).straws as *mut libc::c_void);
    }
    if !((*b).item_weights).is_null() {
        free((*b).item_weights as *mut libc::c_void);
    }
    if !((*b).h.items).is_null() {
        free((*b).h.items as *mut libc::c_void);
    }
    if !b.is_null() {
        free(b as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_straw2(mut b: *mut crush_bucket_straw2) {
    if !((*b).item_weights).is_null() {
        free((*b).item_weights as *mut libc::c_void);
    }
    if !((*b).h.items).is_null() {
        free((*b).h.items as *mut libc::c_void);
    }
    if !b.is_null() {
        free(b as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket(mut b: *mut crush_bucket) {
    match (*b).alg as libc::c_int {
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
        b = 0 as libc::c_int;
        while b < (*map).max_buckets {
            if !(*((*map).buckets).offset(b as isize)).is_null() {
                crush_destroy_bucket(*((*map).buckets).offset(b as isize));
            }
            b += 1;
            b;
        }
        if !((*map).buckets).is_null() {
            free((*map).buckets as *mut libc::c_void);
        }
    }
    if !((*map).rules).is_null() {
        let mut b_0: __u32 = 0;
        b_0 = 0 as libc::c_int as __u32;
        while b_0 < (*map).max_rules {
            crush_destroy_rule(*((*map).rules).offset(b_0 as isize));
            b_0 = b_0.wrapping_add(1);
            b_0;
        }
        if !((*map).rules).is_null() {
            free((*map).rules as *mut libc::c_void);
        }
    }
    if !((*map).choose_tries).is_null() {
        free((*map).choose_tries as *mut libc::c_void);
    }
    if !map.is_null() {
        free(map as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_rule(mut rule: *mut crush_rule) {
    if !rule.is_null() {
        free(rule as *mut libc::c_void);
    }
}
