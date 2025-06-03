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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn crush_find_roots(
    mut map: *mut crush_map,
    mut buckets: *mut *mut libc::c_int,
) -> libc::c_int {
    let vla = (*map).max_buckets as usize;
    let mut ref_0: Vec<libc::c_int> = ::std::vec::from_elem(0, vla);
    let mut root_count: libc::c_int = (*map).max_buckets;
    let mut pos: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    memset(
        ref_0.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        (vla * ::core::mem::size_of::<libc::c_int>()) as libc::c_ulong,
    );
    pos = 0 as libc::c_int;
    while pos < (*map).max_buckets {
        let mut b: *mut crush_bucket = *((*map).buckets).offset(pos as isize);
        if b.is_null() {
            root_count -= 1;
            root_count;
        } else {
            i = 0 as libc::c_int;
            while (i as __u32) < (*b).size {
                if !(*((*b).items).offset(i as isize) >= 0 as libc::c_int) {
                    let mut item: libc::c_int =
                        -(1 as libc::c_int) - *((*b).items).offset(i as isize);
                    if item >= (*map).max_buckets {
                        return -(22 as libc::c_int);
                    }
                    if *ref_0.as_mut_ptr().offset(item as isize) == 0 as libc::c_int {
                        root_count -= 1;
                        root_count;
                    }
                    let ref mut fresh0 = *ref_0.as_mut_ptr().offset(item as isize);
                    *fresh0 += 1;
                    *fresh0;
                }
                i += 1;
                i;
            }
        }
        pos += 1;
        pos;
    }
    let mut roots: *mut libc::c_int = malloc(
        (root_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if roots.is_null() {
        return -(12 as libc::c_int);
    }
    let mut roots_length: libc::c_int = 0 as libc::c_int;
    pos = 0 as libc::c_int;
    while pos < (*map).max_buckets {
        if !(*((*map).buckets).offset(pos as isize)).is_null()
            && *ref_0.as_mut_ptr().offset(pos as isize) == 0 as libc::c_int
        {
            let fresh1 = roots_length;
            roots_length = roots_length + 1;
            *roots.offset(fresh1 as isize) = -(1 as libc::c_int) - pos;
        }
        pos += 1;
        pos;
    }
    if roots_length == root_count {
    } else {
        __assert_fail(
            b"roots_length == root_count\0" as *const u8 as *const libc::c_char,
            b"/home/sevki/src/libcrush/crush/helpers.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                b"int crush_find_roots(struct crush_map *, int **)\0",
            ))
            .as_ptr(),
        );
    }
    'c_2267: {
        if roots_length == root_count {
        } else {
            __assert_fail(
                b"roots_length == root_count\0" as *const u8 as *const libc::c_char,
                b"/home/sevki/src/libcrush/crush/helpers.c\0" as *const u8 as *const libc::c_char,
                38 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"int crush_find_roots(struct crush_map *, int **)\0",
                ))
                .as_ptr(),
            );
        }
    };
    *buckets = roots;
    return root_count;
}
