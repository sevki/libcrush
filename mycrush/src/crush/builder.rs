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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn crush_destroy_bucket(b: *mut crush_bucket);
}
#[inline]
unsafe extern "C" fn crush_calc_tree_node(mut i: libc::c_int) -> libc::c_int {
    ((i + 1 as libc::c_int) << 1 as libc::c_int) - 1 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_create() -> *mut crush_map {
    let mut m: *mut crush_map = std::ptr::null_mut::<crush_map>();
    m = malloc(::core::mem::size_of::<crush_map>() as libc::c_ulong) as *mut crush_map;
    if m.is_null() {
        return std::ptr::null_mut::<crush_map>();
    }
    memset(
        m as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<crush_map>() as libc::c_ulong,
    );
    set_optimal_crush_map(m);
    m
}
#[no_mangle]
pub unsafe extern "C" fn crush_finalize(mut map: *mut crush_map) {
    let mut b: libc::c_int = 0;
    let mut i: __u32 = 0;
    (*map).working_size = ::core::mem::size_of::<crush_work>() as libc::c_ulong;
    (*map).working_size = ((*map).working_size as libc::c_ulong).wrapping_add(
        ((*map).max_buckets as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut crush_work_bucket>() as libc::c_ulong),
    ) as size_t as size_t;
    (*map).max_devices = 0 as libc::c_int;
    b = 0 as libc::c_int;
    while b < (*map).max_buckets {
        if !(*((*map).buckets).offset(b as isize)).is_null() {
            i = 0 as libc::c_int as __u32;
            while i < (**((*map).buckets).offset(b as isize)).size {
                if *((**((*map).buckets).offset(b as isize)).items).offset(i as isize)
                    >= (*map).max_devices
                {
                    (*map).max_devices = *((**((*map).buckets).offset(b as isize)).items)
                        .offset(i as isize)
                        + 1 as libc::c_int;
                }
                i = i.wrapping_add(1);
                i;
            }
            match (**((*map).buckets).offset(b as isize)).alg as libc::c_int {
                _ => {}
            }
            (*map).working_size = ((*map).working_size as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<crush_work_bucket>() as libc::c_ulong)
                as size_t as size_t;
            (*map).working_size = ((*map).working_size as libc::c_ulong).wrapping_add(
                ((**((*map).buckets).offset(b as isize)).size as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<__u32>() as libc::c_ulong),
            ) as size_t as size_t;
        }
        b += 1;
        b;
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_add_rule(
    mut map: *mut crush_map,
    mut rule: *mut crush_rule,
    mut ruleno: libc::c_int,
) -> libc::c_int {
    let mut r: __u32 = 0;
    if ruleno < 0 as libc::c_int {
        r = 0 as libc::c_int as __u32;
        while r < (*map).max_rules {
            if (*((*map).rules).offset(r as isize)).is_null() {
                break;
            }
            r = r.wrapping_add(1);
            r;
        }
        if r < ((1 as libc::c_int) << 8 as libc::c_int) as __u32 {
        } else {
            __assert_fail(
                b"r < CRUSH_MAX_RULES\0" as *const u8 as *const libc::c_char,
                b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"int crush_add_rule(struct crush_map *, struct crush_rule *, int)\0",
                ))
                .as_ptr(),
            );
        }
        'c_4117: {
            if r < ((1 as libc::c_int) << 8 as libc::c_int) as __u32 {
            } else {
                __assert_fail(
                    b"r < CRUSH_MAX_RULES\0" as *const u8 as *const libc::c_char,
                    b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8
                        as *const libc::c_char,
                    78 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                        b"int crush_add_rule(struct crush_map *, struct crush_rule *, int)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
    } else {
        r = ruleno as __u32;
    }
    if r >= (*map).max_rules {
        let mut oldsize: libc::c_int = 0;
        let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
        if ((*map).max_rules).wrapping_add(1 as libc::c_int as __u32)
            > ((1 as libc::c_int) << 8 as libc::c_int) as __u32
        {
            return -(28 as libc::c_int);
        }
        oldsize = (*map).max_rules as libc::c_int;
        (*map).max_rules = r.wrapping_add(1 as libc::c_int as __u32);
        _realloc = realloc(
            (*map).rules as *mut libc::c_void,
            ((*map).max_rules as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut crush_rule>() as libc::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as libc::c_int);
        } else {
            (*map).rules = _realloc as *mut *mut crush_rule;
        }
        memset(
            ((*map).rules).offset(oldsize as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (((*map).max_rules).wrapping_sub(oldsize as __u32) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut crush_rule>() as libc::c_ulong),
        );
    }
    let fresh0 = &mut (*((*map).rules).offset(r as isize));
    *fresh0 = rule;
    r as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_make_rule(
    mut len: libc::c_int,
    mut ruleset: libc::c_int,
    mut type_0: libc::c_int,
    mut minsize: libc::c_int,
    mut maxsize: libc::c_int,
) -> *mut crush_rule {
    let mut rule: *mut crush_rule = std::ptr::null_mut::<crush_rule>();
    rule = malloc(
        (::core::mem::size_of::<crush_rule>() as libc::c_ulong).wrapping_add(
            (len as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<crush_rule_step>() as libc::c_ulong),
        ),
    ) as *mut crush_rule;
    if rule.is_null() {
        return std::ptr::null_mut::<crush_rule>();
    }
    (*rule).len = len as __u32;
    (*rule).mask.ruleset = ruleset as __u8;
    (*rule).mask.type_0 = type_0 as __u8;
    (*rule).mask.min_size = minsize as __u8;
    (*rule).mask.max_size = maxsize as __u8;
    rule
}
#[no_mangle]
pub unsafe extern "C" fn crush_rule_set_step(
    mut rule: *mut crush_rule,
    mut n: libc::c_int,
    mut op: libc::c_int,
    mut arg1: libc::c_int,
    mut arg2: libc::c_int,
) {
    if (n as __u32) < (*rule).len {
    } else {
        __assert_fail(
            b"(__u32)n < rule->len\0" as *const u8 as *const libc::c_char,
            b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                b"void crush_rule_set_step(struct crush_rule *, int, int, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3903: {
        if (n as __u32) < (*rule).len {
        } else {
            __assert_fail(
                b"(__u32)n < rule->len\0" as *const u8 as *const libc::c_char,
                b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
                123 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                    b"void crush_rule_set_step(struct crush_rule *, int, int, int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    (*((*rule).steps).as_mut_ptr().offset(n as isize)).op = op as __u32;
    (*((*rule).steps).as_mut_ptr().offset(n as isize)).arg1 = arg1;
    (*((*rule).steps).as_mut_ptr().offset(n as isize)).arg2 = arg2;
}
#[no_mangle]
pub unsafe extern "C" fn crush_get_next_bucket_id(mut map: *mut crush_map) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    pos = 0 as libc::c_int;
    while pos < (*map).max_buckets {
        if (*((*map).buckets).offset(pos as isize)).is_null() {
            break;
        }
        pos += 1;
        pos;
    }
    -(1 as libc::c_int) - pos
}
#[no_mangle]
pub unsafe extern "C" fn crush_add_bucket(
    mut map: *mut crush_map,
    mut id: libc::c_int,
    mut bucket: *mut crush_bucket,
    mut idout: *mut libc::c_int,
) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    if id == 0 as libc::c_int {
        id = crush_get_next_bucket_id(map);
    }
    pos = -(1 as libc::c_int) - id;
    while pos >= (*map).max_buckets {
        let mut oldsize: libc::c_int = (*map).max_buckets;
        if (*map).max_buckets != 0 {
            (*map).max_buckets *= 2 as libc::c_int;
        } else {
            (*map).max_buckets = 8 as libc::c_int;
        }
        let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
        _realloc = realloc(
            (*map).buckets as *mut libc::c_void,
            ((*map).max_buckets as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut crush_bucket>() as libc::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as libc::c_int);
        } else {
            (*map).buckets = _realloc as *mut *mut crush_bucket;
        }
        memset(
            ((*map).buckets).offset(oldsize as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (((*map).max_buckets - oldsize) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut crush_bucket>() as libc::c_ulong),
        );
    }
    if !(*((*map).buckets).offset(pos as isize)).is_null() {
        return -(17 as libc::c_int);
    }
    (*bucket).id = id;
    let fresh1 = &mut (*((*map).buckets).offset(pos as isize));
    *fresh1 = bucket;
    if !idout.is_null() {
        *idout = id;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_remove_bucket(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket,
) -> libc::c_int {
    let mut pos: libc::c_int = -(1 as libc::c_int) - (*bucket).id;
    if pos < (*map).max_buckets {
    } else {
        __assert_fail(
            b"pos < map->max_buckets\0" as *const u8 as *const libc::c_char,
            b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 67], &[libc::c_char; 67]>(
                b"int crush_remove_bucket(struct crush_map *, struct crush_bucket *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_9808: {
        if pos < (*map).max_buckets {
        } else {
            __assert_fail(
                b"pos < map->max_buckets\0" as *const u8 as *const libc::c_char,
                b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
                184 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 67], &[libc::c_char; 67]>(
                    b"int crush_remove_bucket(struct crush_map *, struct crush_bucket *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    let fresh2 = &mut (*((*map).buckets).offset(pos as isize));
    *fresh2 = std::ptr::null_mut::<crush_bucket>();
    crush_destroy_bucket(bucket);
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_make_uniform_bucket(
    mut hash: libc::c_int,
    mut type_0: libc::c_int,
    mut size: libc::c_int,
    mut items: *mut libc::c_int,
    mut item_weight: libc::c_int,
) -> *mut crush_bucket_uniform {
    let mut i: libc::c_int = 0;
    let mut bucket: *mut crush_bucket_uniform = std::ptr::null_mut::<crush_bucket_uniform>();
    bucket = malloc(::core::mem::size_of::<crush_bucket_uniform>() as libc::c_ulong)
        as *mut crush_bucket_uniform;
    if bucket.is_null() {
        return std::ptr::null_mut::<crush_bucket_uniform>();
    }
    memset(
        bucket as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<crush_bucket_uniform>() as libc::c_ulong,
    );
    (*bucket).h.alg = CRUSH_BUCKET_UNIFORM as libc::c_int as __u8;
    (*bucket).h.hash = hash as __u8;
    (*bucket).h.type_0 = type_0 as __u16;
    (*bucket).h.size = size as __u32;
    if crush_multiplication_is_unsafe(size as __u32, item_weight as __u32) == 0 {
        (*bucket).h.weight = (size * item_weight) as __u32;
        (*bucket).item_weight = item_weight as __u32;
        (*bucket).h.items = malloc(
            (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
        ) as *mut __s32;
        if !((*bucket).h.items).is_null() {
            i = 0 as libc::c_int;
            while i < size {
                *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
                i += 1;
                i;
            }
            return bucket;
        }
    }
    free((*bucket).h.items as *mut libc::c_void);
    free(bucket as *mut libc::c_void);
    std::ptr::null_mut::<crush_bucket_uniform>()
}
#[no_mangle]
pub unsafe extern "C" fn crush_make_list_bucket(
    mut hash: libc::c_int,
    mut type_0: libc::c_int,
    mut size: libc::c_int,
    mut items: *mut libc::c_int,
    mut weights: *mut libc::c_int,
) -> *mut crush_bucket_list {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut bucket: *mut crush_bucket_list = std::ptr::null_mut::<crush_bucket_list>();
    bucket = malloc(::core::mem::size_of::<crush_bucket_list>() as libc::c_ulong)
        as *mut crush_bucket_list;
    if bucket.is_null() {
        return std::ptr::null_mut::<crush_bucket_list>();
    }
    memset(
        bucket as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<crush_bucket_list>() as libc::c_ulong,
    );
    (*bucket).h.alg = CRUSH_BUCKET_LIST as libc::c_int as __u8;
    (*bucket).h.hash = hash as __u8;
    (*bucket).h.type_0 = type_0 as __u16;
    (*bucket).h.size = size as __u32;
    (*bucket).h.items = malloc(
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
    ) as *mut __s32;
    if !((*bucket).h.items).is_null() {
        (*bucket).item_weights = malloc(
            (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
        ) as *mut __u32;
        if !((*bucket).item_weights).is_null() {
            (*bucket).sum_weights = malloc(
                (::core::mem::size_of::<__u32>() as libc::c_ulong)
                    .wrapping_mul(size as libc::c_ulong),
            ) as *mut __u32;
            if !((*bucket).sum_weights).is_null() {
                w = 0 as libc::c_int;
                i = 0 as libc::c_int;
                loop {
                    if i >= size {
                        current_block = 15652330335145281839;
                        break;
                    }
                    *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
                    *((*bucket).item_weights).offset(i as isize) =
                        *weights.offset(i as isize) as __u32;
                    if crush_addition_is_unsafe(w as __u32, *weights.offset(i as isize) as __u32)
                        != 0
                    {
                        current_block = 944831508617719848;
                        break;
                    }
                    w += *weights.offset(i as isize);
                    *((*bucket).sum_weights).offset(i as isize) = w as __u32;
                    i += 1;
                    i;
                }
                match current_block {
                    944831508617719848 => {}
                    _ => {
                        (*bucket).h.weight = w as __u32;
                        return bucket;
                    }
                }
            }
        }
    }
    free((*bucket).sum_weights as *mut libc::c_void);
    free((*bucket).item_weights as *mut libc::c_void);
    free((*bucket).h.items as *mut libc::c_void);
    free(bucket as *mut libc::c_void);
    std::ptr::null_mut::<crush_bucket_list>()
}
unsafe extern "C" fn height(mut n: libc::c_int) -> libc::c_int {
    let mut h: libc::c_int = 0 as libc::c_int;
    while n & 1 as libc::c_int == 0 as libc::c_int {
        h += 1;
        h;
        n >>= 1 as libc::c_int;
    }
    h
}
unsafe extern "C" fn on_right(mut n: libc::c_int, mut h: libc::c_int) -> libc::c_int {
    n & (1 as libc::c_int) << (h + 1 as libc::c_int)
}
unsafe extern "C" fn parent(mut n: libc::c_int) -> libc::c_int {
    let mut h: libc::c_int = height(n);
    if on_right(n, h) != 0 {
        n - ((1 as libc::c_int) << h)
    } else {
        n + ((1 as libc::c_int) << h)
    }
}
unsafe extern "C" fn calc_depth(mut size: libc::c_int) -> libc::c_int {
    if size == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut depth: libc::c_int = 1 as libc::c_int;
    let mut t: libc::c_int = size - 1 as libc::c_int;
    while t != 0 {
        t >>= 1 as libc::c_int;
        depth += 1;
        depth;
    }
    depth
}
#[no_mangle]
pub unsafe extern "C" fn crush_make_tree_bucket(
    mut hash: libc::c_int,
    mut type_0: libc::c_int,
    mut size: libc::c_int,
    mut items: *mut libc::c_int,
    mut weights: *mut libc::c_int,
) -> *mut crush_bucket_tree {
    let mut current_block: u64;
    let mut bucket: *mut crush_bucket_tree = std::ptr::null_mut::<crush_bucket_tree>();
    let mut depth: libc::c_int = 0;
    let mut node: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    bucket = malloc(::core::mem::size_of::<crush_bucket_tree>() as libc::c_ulong)
        as *mut crush_bucket_tree;
    if bucket.is_null() {
        return std::ptr::null_mut::<crush_bucket_tree>();
    }
    memset(
        bucket as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<crush_bucket_tree>() as libc::c_ulong,
    );
    (*bucket).h.alg = CRUSH_BUCKET_TREE as libc::c_int as __u8;
    (*bucket).h.hash = hash as __u8;
    (*bucket).h.type_0 = type_0 as __u16;
    (*bucket).h.size = size as __u32;
    if size == 0 as libc::c_int {
        (*bucket).h.items = std::ptr::null_mut::<__s32>();
        (*bucket).h.weight = 0 as libc::c_int as __u32;
        (*bucket).node_weights = std::ptr::null_mut::<__u32>();
        (*bucket).num_nodes = 0 as libc::c_int as __u8;
        return bucket;
    }
    (*bucket).h.items = malloc(
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
    ) as *mut __s32;
    if !((*bucket).h.items).is_null() {
        depth = calc_depth(size);
        (*bucket).num_nodes = ((1 as libc::c_int) << depth) as __u8;
        (*bucket).node_weights = malloc(
            (::core::mem::size_of::<__u32>() as libc::c_ulong)
                .wrapping_mul((*bucket).num_nodes as libc::c_ulong),
        ) as *mut __u32;
        if !((*bucket).node_weights).is_null() {
            memset(
                (*bucket).h.items as *mut libc::c_void,
                0 as libc::c_int,
                (::core::mem::size_of::<__s32>() as libc::c_ulong)
                    .wrapping_mul((*bucket).h.size as libc::c_ulong),
            );
            memset(
                (*bucket).node_weights as *mut libc::c_void,
                0 as libc::c_int,
                (::core::mem::size_of::<__u32>() as libc::c_ulong)
                    .wrapping_mul((*bucket).num_nodes as libc::c_ulong),
            );
            i = 0 as libc::c_int;
            's_88: loop {
                if i >= size {
                    current_block = 16924917904204750491;
                    break;
                }
                *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
                node = crush_calc_tree_node(i);
                *((*bucket).node_weights).offset(node as isize) =
                    *weights.offset(i as isize) as __u32;
                if crush_addition_is_unsafe(
                    (*bucket).h.weight,
                    *weights.offset(i as isize) as __u32,
                ) != 0
                {
                    current_block = 1061975787736880768;
                    break;
                }
                (*bucket).h.weight =
                    ((*bucket).h.weight).wrapping_add(*weights.offset(i as isize) as __u32);
                j = 1 as libc::c_int;
                while j < depth {
                    node = parent(node);
                    if crush_addition_is_unsafe(
                        *((*bucket).node_weights).offset(node as isize),
                        *weights.offset(i as isize) as __u32,
                    ) != 0
                    {
                        current_block = 1061975787736880768;
                        break 's_88;
                    }
                    let fresh3 = &mut (*((*bucket).node_weights).offset(node as isize));
                    *fresh3 = (*fresh3).wrapping_add(*weights.offset(i as isize) as __u32);
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            match current_block {
                1061975787736880768 => {}
                _ => {
                    if *((*bucket).node_weights)
                        .offset(((*bucket).num_nodes as libc::c_int / 2 as libc::c_int) as isize) == (*bucket).h.weight
                    {
                    } else {
                        __assert_fail(
                            b"!(bucket->node_weights[bucket->num_nodes/2] != bucket->h.weight)\0"
                                as *const u8 as *const libc::c_char,
                            b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8
                                as *const libc::c_char,
                            389 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 78],
                                &[libc::c_char; 78],
                            >(
                                b"struct crush_bucket_tree *crush_make_tree_bucket(int, int, int, int *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_5687: {
                        if *((*bucket).node_weights).offset(
                            ((*bucket).num_nodes as libc::c_int / 2 as libc::c_int) as isize,
                        ) == (*bucket).h.weight
                        {
                        } else {
                            __assert_fail(
                                b"!(bucket->node_weights[bucket->num_nodes/2] != bucket->h.weight)\0"
                                    as *const u8 as *const libc::c_char,
                                b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8
                                    as *const libc::c_char,
                                389 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 78],
                                    &[libc::c_char; 78],
                                >(
                                    b"struct crush_bucket_tree *crush_make_tree_bucket(int, int, int, int *, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    return bucket;
                }
            }
        }
    }
    free((*bucket).node_weights as *mut libc::c_void);
    free((*bucket).h.items as *mut libc::c_void);
    free(bucket as *mut libc::c_void);
    std::ptr::null_mut::<crush_bucket_tree>()
}
#[no_mangle]
pub unsafe extern "C" fn crush_calc_straw(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_straw,
) -> libc::c_int {
    let mut reverse: *mut libc::c_int = std::ptr::null_mut::<libc::c_int>();
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut straw: libc::c_double = 0.;
    let mut wbelow: libc::c_double = 0.;
    let mut lastw: libc::c_double = 0.;
    let mut wnext: libc::c_double = 0.;
    let mut pbelow: libc::c_double = 0.;
    let mut numleft: libc::c_int = 0;
    let mut size: libc::c_int = (*bucket).h.size as libc::c_int;
    let mut weights: *mut __u32 = (*bucket).item_weights;
    reverse = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(size as libc::c_ulong),
    ) as *mut libc::c_int;
    if reverse.is_null() {
        return -(12 as libc::c_int);
    }
    if size != 0 {
        *reverse.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < size {
        j = 0 as libc::c_int;
        while j < i {
            if *weights.offset(i as isize) < *weights.offset(*reverse.offset(j as isize) as isize) {
                k = i;
                while k > j {
                    *reverse.offset(k as isize) = *reverse.offset((k - 1 as libc::c_int) as isize);
                    k -= 1;
                    k;
                }
                *reverse.offset(j as isize) = i;
                break;
            } else {
                j += 1;
                j;
            }
        }
        if j == i {
            *reverse.offset(i as isize) = i;
        }
        i += 1;
        i;
    }
    numleft = size;
    straw = 1.0f64;
    wbelow = 0 as libc::c_int as libc::c_double;
    lastw = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < size {
        if (*map).straw_calc_version as libc::c_int == 0 as libc::c_int {
            if *weights.offset(*reverse.offset(i as isize) as isize) == 0 as libc::c_int as __u32 {
                *((*bucket).straws).offset(*reverse.offset(i as isize) as isize) =
                    0 as libc::c_int as __u32;
                i += 1;
                i;
            } else {
                *((*bucket).straws).offset(*reverse.offset(i as isize) as isize) =
                    (straw * 0x10000 as libc::c_int as libc::c_double) as __u32;
                i += 1;
                i;
                if i == size {
                    break;
                }
                if *weights.offset(*reverse.offset(i as isize) as isize)
                    == *weights.offset(*reverse.offset((i - 1 as libc::c_int) as isize) as isize)
                {
                    continue;
                }
                wbelow += (*weights
                    .offset(*reverse.offset((i - 1 as libc::c_int) as isize) as isize)
                    as libc::c_double
                    - lastw)
                    * numleft as libc::c_double;
                j = i;
                while j < size {
                    if *weights.offset(*reverse.offset(j as isize) as isize) != *weights.offset(*reverse.offset(i as isize) as isize)
                    {
                        break;
                    }
                    numleft -= 1;
                    numleft;
                    j += 1;
                    j;
                }
                wnext = (numleft as __u32
                    * (*weights.offset(*reverse.offset(i as isize) as isize)).wrapping_sub(
                        *weights.offset(*reverse.offset((i - 1 as libc::c_int) as isize) as isize),
                    )) as libc::c_double;
                pbelow = wbelow / (wbelow + wnext);
                straw *= pow(1.0f64 / pbelow, 1.0f64 / numleft as libc::c_double);
                lastw = *weights.offset(*reverse.offset((i - 1 as libc::c_int) as isize) as isize)
                    as libc::c_double;
            }
        } else {
            if ((*map).straw_calc_version as libc::c_int) < 1 as libc::c_int {
                continue;
            }
            if *weights.offset(*reverse.offset(i as isize) as isize) == 0 as libc::c_int as __u32 {
                *((*bucket).straws).offset(*reverse.offset(i as isize) as isize) =
                    0 as libc::c_int as __u32;
                i += 1;
                i;
                numleft -= 1;
                numleft;
            } else {
                *((*bucket).straws).offset(*reverse.offset(i as isize) as isize) =
                    (straw * 0x10000 as libc::c_int as libc::c_double) as __u32;
                i += 1;
                i;
                if i == size {
                    break;
                }
                wbelow += (*weights
                    .offset(*reverse.offset((i - 1 as libc::c_int) as isize) as isize)
                    as libc::c_double
                    - lastw)
                    * numleft as libc::c_double;
                numleft -= 1;
                numleft;
                wnext = (numleft as __u32
                    * (*weights.offset(*reverse.offset(i as isize) as isize)).wrapping_sub(
                        *weights.offset(*reverse.offset((i - 1 as libc::c_int) as isize) as isize),
                    )) as libc::c_double;
                pbelow = wbelow / (wbelow + wnext);
                straw *= pow(1.0f64 / pbelow, 1.0f64 / numleft as libc::c_double);
                lastw = *weights.offset(*reverse.offset((i - 1 as libc::c_int) as isize) as isize)
                    as libc::c_double;
            }
        }
    }
    free(reverse as *mut libc::c_void);
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_make_straw_bucket(
    mut map: *mut crush_map,
    mut hash: libc::c_int,
    mut type_0: libc::c_int,
    mut size: libc::c_int,
    mut items: *mut libc::c_int,
    mut weights: *mut libc::c_int,
) -> *mut crush_bucket_straw {
    let mut bucket: *mut crush_bucket_straw = std::ptr::null_mut::<crush_bucket_straw>();
    let mut i: libc::c_int = 0;
    bucket = malloc(::core::mem::size_of::<crush_bucket_straw>() as libc::c_ulong)
        as *mut crush_bucket_straw;
    if bucket.is_null() {
        return std::ptr::null_mut::<crush_bucket_straw>();
    }
    memset(
        bucket as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<crush_bucket_straw>() as libc::c_ulong,
    );
    (*bucket).h.alg = CRUSH_BUCKET_STRAW as libc::c_int as __u8;
    (*bucket).h.hash = hash as __u8;
    (*bucket).h.type_0 = type_0 as __u16;
    (*bucket).h.size = size as __u32;
    (*bucket).h.items = malloc(
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
    ) as *mut __s32;
    if !((*bucket).h.items).is_null() {
        (*bucket).item_weights = malloc(
            (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
        ) as *mut __u32;
        if !((*bucket).item_weights).is_null() {
            (*bucket).straws = malloc(
                (::core::mem::size_of::<__u32>() as libc::c_ulong)
                    .wrapping_mul(size as libc::c_ulong),
            ) as *mut __u32;
            if !((*bucket).straws).is_null() {
                (*bucket).h.weight = 0 as libc::c_int as __u32;
                i = 0 as libc::c_int;
                while i < size {
                    *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
                    (*bucket).h.weight =
                        ((*bucket).h.weight).wrapping_add(*weights.offset(i as isize) as __u32);
                    *((*bucket).item_weights).offset(i as isize) =
                        *weights.offset(i as isize) as __u32;
                    i += 1;
                    i;
                }
                if crush_calc_straw(map, bucket) >= 0 as libc::c_int {
                    return bucket;
                }
            }
        }
    }
    free((*bucket).straws as *mut libc::c_void);
    free((*bucket).item_weights as *mut libc::c_void);
    free((*bucket).h.items as *mut libc::c_void);
    free(bucket as *mut libc::c_void);
    std::ptr::null_mut::<crush_bucket_straw>()
}
#[no_mangle]
pub unsafe extern "C" fn crush_make_straw2_bucket(
    mut map: *mut crush_map,
    mut hash: libc::c_int,
    mut type_0: libc::c_int,
    mut size: libc::c_int,
    mut items: *mut libc::c_int,
    mut weights: *mut libc::c_int,
) -> *mut crush_bucket_straw2 {
    let mut bucket: *mut crush_bucket_straw2 = std::ptr::null_mut::<crush_bucket_straw2>();
    let mut i: libc::c_int = 0;
    bucket = malloc(::core::mem::size_of::<crush_bucket_straw2>() as libc::c_ulong)
        as *mut crush_bucket_straw2;
    if bucket.is_null() {
        return std::ptr::null_mut::<crush_bucket_straw2>();
    }
    memset(
        bucket as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<crush_bucket_straw2>() as libc::c_ulong,
    );
    (*bucket).h.alg = CRUSH_BUCKET_STRAW2 as libc::c_int as __u8;
    (*bucket).h.hash = hash as __u8;
    (*bucket).h.type_0 = type_0 as __u16;
    (*bucket).h.size = size as __u32;
    (*bucket).h.items = malloc(
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
    ) as *mut __s32;
    if !((*bucket).h.items).is_null() {
        (*bucket).item_weights = malloc(
            (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
        ) as *mut __u32;
        if !((*bucket).item_weights).is_null() {
            (*bucket).h.weight = 0 as libc::c_int as __u32;
            i = 0 as libc::c_int;
            while i < size {
                *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
                (*bucket).h.weight =
                    ((*bucket).h.weight).wrapping_add(*weights.offset(i as isize) as __u32);
                *((*bucket).item_weights).offset(i as isize) = *weights.offset(i as isize) as __u32;
                i += 1;
                i;
            }
            return bucket;
        }
    }
    free((*bucket).item_weights as *mut libc::c_void);
    free((*bucket).h.items as *mut libc::c_void);
    free(bucket as *mut libc::c_void);
    std::ptr::null_mut::<crush_bucket_straw2>()
}
#[no_mangle]
pub unsafe extern "C" fn crush_make_bucket(
    mut map: *mut crush_map,
    mut alg: libc::c_int,
    mut hash: libc::c_int,
    mut type_0: libc::c_int,
    mut size: libc::c_int,
    mut items: *mut libc::c_int,
    mut weights: *mut libc::c_int,
) -> *mut crush_bucket {
    let mut item_weight: libc::c_int = 0;
    match alg {
        1 => {
            if size != 0 && !weights.is_null() {
                item_weight = *weights.offset(0 as libc::c_int as isize);
            } else {
                item_weight = 0 as libc::c_int;
            }
            return crush_make_uniform_bucket(hash, type_0, size, items, item_weight)
                as *mut crush_bucket;
        }
        2 => {
            return crush_make_list_bucket(hash, type_0, size, items, weights) as *mut crush_bucket;
        }
        3 => {
            return crush_make_tree_bucket(hash, type_0, size, items, weights) as *mut crush_bucket;
        }
        4 => {
            return crush_make_straw_bucket(map, hash, type_0, size, items, weights)
                as *mut crush_bucket;
        }
        5 => {
            return crush_make_straw2_bucket(map, hash, type_0, size, items, weights)
                as *mut crush_bucket;
        }
        _ => {}
    }
    std::ptr::null_mut::<crush_bucket>()
}
#[no_mangle]
pub unsafe extern "C" fn crush_add_uniform_bucket_item(
    mut bucket: *mut crush_bucket_uniform,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    let mut newsize: libc::c_int =
        ((*bucket).h.size).wrapping_add(1 as libc::c_int as __u32) as libc::c_int;
    let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    if (*bucket).item_weight != weight as __u32 {
        return -(22 as libc::c_int);
    }
    _realloc = realloc(
        (*bucket).h.items as *mut libc::c_void,
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).h.items = _realloc as *mut __s32;
    }
    *((*bucket).h.items).offset((newsize - 1 as libc::c_int) as isize) = item;
    if crush_addition_is_unsafe((*bucket).h.weight, weight as __u32) != 0 {
        return -(34 as libc::c_int);
    }
    (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as __u32);
    (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
    (*bucket).h.size;
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_add_list_bucket_item(
    mut bucket: *mut crush_bucket_list,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    let mut newsize: libc::c_int =
        ((*bucket).h.size).wrapping_add(1 as libc::c_int as __u32) as libc::c_int;
    let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    _realloc = realloc(
        (*bucket).h.items as *mut libc::c_void,
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).h.items = _realloc as *mut __s32;
    }
    _realloc = realloc(
        (*bucket).item_weights as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).item_weights = _realloc as *mut __u32;
    }
    _realloc = realloc(
        (*bucket).sum_weights as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).sum_weights = _realloc as *mut __u32;
    }
    *((*bucket).h.items).offset((newsize - 1 as libc::c_int) as isize) = item;
    *((*bucket).item_weights).offset((newsize - 1 as libc::c_int) as isize) = weight as __u32;
    if newsize > 1 as libc::c_int {
        if crush_addition_is_unsafe(
            *((*bucket).sum_weights).offset((newsize - 2 as libc::c_int) as isize),
            weight as __u32,
        ) != 0
        {
            return -(34 as libc::c_int);
        }
        *((*bucket).sum_weights).offset((newsize - 1 as libc::c_int) as isize) =
            (*((*bucket).sum_weights).offset((newsize - 2 as libc::c_int) as isize))
                .wrapping_add(weight as __u32);
    } else {
        *((*bucket).sum_weights).offset((newsize - 1 as libc::c_int) as isize) = weight as __u32;
    }
    (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as __u32);
    (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
    (*bucket).h.size;
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_add_tree_bucket_item(
    mut bucket: *mut crush_bucket_tree,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    let mut newsize: libc::c_int =
        ((*bucket).h.size).wrapping_add(1 as libc::c_int as __u32) as libc::c_int;
    let mut depth: libc::c_int = calc_depth(newsize);
    let mut node: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    (*bucket).num_nodes = ((1 as libc::c_int) << depth) as __u8;
    _realloc = realloc(
        (*bucket).h.items as *mut libc::c_void,
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).h.items = _realloc as *mut __s32;
    }
    _realloc = realloc(
        (*bucket).node_weights as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong)
            .wrapping_mul((*bucket).num_nodes as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).node_weights = _realloc as *mut __u32;
    }
    node = crush_calc_tree_node(newsize - 1 as libc::c_int);
    *((*bucket).node_weights).offset(node as isize) = weight as __u32;
    let mut root: libc::c_int = (*bucket).num_nodes as libc::c_int / 2 as libc::c_int;
    if depth >= 2 as libc::c_int && node - 1 as libc::c_int == root {
        *((*bucket).node_weights).offset(root as isize) =
            *((*bucket).node_weights).offset((root / 2 as libc::c_int) as isize);
    }
    j = 1 as libc::c_int;
    while j < depth {
        node = parent(node);
        if crush_addition_is_unsafe(
            *((*bucket).node_weights).offset(node as isize),
            weight as __u32,
        ) != 0
        {
            return -(34 as libc::c_int);
        }
        let fresh4 = &mut (*((*bucket).node_weights).offset(node as isize));
        *fresh4 = (*fresh4).wrapping_add(weight as __u32);
        j += 1;
        j;
    }
    if crush_addition_is_unsafe((*bucket).h.weight, weight as __u32) != 0 {
        return -(34 as libc::c_int);
    }
    *((*bucket).h.items).offset((newsize - 1 as libc::c_int) as isize) = item;
    (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as __u32);
    (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
    (*bucket).h.size;
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_add_straw_bucket_item(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_straw,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    let mut newsize: libc::c_int =
        ((*bucket).h.size).wrapping_add(1 as libc::c_int as __u32) as libc::c_int;
    let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    _realloc = realloc(
        (*bucket).h.items as *mut libc::c_void,
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).h.items = _realloc as *mut __s32;
    }
    _realloc = realloc(
        (*bucket).item_weights as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).item_weights = _realloc as *mut __u32;
    }
    _realloc = realloc(
        (*bucket).straws as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).straws = _realloc as *mut __u32;
    }
    *((*bucket).h.items).offset((newsize - 1 as libc::c_int) as isize) = item;
    *((*bucket).item_weights).offset((newsize - 1 as libc::c_int) as isize) = weight as __u32;
    if crush_addition_is_unsafe((*bucket).h.weight, weight as __u32) != 0 {
        return -(34 as libc::c_int);
    }
    (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as __u32);
    (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
    (*bucket).h.size;
    crush_calc_straw(map, bucket)
}
#[no_mangle]
pub unsafe extern "C" fn crush_add_straw2_bucket_item(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_straw2,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    let mut newsize: libc::c_int =
        ((*bucket).h.size).wrapping_add(1 as libc::c_int as __u32) as libc::c_int;
    let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    _realloc = realloc(
        (*bucket).h.items as *mut libc::c_void,
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).h.items = _realloc as *mut __s32;
    }
    _realloc = realloc(
        (*bucket).item_weights as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).item_weights = _realloc as *mut __u32;
    }
    *((*bucket).h.items).offset((newsize - 1 as libc::c_int) as isize) = item;
    *((*bucket).item_weights).offset((newsize - 1 as libc::c_int) as isize) = weight as __u32;
    if crush_addition_is_unsafe((*bucket).h.weight, weight as __u32) != 0 {
        return -(34 as libc::c_int);
    }
    (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as __u32);
    (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
    (*bucket).h.size;
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_bucket_add_item(
    mut map: *mut crush_map,
    mut b: *mut crush_bucket,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    match (*b).alg as libc::c_int {
        1 => {
            crush_add_uniform_bucket_item(b as *mut crush_bucket_uniform, item, weight)
        }
        2 => crush_add_list_bucket_item(b as *mut crush_bucket_list, item, weight),
        3 => crush_add_tree_bucket_item(b as *mut crush_bucket_tree, item, weight),
        4 => {
            crush_add_straw_bucket_item(map, b as *mut crush_bucket_straw, item, weight)
        }
        5 => {
            crush_add_straw2_bucket_item(map, b as *mut crush_bucket_straw2, item, weight)
        }
        _ => -(1 as libc::c_int),
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_remove_uniform_bucket_item(
    mut bucket: *mut crush_bucket_uniform,
    mut item: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut newsize: libc::c_int = 0;
    let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        if *((*bucket).h.items).offset(i as isize) == item {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i == (*bucket).h.size {
        return -(2 as libc::c_int);
    }
    j = i;
    while j < (*bucket).h.size {
        *((*bucket).h.items).offset(j as isize) =
            *((*bucket).h.items).offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        j = j.wrapping_add(1);
        j;
    }
    (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
    newsize = (*bucket).h.size as libc::c_int;
    if (*bucket).item_weight < (*bucket).h.weight {
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_sub((*bucket).item_weight);
    } else {
        (*bucket).h.weight = 0 as libc::c_int as __u32;
    }
    _realloc = realloc(
        (*bucket).h.items as *mut libc::c_void,
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).h.items = _realloc as *mut __s32;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_remove_list_bucket_item(
    mut bucket: *mut crush_bucket_list,
    mut item: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut newsize: libc::c_int = 0;
    let mut weight: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        if *((*bucket).h.items).offset(i as isize) == item {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i == (*bucket).h.size {
        return -(2 as libc::c_int);
    }
    weight = *((*bucket).item_weights).offset(i as isize);
    j = i;
    while j < (*bucket).h.size {
        *((*bucket).h.items).offset(j as isize) =
            *((*bucket).h.items).offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        *((*bucket).item_weights).offset(j as isize) = *((*bucket).item_weights)
            .offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        *((*bucket).sum_weights).offset(j as isize) = (*((*bucket).sum_weights)
            .offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize))
        .wrapping_sub(weight);
        j = j.wrapping_add(1);
        j;
    }
    if weight < (*bucket).h.weight {
        (*bucket).h.weight =
            ((*bucket).h.weight as libc::c_uint).wrapping_sub(weight) as __u32 as __u32;
    } else {
        (*bucket).h.weight = 0 as libc::c_int as __u32;
    }
    (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
    newsize = (*bucket).h.size as libc::c_int;
    let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    _realloc = realloc(
        (*bucket).h.items as *mut libc::c_void,
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).h.items = _realloc as *mut __s32;
    }
    _realloc = realloc(
        (*bucket).item_weights as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).item_weights = _realloc as *mut __u32;
    }
    _realloc = realloc(
        (*bucket).sum_weights as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).sum_weights = _realloc as *mut __u32;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_remove_tree_bucket_item(
    mut bucket: *mut crush_bucket_tree,
    mut item: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut newsize: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        let mut node: libc::c_int = 0;
        let mut weight: libc::c_uint = 0;
        let mut j: libc::c_int = 0;
        let mut depth: libc::c_int = calc_depth((*bucket).h.size as libc::c_int);
        if *((*bucket).h.items).offset(i as isize) != item {
            i = i.wrapping_add(1);
            i;
        } else {
            *((*bucket).h.items).offset(i as isize) = 0 as libc::c_int;
            node = crush_calc_tree_node(i as libc::c_int);
            weight = *((*bucket).node_weights).offset(node as isize);
            *((*bucket).node_weights).offset(node as isize) = 0 as libc::c_int as __u32;
            j = 1 as libc::c_int;
            while j < depth {
                node = parent(node);
                let fresh5 = &mut (*((*bucket).node_weights).offset(node as isize));
                *fresh5 = (*fresh5 as libc::c_uint).wrapping_sub(weight) as __u32 as __u32;
                j += 1;
                j;
            }
            if weight < (*bucket).h.weight {
                (*bucket).h.weight =
                    ((*bucket).h.weight as libc::c_uint).wrapping_sub(weight) as __u32 as __u32;
            } else {
                (*bucket).h.weight = 0 as libc::c_int as __u32;
            }
            break;
        }
    }
    if i == (*bucket).h.size {
        return -(2 as libc::c_int);
    }
    newsize = (*bucket).h.size;
    while newsize > 0 as libc::c_int as libc::c_uint {
        let mut node_0: libc::c_int = crush_calc_tree_node(
            newsize.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
        );
        if *((*bucket).node_weights).offset(node_0 as isize) != 0 {
            break;
        }
        newsize = newsize.wrapping_sub(1);
        newsize;
    }
    if newsize != (*bucket).h.size {
        let mut olddepth: libc::c_int = 0;
        let mut newdepth: libc::c_int = 0;
        let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
        _realloc = realloc(
            (*bucket).h.items as *mut libc::c_void,
            (::core::mem::size_of::<__s32>() as libc::c_ulong)
                .wrapping_mul(newsize as libc::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as libc::c_int);
        } else {
            (*bucket).h.items = _realloc as *mut __s32;
        }
        olddepth = calc_depth((*bucket).h.size as libc::c_int);
        newdepth = calc_depth(newsize as libc::c_int);
        if olddepth != newdepth {
            (*bucket).num_nodes = ((1 as libc::c_int) << newdepth) as __u8;
            _realloc = realloc(
                (*bucket).node_weights as *mut libc::c_void,
                (::core::mem::size_of::<__u32>() as libc::c_ulong)
                    .wrapping_mul((*bucket).num_nodes as libc::c_ulong),
            );
            if _realloc.is_null() {
                return -(12 as libc::c_int);
            } else {
                (*bucket).node_weights = _realloc as *mut __u32;
            }
        }
        (*bucket).h.size = newsize;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_remove_straw_bucket_item(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_straw,
    mut item: libc::c_int,
) -> libc::c_int {
    let mut newsize: libc::c_int =
        ((*bucket).h.size).wrapping_sub(1 as libc::c_int as __u32) as libc::c_int;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        if *((*bucket).h.items).offset(i as isize) == item {
            (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
            (*bucket).h.size;
            if *((*bucket).item_weights).offset(i as isize) < (*bucket).h.weight {
                (*bucket).h.weight =
                    ((*bucket).h.weight).wrapping_sub(*((*bucket).item_weights).offset(i as isize));
            } else {
                (*bucket).h.weight = 0 as libc::c_int as __u32;
            }
            j = i;
            while j < (*bucket).h.size {
                *((*bucket).h.items).offset(j as isize) = *((*bucket).h.items)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
                *((*bucket).item_weights).offset(j as isize) = *((*bucket).item_weights)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
                j = j.wrapping_add(1);
                j;
            }
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    if i == (*bucket).h.size {
        return -(2 as libc::c_int);
    }
    let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    _realloc = realloc(
        (*bucket).h.items as *mut libc::c_void,
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).h.items = _realloc as *mut __s32;
    }
    _realloc = realloc(
        (*bucket).item_weights as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).item_weights = _realloc as *mut __u32;
    }
    _realloc = realloc(
        (*bucket).straws as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).straws = _realloc as *mut __u32;
    }
    crush_calc_straw(map, bucket)
}
#[no_mangle]
pub unsafe extern "C" fn crush_remove_straw2_bucket_item(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_straw2,
    mut item: libc::c_int,
) -> libc::c_int {
    let mut newsize: libc::c_int =
        ((*bucket).h.size).wrapping_sub(1 as libc::c_int as __u32) as libc::c_int;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        if *((*bucket).h.items).offset(i as isize) == item {
            (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
            (*bucket).h.size;
            if *((*bucket).item_weights).offset(i as isize) < (*bucket).h.weight {
                (*bucket).h.weight =
                    ((*bucket).h.weight).wrapping_sub(*((*bucket).item_weights).offset(i as isize));
            } else {
                (*bucket).h.weight = 0 as libc::c_int as __u32;
            }
            j = i;
            while j < (*bucket).h.size {
                *((*bucket).h.items).offset(j as isize) = *((*bucket).h.items)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
                *((*bucket).item_weights).offset(j as isize) = *((*bucket).item_weights)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
                j = j.wrapping_add(1);
                j;
            }
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    if i == (*bucket).h.size {
        return -(2 as libc::c_int);
    }
    let mut _realloc: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    _realloc = realloc(
        (*bucket).h.items as *mut libc::c_void,
        (::core::mem::size_of::<__s32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).h.items = _realloc as *mut __s32;
    }
    _realloc = realloc(
        (*bucket).item_weights as *mut libc::c_void,
        (::core::mem::size_of::<__u32>() as libc::c_ulong).wrapping_mul(newsize as libc::c_ulong),
    );
    if _realloc.is_null() {
        return -(12 as libc::c_int);
    } else {
        (*bucket).item_weights = _realloc as *mut __u32;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_bucket_remove_item(
    mut map: *mut crush_map,
    mut b: *mut crush_bucket,
    mut item: libc::c_int,
) -> libc::c_int {
    match (*b).alg as libc::c_int {
        1 => {
            crush_remove_uniform_bucket_item(b as *mut crush_bucket_uniform, item)
        }
        2 => crush_remove_list_bucket_item(b as *mut crush_bucket_list, item),
        3 => crush_remove_tree_bucket_item(b as *mut crush_bucket_tree, item),
        4 => {
            crush_remove_straw_bucket_item(map, b as *mut crush_bucket_straw, item)
        }
        5 => {
            crush_remove_straw2_bucket_item(map, b as *mut crush_bucket_straw2, item)
        }
        _ => -(1 as libc::c_int),
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_adjust_uniform_bucket_item_weight(
    mut bucket: *mut crush_bucket_uniform,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    let mut diff: libc::c_int =
        ((weight as __u32).wrapping_sub((*bucket).item_weight) * (*bucket).h.size) as libc::c_int;
    (*bucket).item_weight = weight as __u32;
    (*bucket).h.weight = (*bucket).item_weight * (*bucket).h.size;
    diff
}
#[no_mangle]
pub unsafe extern "C" fn crush_adjust_list_bucket_item_weight(
    mut bucket: *mut crush_bucket_list,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        if *((*bucket).h.items).offset(i as isize) == item {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i == (*bucket).h.size {
        return 0 as libc::c_int;
    }
    diff =
        (weight as __u32).wrapping_sub(*((*bucket).item_weights).offset(i as isize)) as libc::c_int;
    *((*bucket).item_weights).offset(i as isize) = weight as __u32;
    (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(diff as __u32);
    j = i;
    while j < (*bucket).h.size {
        let fresh6 = &mut (*((*bucket).sum_weights).offset(j as isize));
        *fresh6 = (*fresh6).wrapping_add(diff as __u32);
        j = j.wrapping_add(1);
        j;
    }
    diff
}
#[no_mangle]
pub unsafe extern "C" fn crush_adjust_tree_bucket_item_weight(
    mut bucket: *mut crush_bucket_tree,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    let mut node: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut depth: libc::c_uint = calc_depth((*bucket).h.size as libc::c_int) as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        if *((*bucket).h.items).offset(i as isize) == item {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i == (*bucket).h.size {
        return 0 as libc::c_int;
    }
    node = crush_calc_tree_node(i as libc::c_int);
    diff = (weight as __u32).wrapping_sub(*((*bucket).node_weights).offset(node as isize))
        as libc::c_int;
    *((*bucket).node_weights).offset(node as isize) = weight as __u32;
    (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(diff as __u32);
    j = 1 as libc::c_int as libc::c_uint;
    while j < depth {
        node = parent(node);
        let fresh7 = &mut (*((*bucket).node_weights).offset(node as isize));
        *fresh7 = (*fresh7).wrapping_add(diff as __u32);
        j = j.wrapping_add(1);
        j;
    }
    diff
}
#[no_mangle]
pub unsafe extern "C" fn crush_adjust_straw_bucket_item_weight(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_straw,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    let mut idx: libc::c_uint = 0;
    let mut diff: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    idx = 0 as libc::c_int as libc::c_uint;
    while idx < (*bucket).h.size {
        if *((*bucket).h.items).offset(idx as isize) == item {
            break;
        }
        idx = idx.wrapping_add(1);
        idx;
    }
    if idx == (*bucket).h.size {
        return 0 as libc::c_int;
    }
    diff = (weight as __u32).wrapping_sub(*((*bucket).item_weights).offset(idx as isize))
        as libc::c_int;
    *((*bucket).item_weights).offset(idx as isize) = weight as __u32;
    (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(diff as __u32);
    r = crush_calc_straw(map, bucket);
    if r < 0 as libc::c_int {
        return r;
    }
    diff
}
#[no_mangle]
pub unsafe extern "C" fn crush_adjust_straw2_bucket_item_weight(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_straw2,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    let mut idx: libc::c_uint = 0;
    let mut diff: libc::c_int = 0;
    idx = 0 as libc::c_int as libc::c_uint;
    while idx < (*bucket).h.size {
        if *((*bucket).h.items).offset(idx as isize) == item {
            break;
        }
        idx = idx.wrapping_add(1);
        idx;
    }
    if idx == (*bucket).h.size {
        return 0 as libc::c_int;
    }
    diff = (weight as __u32).wrapping_sub(*((*bucket).item_weights).offset(idx as isize))
        as libc::c_int;
    *((*bucket).item_weights).offset(idx as isize) = weight as __u32;
    (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(diff as __u32);
    diff
}
#[no_mangle]
pub unsafe extern "C" fn crush_bucket_adjust_item_weight(
    mut map: *mut crush_map,
    mut b: *mut crush_bucket,
    mut item: libc::c_int,
    mut weight: libc::c_int,
) -> libc::c_int {
    match (*b).alg as libc::c_int {
        1 => {
            crush_adjust_uniform_bucket_item_weight(
                b as *mut crush_bucket_uniform,
                item,
                weight,
            )
        }
        2 => {
            crush_adjust_list_bucket_item_weight(b as *mut crush_bucket_list, item, weight)
        }
        3 => {
            crush_adjust_tree_bucket_item_weight(b as *mut crush_bucket_tree, item, weight)
        }
        4 => {
            crush_adjust_straw_bucket_item_weight(
                map,
                b as *mut crush_bucket_straw,
                item,
                weight,
            )
        }
        5 => {
            crush_adjust_straw2_bucket_item_weight(
                map,
                b as *mut crush_bucket_straw2,
                item,
                weight,
            )
        }
        _ => -(1 as libc::c_int),
    }
}
unsafe extern "C" fn crush_reweight_uniform_bucket(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_uniform,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut sum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut leaves: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        let mut id: libc::c_int = *((*bucket).h.items).offset(i as isize);
        if id < 0 as libc::c_int {
            let mut c: *mut crush_bucket =
                *((*map).buckets).offset((-(1 as libc::c_int) - id) as isize);
            crush_reweight_bucket(map, c);
            if crush_addition_is_unsafe(sum, (*c).weight) != 0 {
                return -(34 as libc::c_int);
            }
            sum = sum.wrapping_add((*c).weight);
            n = n.wrapping_add(1);
            n;
        } else {
            leaves = leaves.wrapping_add(1);
            leaves;
        }
        i = i.wrapping_add(1);
        i;
    }
    if n > leaves {
        (*bucket).item_weight = sum.wrapping_div(n);
    }
    (*bucket).h.weight = (*bucket).item_weight * (*bucket).h.size;
    0 as libc::c_int
}
unsafe extern "C" fn crush_reweight_list_bucket(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_list,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    (*bucket).h.weight = 0 as libc::c_int as __u32;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        let mut id: libc::c_int = *((*bucket).h.items).offset(i as isize);
        if id < 0 as libc::c_int {
            let mut c: *mut crush_bucket =
                *((*map).buckets).offset((-(1 as libc::c_int) - id) as isize);
            crush_reweight_bucket(map, c);
            *((*bucket).item_weights).offset(i as isize) = (*c).weight;
        }
        if crush_addition_is_unsafe(
            (*bucket).h.weight,
            *((*bucket).item_weights).offset(i as isize),
        ) != 0
        {
            return -(34 as libc::c_int);
        }
        (*bucket).h.weight =
            ((*bucket).h.weight).wrapping_add(*((*bucket).item_weights).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    0 as libc::c_int
}
unsafe extern "C" fn crush_reweight_tree_bucket(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_tree,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    (*bucket).h.weight = 0 as libc::c_int as __u32;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        let mut node: libc::c_int = crush_calc_tree_node(i as libc::c_int);
        let mut id: libc::c_int = *((*bucket).h.items).offset(i as isize);
        if id < 0 as libc::c_int {
            let mut c: *mut crush_bucket =
                *((*map).buckets).offset((-(1 as libc::c_int) - id) as isize);
            crush_reweight_bucket(map, c);
            *((*bucket).node_weights).offset(node as isize) = (*c).weight;
        }
        if crush_addition_is_unsafe(
            (*bucket).h.weight,
            *((*bucket).node_weights).offset(node as isize),
        ) != 0
        {
            return -(34 as libc::c_int);
        }
        (*bucket).h.weight =
            ((*bucket).h.weight).wrapping_add(*((*bucket).node_weights).offset(node as isize));
        i = i.wrapping_add(1);
        i;
    }
    0 as libc::c_int
}
unsafe extern "C" fn crush_reweight_straw_bucket(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_straw,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    (*bucket).h.weight = 0 as libc::c_int as __u32;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        let mut id: libc::c_int = *((*bucket).h.items).offset(i as isize);
        if id < 0 as libc::c_int {
            let mut c: *mut crush_bucket =
                *((*map).buckets).offset((-(1 as libc::c_int) - id) as isize);
            crush_reweight_bucket(map, c);
            *((*bucket).item_weights).offset(i as isize) = (*c).weight;
        }
        if crush_addition_is_unsafe(
            (*bucket).h.weight,
            *((*bucket).item_weights).offset(i as isize),
        ) != 0
        {
            return -(34 as libc::c_int);
        }
        (*bucket).h.weight =
            ((*bucket).h.weight).wrapping_add(*((*bucket).item_weights).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    crush_calc_straw(map, bucket);
    0 as libc::c_int
}
unsafe extern "C" fn crush_reweight_straw2_bucket(
    mut map: *mut crush_map,
    mut bucket: *mut crush_bucket_straw2,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    (*bucket).h.weight = 0 as libc::c_int as __u32;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*bucket).h.size {
        let mut id: libc::c_int = *((*bucket).h.items).offset(i as isize);
        if id < 0 as libc::c_int {
            let mut c: *mut crush_bucket =
                *((*map).buckets).offset((-(1 as libc::c_int) - id) as isize);
            crush_reweight_bucket(map, c);
            *((*bucket).item_weights).offset(i as isize) = (*c).weight;
        }
        if crush_addition_is_unsafe(
            (*bucket).h.weight,
            *((*bucket).item_weights).offset(i as isize),
        ) != 0
        {
            return -(34 as libc::c_int);
        }
        (*bucket).h.weight =
            ((*bucket).h.weight).wrapping_add(*((*bucket).item_weights).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn crush_reweight_bucket(
    mut map: *mut crush_map,
    mut b: *mut crush_bucket,
) -> libc::c_int {
    match (*b).alg as libc::c_int {
        1 => crush_reweight_uniform_bucket(map, b as *mut crush_bucket_uniform),
        2 => crush_reweight_list_bucket(map, b as *mut crush_bucket_list),
        3 => crush_reweight_tree_bucket(map, b as *mut crush_bucket_tree),
        4 => crush_reweight_straw_bucket(map, b as *mut crush_bucket_straw),
        5 => crush_reweight_straw2_bucket(map, b as *mut crush_bucket_straw2),
        _ => -(1 as libc::c_int),
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_make_choose_args(
    mut map: *mut crush_map,
    mut num_positions: libc::c_int,
) -> *mut crush_choose_arg {
    let mut b: libc::c_int = 0;
    let mut sum_bucket_size: libc::c_int = 0 as libc::c_int;
    let mut bucket_count: libc::c_int = 0 as libc::c_int;
    b = 0 as libc::c_int;
    while b < (*map).max_buckets {
        if !(*((*map).buckets).offset(b as isize)).is_null() {
            sum_bucket_size = (sum_bucket_size as __u32)
                .wrapping_add((**((*map).buckets).offset(b as isize)).size)
                as libc::c_int as libc::c_int;
            bucket_count += 1;
            bucket_count;
        }
        b += 1;
        b;
    }
    let mut size: libc::c_int = (::core::mem::size_of::<crush_choose_arg>() as libc::c_ulong)
        .wrapping_mul((*map).max_buckets as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<crush_weight_set>() as libc::c_ulong)
                .wrapping_mul(bucket_count as libc::c_ulong)
                .wrapping_mul(num_positions as libc::c_ulong),
        )
        .wrapping_add(
            (::core::mem::size_of::<__u32>() as libc::c_ulong)
                .wrapping_mul(sum_bucket_size as libc::c_ulong)
                .wrapping_mul(num_positions as libc::c_ulong),
        )
        .wrapping_add(
            (::core::mem::size_of::<__u32>() as libc::c_ulong)
                .wrapping_mul(sum_bucket_size as libc::c_ulong),
        ) as libc::c_int;
    let mut space: *mut libc::c_char = malloc(size as libc::c_ulong) as *mut libc::c_char;
    let mut arg: *mut crush_choose_arg = space as *mut crush_choose_arg;
    let mut weight_set: *mut crush_weight_set =
        arg.offset((*map).max_buckets as isize) as *mut crush_weight_set;
    let mut weights: *mut __u32 =
        weight_set.offset((bucket_count * num_positions) as isize) as *mut __u32;
    let mut weight_set_ends: *mut libc::c_char = weights as *mut libc::c_char;
    let mut ids: *mut libc::c_int =
        weights.offset((sum_bucket_size * num_positions) as isize) as *mut libc::c_int;
    let mut weights_end: *mut libc::c_char = ids as *mut libc::c_char;
    let mut ids_end: *mut libc::c_char = ids.offset(sum_bucket_size as isize) as *mut libc::c_char;
    if space.offset(size as isize) == ids_end {
    } else {
        __assert_fail(
            b"!(space + size != ids_end)\0" as *const u8 as *const libc::c_char,
            b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
            1428 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                b"struct crush_choose_arg *crush_make_choose_args(struct crush_map *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_7274: {
        if space.offset(size as isize) == ids_end {
        } else {
            __assert_fail(
                b"!(space + size != ids_end)\0" as *const u8 as *const libc::c_char,
                b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
                1428 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                    b"struct crush_choose_arg *crush_make_choose_args(struct crush_map *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    b = 0 as libc::c_int;
    while b < (*map).max_buckets {
        if (*((*map).buckets).offset(b as isize)).is_null() {
            memset(
                &mut *arg.offset(b as isize) as *mut crush_choose_arg as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<crush_choose_arg>() as libc::c_ulong,
            );
        } else {
            let mut bucket: *mut crush_bucket_straw2 =
                *((*map).buckets).offset(b as isize) as *mut crush_bucket_straw2;
            let mut position: libc::c_int = 0;
            position = 0 as libc::c_int;
            while position < num_positions {
                memcpy(
                    weights as *mut libc::c_void,
                    (*bucket).item_weights as *const libc::c_void,
                    (::core::mem::size_of::<__u32>() as libc::c_ulong)
                        .wrapping_mul((*bucket).h.size as libc::c_ulong),
                );
                let fresh8 = &mut (*weight_set.offset(position as isize)).weights;
                *fresh8 = weights;
                (*weight_set.offset(position as isize)).size = (*bucket).h.size;
                weights = weights.offset((*bucket).h.size as isize);
                position += 1;
                position;
            }
            let fresh9 = &mut (*arg.offset(b as isize)).weight_set;
            *fresh9 = weight_set;
            (*arg.offset(b as isize)).weight_set_size = num_positions as __u32;
            weight_set = weight_set.offset(position as isize);
            memcpy(
                ids as *mut libc::c_void,
                (*bucket).h.items as *const libc::c_void,
                (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul((*bucket).h.size as libc::c_ulong),
            );
            let fresh10 = &mut (*arg.offset(b as isize)).ids;
            *fresh10 = ids;
            (*arg.offset(b as isize)).ids_size = (*bucket).h.size;
            ids = ids.offset((*bucket).h.size as isize);
        }
        b += 1;
        b;
    }
    if weight_set_ends == weight_set as *mut libc::c_char {
    } else {
        __assert_fail(
            b"!((char*)weight_set_ends != (char*)weight_set)\0" as *const u8 as *const libc::c_char,
            b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
            1453 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                b"struct crush_choose_arg *crush_make_choose_args(struct crush_map *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_7024: {
        if weight_set_ends == weight_set as *mut libc::c_char {
        } else {
            __assert_fail(
                b"!((char*)weight_set_ends != (char*)weight_set)\0" as *const u8
                    as *const libc::c_char,
                b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
                1453 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                    b"struct crush_choose_arg *crush_make_choose_args(struct crush_map *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if weights_end == weights as *mut libc::c_char {
    } else {
        __assert_fail(
            b"!((char*)weights_end != (char*)weights)\0" as *const u8 as *const libc::c_char,
            b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
            1454 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                b"struct crush_choose_arg *crush_make_choose_args(struct crush_map *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_6973: {
        if weights_end == weights as *mut libc::c_char {
        } else {
            __assert_fail(
                b"!((char*)weights_end != (char*)weights)\0" as *const u8 as *const libc::c_char,
                b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
                1454 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                    b"struct crush_choose_arg *crush_make_choose_args(struct crush_map *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if ids as *mut libc::c_char == ids_end {
    } else {
        __assert_fail(
            b"!((char*)ids != (char*)ids_end)\0" as *const u8 as *const libc::c_char,
            b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
            1455 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                b"struct crush_choose_arg *crush_make_choose_args(struct crush_map *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_6885: {
        if ids as *mut libc::c_char == ids_end {
        } else {
            __assert_fail(
                b"!((char*)ids != (char*)ids_end)\0" as *const u8 as *const libc::c_char,
                b"/home/sevki/src/libcrush/crush/builder.c\0" as *const u8 as *const libc::c_char,
                1455 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                    b"struct crush_choose_arg *crush_make_choose_args(struct crush_map *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    arg
}
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_choose_args(mut args: *mut crush_choose_arg) {
    free(args as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn crush_addition_is_unsafe(mut a: __u32, mut b: __u32) -> libc::c_int {
    if (-(1 as libc::c_int) as __u32).wrapping_sub(b) < a {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_multiplication_is_unsafe(mut a: __u32, mut b: __u32) -> libc::c_int {
    if a == 0 {
        return 0 as libc::c_int;
    }
    if b == 0 {
        return 1 as libc::c_int;
    }
    if -(1 as libc::c_int) as __u32 / b < a {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_legacy_crush_map(mut map: *mut crush_map) {
    (*map).choose_local_tries = 2 as libc::c_int as __u32;
    (*map).choose_local_fallback_tries = 5 as libc::c_int as __u32;
    (*map).choose_total_tries = 19 as libc::c_int as __u32;
    (*map).chooseleaf_descend_once = 0 as libc::c_int as __u32;
    (*map).chooseleaf_vary_r = 0 as libc::c_int as __u8;
    (*map).chooseleaf_stable = 0 as libc::c_int as __u8;
    (*map).straw_calc_version = 0 as libc::c_int as __u8;
    (*map).allowed_bucket_algs = ((1 as libc::c_int) << CRUSH_BUCKET_UNIFORM as libc::c_int
        | (1 as libc::c_int) << CRUSH_BUCKET_LIST as libc::c_int
        | (1 as libc::c_int) << CRUSH_BUCKET_STRAW as libc::c_int)
        as __u32;
}
#[no_mangle]
pub unsafe extern "C" fn set_optimal_crush_map(mut map: *mut crush_map) {
    (*map).choose_local_tries = 0 as libc::c_int as __u32;
    (*map).choose_local_fallback_tries = 0 as libc::c_int as __u32;
    (*map).choose_total_tries = 50 as libc::c_int as __u32;
    (*map).chooseleaf_descend_once = 1 as libc::c_int as __u32;
    (*map).chooseleaf_vary_r = 1 as libc::c_int as __u8;
    (*map).chooseleaf_stable = 1 as libc::c_int as __u8;
    (*map).allowed_bucket_algs = ((1 as libc::c_int) << CRUSH_BUCKET_UNIFORM as libc::c_int
        | (1 as libc::c_int) << CRUSH_BUCKET_LIST as libc::c_int
        | (1 as libc::c_int) << CRUSH_BUCKET_STRAW as libc::c_int
        | (1 as libc::c_int) << CRUSH_BUCKET_STRAW2 as libc::c_int)
        as __u32;
}
