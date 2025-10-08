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
unsafe extern "C" {
    fn malloc(_: ffi::c_ulong) -> *mut ffi::c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_find_roots(
    map: *mut CrushMap,
    buckets: *mut *mut ffi::c_int,
) -> ffi::c_int {
    let vla = (*map).max_buckets as usize;
    let mut ref_0: Vec<ffi::c_int> = vec![0; vla];
    let mut root_count: ffi::c_int = (*map).max_buckets;
    
    // Vec is already zeroed by vec![0; vla], no need for memset
    
    for pos in 0..(*map).max_buckets {
        let b: *mut CrushBucket = *((*map).buckets).offset(pos as isize);
        if b.is_null() {
            root_count -= 1;
        } else {
            for i in 0..(*b).size {
                if *((*b).items).offset(i as isize) < 0 {
                    let item: ffi::c_int = -1 - *((*b).items).offset(i as isize);
                    if item >= (*map).max_buckets {
                        return -22;
                    }
                    if ref_0[item as usize] == 0 {
                        root_count -= 1;
                    }
                    ref_0[item as usize] += 1;
                }
            }
        }
    }
    
    let roots: *mut ffi::c_int = malloc(
        (root_count as ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<ffi::c_int>() as ffi::c_ulong),
    ) as *mut ffi::c_int;
    if roots.is_null() {
        return -12;
    }
    
    let mut roots_length: ffi::c_int = 0;
    for pos in 0..(*map).max_buckets {
        if !(*((*map).buckets).offset(pos as isize)).is_null()
            && ref_0[pos as usize] == 0
        {
            let fresh1 = roots_length;
            roots_length += 1;
            *roots.offset(fresh1 as isize) = -1 - pos;
        }
    }
    
    if roots_length != root_count {
        panic!("Assertion failed: roots_length == root_count");
    }

    *buckets = roots;
    root_count
}
