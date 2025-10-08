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
    fn memcpy(_: *mut ffi::c_void, _: *const ffi::c_void, _: ffi::c_ulong) -> *mut ffi::c_void;
    fn memset(_: *mut ffi::c_void, _: ffi::c_int, _: ffi::c_ulong) -> *mut ffi::c_void;
    fn pow(_: ffi::c_double, _: ffi::c_double) -> ffi::c_double;
    fn malloc(_: ffi::c_ulong) -> *mut ffi::c_void;
    fn realloc(_: *mut ffi::c_void, _: ffi::c_ulong) -> *mut ffi::c_void;
    fn free(_: *mut ffi::c_void);
    fn crush_destroy_bucket(b: *mut CrushBucket);
}
#[inline]
fn crush_calc_tree_node(i: i32) -> i32 {
    ((i + 1) << 1) - 1
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_create() -> *mut CrushMap {
    unsafe {
        let mut m: *mut CrushMap = std::ptr::null_mut::<CrushMap>();
        m = malloc(::core::mem::size_of::<CrushMap>() as ffi::c_ulong) as *mut CrushMap;
        if m.is_null() {
            return std::ptr::null_mut::<CrushMap>();
        }
        memset(
            m as *mut ffi::c_void,
            0,
            ::core::mem::size_of::<CrushMap>() as ffi::c_ulong,
        );
        set_optimal_crush_map(m);
        m
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_finalize(mut map: *mut CrushMap) {
    unsafe {
        let mut b: ffi::c_int = 0;
        let mut i: U32 = 0;
        (*map).working_size = ::core::mem::size_of::<CrushWork>() as ffi::c_ulong;
        (*map).working_size = ((*map).working_size as ffi::c_ulong).wrapping_add(
            ((*map).max_buckets as ffi::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut CrushWorkBucket>() as ffi::c_ulong),
        ) as SizeT as SizeT;
        (*map).max_devices = 0;
        b = 0;
        while b < (*map).max_buckets {
            if !(*((*map).buckets).offset(b as isize)).is_null() {
                i = 0 as U32;
                while i < (**((*map).buckets).offset(b as isize)).size {
                    if *((**((*map).buckets).offset(b as isize)).items).offset(i as isize)
                        >= (*map).max_devices
                    {
                        (*map).max_devices = *((**((*map).buckets).offset(b as isize)).items)
                            .offset(i as isize)
                            + 1;
                    }
                    i = i.wrapping_add(1);
                }
                let _ = ((*map).buckets).offset(b as isize);
                (*map).working_size = ((*map).working_size as ffi::c_ulong)
                    .wrapping_add(::core::mem::size_of::<CrushWorkBucket>() as ffi::c_ulong)
                    as SizeT as SizeT;
                (*map).working_size = ((*map).working_size as ffi::c_ulong).wrapping_add(
                    ((**((*map).buckets).offset(b as isize)).size as ffi::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<U32>() as ffi::c_ulong),
                ) as SizeT as SizeT;
            }
            b += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_add_rule(
    mut map: *mut CrushMap,
    mut rule: *mut CrushRule,
    mut ruleno: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut r: U32 = 0;
        if ruleno < 0 {
            r = 0 as U32;
            while r < (*map).max_rules {
                if (*((*map).rules).offset(r as isize)).is_null() {
                    break;
                }
                r = r.wrapping_add(1);
            }
            if r < ((1) << 8 as ffi::c_int) as U32 {
            } else {
                panic!("Assertion failed: r({}) < CRUSH_MAX_RULES({})", r, (1 << 8));
            }
        } else {
            r = ruleno as U32;
        }
        if r >= (*map).max_rules {
            let mut oldsize: ffi::c_int = 0;
            let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
            if ((*map).max_rules).wrapping_add(1 as U32)
                > ((1) << 8 as ffi::c_int) as U32
            {
                return -(28 as ffi::c_int);
            }
            oldsize = (*map).max_rules as ffi::c_int;
            (*map).max_rules = r.wrapping_add(1 as U32);
            _realloc = realloc(
                (*map).rules as *mut ffi::c_void,
                ((*map).max_rules as ffi::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut CrushRule>() as ffi::c_ulong),
            );
            if _realloc.is_null() {
                return -(12 as ffi::c_int);
            } else {
                (*map).rules = _realloc as *mut *mut CrushRule;
            }
            memset(
                ((*map).rules).offset(oldsize as isize) as *mut ffi::c_void,
                0,
                (((*map).max_rules).wrapping_sub(oldsize as U32) as ffi::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut CrushRule>() as ffi::c_ulong),
            );
        }
        let fresh0 = &mut (*((*map).rules).offset(r as isize));
        *fresh0 = rule;
        r as ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_make_rule(
    mut len: ffi::c_int,
    mut ruleset: ffi::c_int,
    mut type_0: ffi::c_int,
    mut minsize: ffi::c_int,
    mut maxsize: ffi::c_int,
) -> *mut CrushRule {
    unsafe {
        let mut rule: *mut CrushRule = std::ptr::null_mut::<CrushRule>();
        rule = malloc(
            (::core::mem::size_of::<CrushRule>() as ffi::c_ulong).wrapping_add(
                (len as ffi::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<CrushRuleStep>() as ffi::c_ulong),
            ),
        ) as *mut CrushRule;
        if rule.is_null() {
            return std::ptr::null_mut::<CrushRule>();
        }
        (*rule).len = len as U32;
        (*rule).mask.ruleset = ruleset as U8;
        (*rule).mask.type_0 = type_0 as U8;
        (*rule).mask.min_size = minsize as U8;
        (*rule).mask.max_size = maxsize as U8;
        rule
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_rule_set_step(
    mut rule: *mut CrushRule,
    mut n: ffi::c_int,
    mut op: ffi::c_int,
    mut arg1: ffi::c_int,
    mut arg2: ffi::c_int,
) {
    unsafe {
        if (n as U32) < (*rule).len {
        } else {
            panic!("Assertion failed: n({}) < rule->len({})", n, (*rule).len);
        }

        (*((*rule).steps).as_mut_ptr().offset(n as isize)).op = op as U32;
        (*((*rule).steps).as_mut_ptr().offset(n as isize)).arg1 = arg1;
        (*((*rule).steps).as_mut_ptr().offset(n as isize)).arg2 = arg2;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_get_next_bucket_id(mut map: *mut CrushMap) -> ffi::c_int {
    unsafe {
        for pos in 0..(*map).max_buckets {
            if (*((*map).buckets).offset(pos as isize)).is_null() {
                return -1 - pos;
            }
        }
        -1 - (*map).max_buckets
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_add_bucket(
    mut map: *mut CrushMap,
    mut id: ffi::c_int,
    mut bucket: *mut CrushBucket,
    mut idout: *mut ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut pos: ffi::c_int = 0;
        if id == 0 {
            id = crush_get_next_bucket_id(map);
        }
        pos = (-1) - id;
        while pos >= (*map).max_buckets {
            let mut oldsize: ffi::c_int = (*map).max_buckets;
            if (*map).max_buckets != 0 {
                (*map).max_buckets *= 2 as ffi::c_int;
            } else {
                (*map).max_buckets = 8 as ffi::c_int;
            }
            let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
            _realloc = realloc(
                (*map).buckets as *mut ffi::c_void,
                ((*map).max_buckets as ffi::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut CrushBucket>() as ffi::c_ulong),
            );
            if _realloc.is_null() {
                return -(12 as ffi::c_int);
            } else {
                (*map).buckets = _realloc as *mut *mut CrushBucket;
            }
            memset(
                ((*map).buckets).offset(oldsize as isize) as *mut ffi::c_void,
                0,
                (((*map).max_buckets - oldsize) as ffi::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut CrushBucket>() as ffi::c_ulong),
            );
        }
        if !(*((*map).buckets).offset(pos as isize)).is_null() {
            return -(17 as ffi::c_int);
        }
        (*bucket).id = id;
        let fresh1 = &mut (*((*map).buckets).offset(pos as isize));
        *fresh1 = bucket;
        if !idout.is_null() {
            *idout = id;
        }
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_remove_bucket(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucket,
) -> ffi::c_int {
    unsafe {
        let mut pos: ffi::c_int = (-1) - (*bucket).id;
        if pos < (*map).max_buckets {
        } else {
            panic!("Assertion failed: pos({}) < map->max_buckets({})", pos, (*map).max_buckets);
        }
        let fresh2 = &mut (*((*map).buckets).offset(pos as isize));
        *fresh2 = std::ptr::null_mut::<CrushBucket>();
        crush_destroy_bucket(bucket);
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_make_uniform_bucket(
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut item_weight: ffi::c_int,
) -> *mut CrushBucketUniform {
    unsafe {
        let mut i: ffi::c_int = 0;
        let mut bucket: *mut CrushBucketUniform = std::ptr::null_mut::<CrushBucketUniform>();
        bucket = malloc(::core::mem::size_of::<CrushBucketUniform>() as ffi::c_ulong)
            as *mut CrushBucketUniform;
        if bucket.is_null() {
            return std::ptr::null_mut::<CrushBucketUniform>();
        }
        memset(
            bucket as *mut ffi::c_void,
            0,
            ::core::mem::size_of::<CrushBucketUniform>() as ffi::c_ulong,
        );
        (*bucket).h.alg = CRUSH_BUCKET_UNIFORM as ffi::c_int as U8;
        (*bucket).h.hash = hash as U8;
        (*bucket).h.type_0 = type_0 as U16;
        (*bucket).h.size = size as U32;
        if crush_multiplication_is_unsafe(size as U32, item_weight as U32) == 0 {
            (*bucket).h.weight = (size * item_weight) as U32;
            (*bucket).item_weight = item_weight as U32;
            (*bucket).h.items = malloc(
                (::core::mem::size_of::<S32>() as ffi::c_ulong)
                    .wrapping_mul(size as ffi::c_ulong),
            ) as *mut S32;
            if !((*bucket).h.items).is_null() {
                i = 0;
                while i < size {
                    *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
                    i += 1;
                }
                return bucket;
            }
        }
        free((*bucket).h.items as *mut ffi::c_void);
        free(bucket as *mut ffi::c_void);
        std::ptr::null_mut::<CrushBucketUniform>()
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_make_list_bucket(
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut weights: *mut ffi::c_int,
) -> *mut CrushBucketList {
    unsafe {
        let mut current_block: u64;
        let mut i: ffi::c_int = 0;
        let mut w: ffi::c_int = 0;
        let mut bucket: *mut CrushBucketList = std::ptr::null_mut::<CrushBucketList>();
        bucket = malloc(::core::mem::size_of::<CrushBucketList>() as ffi::c_ulong)
            as *mut CrushBucketList;
        if bucket.is_null() {
            return std::ptr::null_mut::<CrushBucketList>();
        }
        memset(
            bucket as *mut ffi::c_void,
            0,
            ::core::mem::size_of::<CrushBucketList>() as ffi::c_ulong,
        );
        (*bucket).h.alg = CRUSH_BUCKET_LIST as ffi::c_int as U8;
        (*bucket).h.hash = hash as U8;
        (*bucket).h.type_0 = type_0 as U16;
        (*bucket).h.size = size as U32;
        (*bucket).h.items = malloc(
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(size as ffi::c_ulong),
        ) as *mut S32;
        if !((*bucket).h.items).is_null() {
            (*bucket).item_weights = malloc(
                (::core::mem::size_of::<U32>() as ffi::c_ulong)
                    .wrapping_mul(size as ffi::c_ulong),
            ) as *mut U32;
            if !((*bucket).item_weights).is_null() {
                (*bucket).sum_weights = malloc(
                    (::core::mem::size_of::<U32>() as ffi::c_ulong)
                        .wrapping_mul(size as ffi::c_ulong),
                ) as *mut U32;
                if !((*bucket).sum_weights).is_null() {
                    w = 0;
                    i = 0;
                    loop {
                        if i >= size {
                            current_block = 15652330335145281839;
                            break;
                        }
                        *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
                        *((*bucket).item_weights).offset(i as isize) =
                            *weights.offset(i as isize) as U32;
                        if crush_addition_is_unsafe(w as U32, *weights.offset(i as isize) as U32)
                            != 0
                        {
                            current_block = 944831508617719848;
                            break;
                        }
                        w += *weights.offset(i as isize);
                        *((*bucket).sum_weights).offset(i as isize) = w as U32;
                        i += 1;
                    }
                    match current_block {
                        944831508617719848 => {}
                        _ => {
                            (*bucket).h.weight = w as U32;
                            return bucket;
                        }
                    }
                }
            }
        }
        free((*bucket).sum_weights as *mut ffi::c_void);
        free((*bucket).item_weights as *mut ffi::c_void);
        free((*bucket).h.items as *mut ffi::c_void);
        free(bucket as *mut ffi::c_void);
        std::ptr::null_mut::<CrushBucketList>()
    }
}
unsafe extern "C" fn height(mut n: ffi::c_int) -> ffi::c_int {
    let mut h: ffi::c_int = 0;
    while n & 1 == 0 {
        h += 1;
        n >>= 1;
    }
    h
}
unsafe extern "C" fn on_right(mut n: ffi::c_int, mut h: ffi::c_int) -> ffi::c_int {
    n & (1) << (h + 1)
}
unsafe extern "C" fn parent(mut n: ffi::c_int) -> ffi::c_int {
    unsafe {
        let mut h: ffi::c_int = height(n);
        if on_right(n, h) != 0 {
            n - ((1) << h)
        } else {
            n + ((1) << h)
        }
    }
}
unsafe extern "C" fn calc_depth(mut size: ffi::c_int) -> ffi::c_int {
    if size == 0 {
        return 0;
    }
    let mut depth: ffi::c_int = 1;
    let mut t: ffi::c_int = size - 1;
    while t != 0 {
        t >>= 1;
        depth += 1;
    }
    depth
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_make_tree_bucket(
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut weights: *mut ffi::c_int,
) -> *mut CrushBucketTree {
    unsafe {
        let mut current_block: u64;
        let mut bucket: *mut CrushBucketTree = std::ptr::null_mut::<CrushBucketTree>();
        let mut depth: ffi::c_int = 0;
        let mut node: ffi::c_int = 0;
        let mut i: ffi::c_int = 0;
        let mut j: ffi::c_int = 0;
        bucket = malloc(::core::mem::size_of::<CrushBucketTree>() as ffi::c_ulong)
            as *mut CrushBucketTree;
        if bucket.is_null() {
            return std::ptr::null_mut::<CrushBucketTree>();
        }
        memset(
            bucket as *mut ffi::c_void,
            0,
            ::core::mem::size_of::<CrushBucketTree>() as ffi::c_ulong,
        );
        (*bucket).h.alg = CRUSH_BUCKET_TREE as ffi::c_int as U8;
        (*bucket).h.hash = hash as U8;
        (*bucket).h.type_0 = type_0 as U16;
        (*bucket).h.size = size as U32;
        if size == 0 {
            (*bucket).h.items = std::ptr::null_mut::<S32>();
            (*bucket).h.weight = 0 as U32;
            (*bucket).node_weights = std::ptr::null_mut::<U32>();
            (*bucket).num_nodes = 0 as U8;
            return bucket;
        }
        (*bucket).h.items = malloc(
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(size as ffi::c_ulong),
        ) as *mut S32;
        if !((*bucket).h.items).is_null() {
            depth = calc_depth(size);
            (*bucket).num_nodes = ((1) << depth) as U8;
            (*bucket).node_weights = malloc(
                (::core::mem::size_of::<U32>() as ffi::c_ulong)
                    .wrapping_mul((*bucket).num_nodes as ffi::c_ulong),
            ) as *mut U32;
            if !((*bucket).node_weights).is_null() {
                memset(
                    (*bucket).h.items as *mut ffi::c_void,
                    0,
                    (::core::mem::size_of::<S32>() as ffi::c_ulong)
                        .wrapping_mul((*bucket).h.size as ffi::c_ulong),
                );
                memset(
                    (*bucket).node_weights as *mut ffi::c_void,
                    0,
                    (::core::mem::size_of::<U32>() as ffi::c_ulong)
                        .wrapping_mul((*bucket).num_nodes as ffi::c_ulong),
                );
                i = 0;
                's_88: loop {
                    if i >= size {
                        current_block = 16924917904204750491;
                        break;
                    }
                    *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
                    node = crush_calc_tree_node(i);
                    *((*bucket).node_weights).offset(node as isize) =
                        *weights.offset(i as isize) as U32;
                    if crush_addition_is_unsafe(
                        (*bucket).h.weight,
                        *weights.offset(i as isize) as U32,
                    ) != 0
                    {
                        current_block = 1061975787736880768;
                        break;
                    }
                    (*bucket).h.weight =
                        ((*bucket).h.weight).wrapping_add(*weights.offset(i as isize) as U32);
                    j = 1;
                    while j < depth {
                        node = parent(node);
                        if crush_addition_is_unsafe(
                            *((*bucket).node_weights).offset(node as isize),
                            *weights.offset(i as isize) as U32,
                        ) != 0
                        {
                            current_block = 1061975787736880768;
                            break 's_88;
                        }
                        let fresh3 = &mut (*((*bucket).node_weights).offset(node as isize));
                        *fresh3 = (*fresh3).wrapping_add(*weights.offset(i as isize) as U32);
                        j += 1;
                    }
                    i += 1;
                }
                match current_block {
                    1061975787736880768 => {}
                    _ => {
                        if *((*bucket).node_weights).offset(
                            ((*bucket).num_nodes as ffi::c_int / 2 as ffi::c_int) as isize,
                        ) == (*bucket).h.weight
                        {
                        } else {
                            panic!("Assertion failed: !(bucket->node_weights[bucket->num_nodes/2] != bucket->h.weight)");
                        }

                        return bucket;
                    }
                }
            }
        }
        free((*bucket).node_weights as *mut ffi::c_void);
        free((*bucket).h.items as *mut ffi::c_void);
        free(bucket as *mut ffi::c_void);
        std::ptr::null_mut::<CrushBucketTree>()
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_calc_straw(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw,
) -> ffi::c_int {
    unsafe {
        let mut reverse: *mut ffi::c_int = std::ptr::null_mut::<ffi::c_int>();
        let mut i: ffi::c_int = 0;
        let mut j: ffi::c_int = 0;
        let mut k: ffi::c_int = 0;
        let mut straw: ffi::c_double = 0.;
        let mut wbelow: ffi::c_double = 0.;
        let mut lastw: ffi::c_double = 0.;
        let mut wnext: ffi::c_double = 0.;
        let mut pbelow: ffi::c_double = 0.;
        let mut numleft: ffi::c_int = 0;
        let mut size: ffi::c_int = (*bucket).h.size as ffi::c_int;
        let mut weights: *mut U32 = (*bucket).item_weights;
        reverse = malloc(
            (::core::mem::size_of::<ffi::c_int>() as ffi::c_ulong)
                .wrapping_mul(size as ffi::c_ulong),
        ) as *mut ffi::c_int;
        if reverse.is_null() {
            return -(12 as ffi::c_int);
        }
        if size != 0 {
            *reverse.offset(0 as isize) = 0;
        }
        i = 1;
        while i < size {
            j = 0;
            while j < i {
                if *weights.offset(i as isize)
                    < *weights.offset(*reverse.offset(j as isize) as isize)
                {
                    k = i;
                    while k > j {
                        *reverse.offset(k as isize) =
                            *reverse.offset((k - 1) as isize);
                        k -= 1;
                    }
                    *reverse.offset(j as isize) = i;
                    break;
                } else {
                    j += 1;
                }
            }
            if j == i {
                *reverse.offset(i as isize) = i;
            }
            i += 1;
        }
        numleft = size;
        straw = 1.0f64;
        wbelow = 0 as ffi::c_double;
        lastw = 0 as ffi::c_double;
        i = 0;
        while i < size {
            if (*map).straw_calc_version as ffi::c_int == 0 {
                if *weights.offset(*reverse.offset(i as isize) as isize) == 0 as U32
                {
                    *((*bucket).straws).offset(*reverse.offset(i as isize) as isize) =
                        0 as U32;
                    i += 1;
                } else {
                    *((*bucket).straws).offset(*reverse.offset(i as isize) as isize) =
                        (straw * 0x10000 as ffi::c_double) as U32;
                    i += 1;
                    if i == size {
                        break;
                    }
                    if *weights.offset(*reverse.offset(i as isize) as isize)
                        == *weights
                            .offset(*reverse.offset((i - 1) as isize) as isize)
                    {
                        continue;
                    }
                    wbelow += (*weights
                        .offset(*reverse.offset((i - 1) as isize) as isize)
                        as ffi::c_double
                        - lastw)
                        * numleft as ffi::c_double;
                    j = i;
                    while j < size {
                        if *weights.offset(*reverse.offset(j as isize) as isize)
                            != *weights.offset(*reverse.offset(i as isize) as isize)
                        {
                            break;
                        }
                        numleft -= 1;
                        j += 1;
                    }
                    wnext = (numleft as U32
                        * (*weights.offset(*reverse.offset(i as isize) as isize)).wrapping_sub(
                            *weights
                                .offset(*reverse.offset((i - 1) as isize) as isize),
                        )) as ffi::c_double;
                    pbelow = wbelow / (wbelow + wnext);
                    straw *= pow(1.0f64 / pbelow, 1.0f64 / numleft as ffi::c_double);
                    lastw = *weights
                        .offset(*reverse.offset((i - 1) as isize) as isize)
                        as ffi::c_double;
                }
            } else {
                if ((*map).straw_calc_version as ffi::c_int) < 1 {
                    continue;
                }
                if *weights.offset(*reverse.offset(i as isize) as isize) == 0 as U32
                {
                    *((*bucket).straws).offset(*reverse.offset(i as isize) as isize) =
                        0 as U32;
                    i += 1;
                    numleft -= 1;
                } else {
                    *((*bucket).straws).offset(*reverse.offset(i as isize) as isize) =
                        (straw * 0x10000 as ffi::c_double) as U32;
                    i += 1;
                    if i == size {
                        break;
                    }
                    wbelow += (*weights
                        .offset(*reverse.offset((i - 1) as isize) as isize)
                        as ffi::c_double
                        - lastw)
                        * numleft as ffi::c_double;
                    numleft -= 1;
                    wnext = (numleft as U32
                        * (*weights.offset(*reverse.offset(i as isize) as isize)).wrapping_sub(
                            *weights
                                .offset(*reverse.offset((i - 1) as isize) as isize),
                        )) as ffi::c_double;
                    pbelow = wbelow / (wbelow + wnext);
                    straw *= pow(1.0f64 / pbelow, 1.0f64 / numleft as ffi::c_double);
                    lastw = *weights
                        .offset(*reverse.offset((i - 1) as isize) as isize)
                        as ffi::c_double;
                }
            }
        }
        free(reverse as *mut ffi::c_void);
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_make_straw_bucket(
    mut map: *mut CrushMap,
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut weights: *mut ffi::c_int,
) -> *mut CrushBucketStraw {
    unsafe {
        let mut bucket: *mut CrushBucketStraw = std::ptr::null_mut::<CrushBucketStraw>();
        let mut i: ffi::c_int = 0;
        bucket = malloc(::core::mem::size_of::<CrushBucketStraw>() as ffi::c_ulong)
            as *mut CrushBucketStraw;
        if bucket.is_null() {
            return std::ptr::null_mut::<CrushBucketStraw>();
        }
        memset(
            bucket as *mut ffi::c_void,
            0,
            ::core::mem::size_of::<CrushBucketStraw>() as ffi::c_ulong,
        );
        (*bucket).h.alg = CRUSH_BUCKET_STRAW as ffi::c_int as U8;
        (*bucket).h.hash = hash as U8;
        (*bucket).h.type_0 = type_0 as U16;
        (*bucket).h.size = size as U32;
        (*bucket).h.items = malloc(
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(size as ffi::c_ulong),
        ) as *mut S32;
        if !((*bucket).h.items).is_null() {
            (*bucket).item_weights = malloc(
                (::core::mem::size_of::<U32>() as ffi::c_ulong)
                    .wrapping_mul(size as ffi::c_ulong),
            ) as *mut U32;
            if !((*bucket).item_weights).is_null() {
                (*bucket).straws = malloc(
                    (::core::mem::size_of::<U32>() as ffi::c_ulong)
                        .wrapping_mul(size as ffi::c_ulong),
                ) as *mut U32;
                if !((*bucket).straws).is_null() {
                    (*bucket).h.weight = 0 as U32;
                    i = 0;
                    while i < size {
                        *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
                        (*bucket).h.weight =
                            ((*bucket).h.weight).wrapping_add(*weights.offset(i as isize) as U32);
                        *((*bucket).item_weights).offset(i as isize) =
                            *weights.offset(i as isize) as U32;
                        i += 1;
                    }
                    if crush_calc_straw(map, bucket) >= 0 {
                        return bucket;
                    }
                }
            }
        }
        free((*bucket).straws as *mut ffi::c_void);
        free((*bucket).item_weights as *mut ffi::c_void);
        free((*bucket).h.items as *mut ffi::c_void);
        free(bucket as *mut ffi::c_void);
        std::ptr::null_mut::<CrushBucketStraw>()
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_make_straw2_bucket(
    mut _map: *mut CrushMap,
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut weights: *mut ffi::c_int,
) -> *mut CrushBucketStraw2 {
    unsafe {
        let mut bucket: *mut CrushBucketStraw2 = std::ptr::null_mut::<CrushBucketStraw2>();
        let mut i: ffi::c_int = 0;
        bucket = malloc(::core::mem::size_of::<CrushBucketStraw2>() as ffi::c_ulong)
            as *mut CrushBucketStraw2;
        if bucket.is_null() {
            return std::ptr::null_mut::<CrushBucketStraw2>();
        }
        memset(
            bucket as *mut ffi::c_void,
            0,
            ::core::mem::size_of::<CrushBucketStraw2>() as ffi::c_ulong,
        );
        (*bucket).h.alg = CRUSH_BUCKET_STRAW2 as ffi::c_int as U8;
        (*bucket).h.hash = hash as U8;
        (*bucket).h.type_0 = type_0 as U16;
        (*bucket).h.size = size as U32;
        (*bucket).h.items = malloc(
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(size as ffi::c_ulong),
        ) as *mut S32;
        if !((*bucket).h.items).is_null() {
            (*bucket).item_weights = malloc(
                (::core::mem::size_of::<U32>() as ffi::c_ulong)
                    .wrapping_mul(size as ffi::c_ulong),
            ) as *mut U32;
            if !((*bucket).item_weights).is_null() {
                (*bucket).h.weight = 0 as U32;
                i = 0;
                while i < size {
                    *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
                    (*bucket).h.weight =
                        ((*bucket).h.weight).wrapping_add(*weights.offset(i as isize) as U32);
                    *((*bucket).item_weights).offset(i as isize) =
                        *weights.offset(i as isize) as U32;
                    i += 1;
                }
                return bucket;
            }
        }
        free((*bucket).item_weights as *mut ffi::c_void);
        free((*bucket).h.items as *mut ffi::c_void);
        free(bucket as *mut ffi::c_void);
        std::ptr::null_mut::<CrushBucketStraw2>()
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_make_bucket(
    mut map: *mut CrushMap,
    mut alg: ffi::c_int,
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut weights: *mut ffi::c_int,
) -> *mut CrushBucket {
    unsafe {
        let mut item_weight: ffi::c_int = 0;
        match alg {
            1 => {
                if size != 0 && !weights.is_null() {
                    item_weight = *weights.offset(0 as isize);
                } else {
                    item_weight = 0;
                }
                return crush_make_uniform_bucket(hash, type_0, size, items, item_weight)
                    as *mut CrushBucket;
            }
            2 => {
                return crush_make_list_bucket(hash, type_0, size, items, weights)
                    as *mut CrushBucket;
            }
            3 => {
                return crush_make_tree_bucket(hash, type_0, size, items, weights)
                    as *mut CrushBucket;
            }
            4 => {
                return crush_make_straw_bucket(map, hash, type_0, size, items, weights)
                    as *mut CrushBucket;
            }
            5 => {
                return crush_make_straw2_bucket(map, hash, type_0, size, items, weights)
                    as *mut CrushBucket;
            }
            _ => {}
        }
        std::ptr::null_mut::<CrushBucket>()
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_add_uniform_bucket_item(
    mut bucket: *mut CrushBucketUniform,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut newsize: ffi::c_int =
            ((*bucket).h.size).wrapping_add(1 as U32) as ffi::c_int;
        let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
        if (*bucket).item_weight != weight as U32 {
            return -(22 as ffi::c_int);
        }
        _realloc = realloc(
            (*bucket).h.items as *mut ffi::c_void,
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).h.items = _realloc as *mut S32;
        }
        *((*bucket).h.items).offset((newsize - 1) as isize) = item;
        if crush_addition_is_unsafe((*bucket).h.weight, weight as U32) != 0 {
            return -(34 as ffi::c_int);
        }
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as U32);
        (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_add_list_bucket_item(
    mut bucket: *mut CrushBucketList,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut newsize: ffi::c_int =
            ((*bucket).h.size).wrapping_add(1 as U32) as ffi::c_int;
        let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
        _realloc = realloc(
            (*bucket).h.items as *mut ffi::c_void,
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).h.items = _realloc as *mut S32;
        }
        _realloc = realloc(
            (*bucket).item_weights as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).item_weights = _realloc as *mut U32;
        }
        _realloc = realloc(
            (*bucket).sum_weights as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).sum_weights = _realloc as *mut U32;
        }
        *((*bucket).h.items).offset((newsize - 1) as isize) = item;
        *((*bucket).item_weights).offset((newsize - 1) as isize) = weight as U32;
        if newsize > 1 {
            if crush_addition_is_unsafe(
                *((*bucket).sum_weights).offset((newsize - 2 as ffi::c_int) as isize),
                weight as U32,
            ) != 0
            {
                return -(34 as ffi::c_int);
            }
            *((*bucket).sum_weights).offset((newsize - 1) as isize) =
                (*((*bucket).sum_weights).offset((newsize - 2 as ffi::c_int) as isize))
                    .wrapping_add(weight as U32);
        } else {
            *((*bucket).sum_weights).offset((newsize - 1) as isize) = weight as U32;
        }
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as U32);
        (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_add_tree_bucket_item(
    mut bucket: *mut CrushBucketTree,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut newsize: ffi::c_int =
            ((*bucket).h.size).wrapping_add(1 as U32) as ffi::c_int;
        let mut depth: ffi::c_int = calc_depth(newsize);
        let mut node: ffi::c_int = 0;
        let mut j: ffi::c_int = 0;
        let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
        (*bucket).num_nodes = ((1) << depth) as U8;
        _realloc = realloc(
            (*bucket).h.items as *mut ffi::c_void,
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).h.items = _realloc as *mut S32;
        }
        _realloc = realloc(
            (*bucket).node_weights as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong)
                .wrapping_mul((*bucket).num_nodes as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).node_weights = _realloc as *mut U32;
        }
        node = crush_calc_tree_node(newsize - 1);
        *((*bucket).node_weights).offset(node as isize) = weight as U32;
        let mut root: ffi::c_int = (*bucket).num_nodes as ffi::c_int / 2 as ffi::c_int;
        if depth >= 2 as ffi::c_int && node - 1 == root {
            *((*bucket).node_weights).offset(root as isize) =
                *((*bucket).node_weights).offset((root / 2 as ffi::c_int) as isize);
        }
        j = 1;
        while j < depth {
            node = parent(node);
            if crush_addition_is_unsafe(
                *((*bucket).node_weights).offset(node as isize),
                weight as U32,
            ) != 0
            {
                return -(34 as ffi::c_int);
            }
            let fresh4 = &mut (*((*bucket).node_weights).offset(node as isize));
            *fresh4 = (*fresh4).wrapping_add(weight as U32);
            j += 1;
        }
        if crush_addition_is_unsafe((*bucket).h.weight, weight as U32) != 0 {
            return -(34 as ffi::c_int);
        }
        *((*bucket).h.items).offset((newsize - 1) as isize) = item;
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as U32);
        (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_add_straw_bucket_item(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut newsize: ffi::c_int =
            ((*bucket).h.size).wrapping_add(1 as U32) as ffi::c_int;
        let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
        _realloc = realloc(
            (*bucket).h.items as *mut ffi::c_void,
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).h.items = _realloc as *mut S32;
        }
        _realloc = realloc(
            (*bucket).item_weights as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).item_weights = _realloc as *mut U32;
        }
        _realloc = realloc(
            (*bucket).straws as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).straws = _realloc as *mut U32;
        }
        *((*bucket).h.items).offset((newsize - 1) as isize) = item;
        *((*bucket).item_weights).offset((newsize - 1) as isize) = weight as U32;
        if crush_addition_is_unsafe((*bucket).h.weight, weight as U32) != 0 {
            return -(34 as ffi::c_int);
        }
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as U32);
        (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
        crush_calc_straw(map, bucket)
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_add_straw2_bucket_item(
    mut _map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw2,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut newsize: ffi::c_int =
            ((*bucket).h.size).wrapping_add(1 as U32) as ffi::c_int;
        let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
        _realloc = realloc(
            (*bucket).h.items as *mut ffi::c_void,
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).h.items = _realloc as *mut S32;
        }
        _realloc = realloc(
            (*bucket).item_weights as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).item_weights = _realloc as *mut U32;
        }
        *((*bucket).h.items).offset((newsize - 1) as isize) = item;
        *((*bucket).item_weights).offset((newsize - 1) as isize) = weight as U32;
        if crush_addition_is_unsafe((*bucket).h.weight, weight as U32) != 0 {
            return -(34 as ffi::c_int);
        }
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as U32);
        (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
        0
    }
}
#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn crush_bucket_add_item(
    mut map: *mut CrushMap,
    mut b: *mut CrushBucket,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        match (*b).alg as ffi::c_int {
            1 => crush_add_uniform_bucket_item(b as *mut CrushBucketUniform, item, weight),
            2 => crush_add_list_bucket_item(b as *mut CrushBucketList, item, weight),
            3 => crush_add_tree_bucket_item(b as *mut CrushBucketTree, item, weight),
            4 => crush_add_straw_bucket_item(map, b as *mut CrushBucketStraw, item, weight),
            5 => crush_add_straw2_bucket_item(map, b as *mut CrushBucketStraw2, item, weight),
            _ => -1,
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_remove_uniform_bucket_item(
    mut bucket: *mut CrushBucketUniform,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_uint = 0;
        let mut j: ffi::c_uint = 0;
        let mut newsize: ffi::c_int = 0;
        let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            if *((*bucket).h.items).offset(i as isize) == item {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i == (*bucket).h.size {
            return -(2 as ffi::c_int);
        }
        j = i;
        while j < (*bucket).h.size {
            *((*bucket).h.items).offset(j as isize) = *((*bucket).h.items)
                .offset(j.wrapping_add(1 as ffi::c_uint) as isize);
            j = j.wrapping_add(1);
        }
        (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
        newsize = (*bucket).h.size as ffi::c_int;
        if (*bucket).item_weight < (*bucket).h.weight {
            (*bucket).h.weight = ((*bucket).h.weight).wrapping_sub((*bucket).item_weight);
        } else {
            (*bucket).h.weight = 0 as U32;
        }
        _realloc = realloc(
            (*bucket).h.items as *mut ffi::c_void,
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).h.items = _realloc as *mut S32;
        }
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_remove_list_bucket_item(
    mut bucket: *mut CrushBucketList,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_uint = 0;
        let mut j: ffi::c_uint = 0;
        let mut newsize: ffi::c_int = 0;
        let mut weight: ffi::c_uint = 0;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            if *((*bucket).h.items).offset(i as isize) == item {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i == (*bucket).h.size {
            return -(2 as ffi::c_int);
        }
        weight = *((*bucket).item_weights).offset(i as isize);
        j = i;
        while j < (*bucket).h.size {
            *((*bucket).h.items).offset(j as isize) = *((*bucket).h.items)
                .offset(j.wrapping_add(1 as ffi::c_uint) as isize);
            *((*bucket).item_weights).offset(j as isize) = *((*bucket).item_weights)
                .offset(j.wrapping_add(1 as ffi::c_uint) as isize);
            *((*bucket).sum_weights).offset(j as isize) = (*((*bucket).sum_weights)
                .offset(j.wrapping_add(1 as ffi::c_uint) as isize))
            .wrapping_sub(weight);
            j = j.wrapping_add(1);
        }
        if weight < (*bucket).h.weight {
            (*bucket).h.weight =
                ((*bucket).h.weight as ffi::c_uint).wrapping_sub(weight) as U32 as U32;
        } else {
            (*bucket).h.weight = 0 as U32;
        }
        (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
        newsize = (*bucket).h.size as ffi::c_int;
        let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
        _realloc = realloc(
            (*bucket).h.items as *mut ffi::c_void,
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).h.items = _realloc as *mut S32;
        }
        _realloc = realloc(
            (*bucket).item_weights as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).item_weights = _realloc as *mut U32;
        }
        _realloc = realloc(
            (*bucket).sum_weights as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).sum_weights = _realloc as *mut U32;
        }
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_remove_tree_bucket_item(
    mut bucket: *mut CrushBucketTree,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_uint = 0;
        let mut newsize: ffi::c_uint = 0;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            let mut node: ffi::c_int = 0;
            let mut weight: ffi::c_uint = 0;
            let mut j: ffi::c_int = 0;
            let mut depth: ffi::c_int = calc_depth((*bucket).h.size as ffi::c_int);
            if *((*bucket).h.items).offset(i as isize) != item {
                i = i.wrapping_add(1);
            } else {
                *((*bucket).h.items).offset(i as isize) = 0;
                node = crush_calc_tree_node(i as ffi::c_int);
                weight = *((*bucket).node_weights).offset(node as isize);
                *((*bucket).node_weights).offset(node as isize) = 0 as U32;
                j = 1;
                while j < depth {
                    node = parent(node);
                    let fresh5 = &mut (*((*bucket).node_weights).offset(node as isize));
                    *fresh5 = (*fresh5 as ffi::c_uint).wrapping_sub(weight) as U32 as U32;
                    j += 1;
                }
                if weight < (*bucket).h.weight {
                    (*bucket).h.weight =
                        ((*bucket).h.weight as ffi::c_uint).wrapping_sub(weight) as U32 as U32;
                } else {
                    (*bucket).h.weight = 0 as U32;
                }
                break;
            }
        }
        if i == (*bucket).h.size {
            return -(2 as ffi::c_int);
        }
        newsize = (*bucket).h.size;
        while newsize > 0 as ffi::c_uint {
            let mut node_0: ffi::c_int = crush_calc_tree_node(
                newsize.wrapping_sub(1 as ffi::c_uint) as ffi::c_int,
            );
            if *((*bucket).node_weights).offset(node_0 as isize) != 0 {
                break;
            }
            newsize = newsize.wrapping_sub(1);
        }
        if newsize != (*bucket).h.size {
            let mut olddepth: ffi::c_int = 0;
            let mut newdepth: ffi::c_int = 0;
            let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
            _realloc = realloc(
                (*bucket).h.items as *mut ffi::c_void,
                (::core::mem::size_of::<S32>() as ffi::c_ulong)
                    .wrapping_mul(newsize as ffi::c_ulong),
            );
            if _realloc.is_null() {
                return -(12 as ffi::c_int);
            } else {
                (*bucket).h.items = _realloc as *mut S32;
            }
            olddepth = calc_depth((*bucket).h.size as ffi::c_int);
            newdepth = calc_depth(newsize as ffi::c_int);
            if olddepth != newdepth {
                (*bucket).num_nodes = ((1) << newdepth) as U8;
                _realloc = realloc(
                    (*bucket).node_weights as *mut ffi::c_void,
                    (::core::mem::size_of::<U32>() as ffi::c_ulong)
                        .wrapping_mul((*bucket).num_nodes as ffi::c_ulong),
                );
                if _realloc.is_null() {
                    return -(12 as ffi::c_int);
                } else {
                    (*bucket).node_weights = _realloc as *mut U32;
                }
            }
            (*bucket).h.size = newsize;
        }
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_remove_straw_bucket_item(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut newsize: ffi::c_int =
            ((*bucket).h.size).wrapping_sub(1 as U32) as ffi::c_int;
        let mut i: ffi::c_uint = 0;
        let mut j: ffi::c_uint = 0;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            if *((*bucket).h.items).offset(i as isize) == item {
                (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
                if *((*bucket).item_weights).offset(i as isize) < (*bucket).h.weight {
                    (*bucket).h.weight = ((*bucket).h.weight)
                        .wrapping_sub(*((*bucket).item_weights).offset(i as isize));
                } else {
                    (*bucket).h.weight = 0 as U32;
                }
                j = i;
                while j < (*bucket).h.size {
                    *((*bucket).h.items).offset(j as isize) = *((*bucket).h.items)
                        .offset(j.wrapping_add(1 as ffi::c_uint) as isize);
                    *((*bucket).item_weights).offset(j as isize) = *((*bucket).item_weights)
                        .offset(j.wrapping_add(1 as ffi::c_uint) as isize);
                    j = j.wrapping_add(1);
                }
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if i == (*bucket).h.size {
            return -(2 as ffi::c_int);
        }
        let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
        _realloc = realloc(
            (*bucket).h.items as *mut ffi::c_void,
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).h.items = _realloc as *mut S32;
        }
        _realloc = realloc(
            (*bucket).item_weights as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).item_weights = _realloc as *mut U32;
        }
        _realloc = realloc(
            (*bucket).straws as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).straws = _realloc as *mut U32;
        }
        crush_calc_straw(map, bucket)
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_remove_straw2_bucket_item(
    mut _map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw2,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut newsize: ffi::c_int =
            ((*bucket).h.size).wrapping_sub(1 as U32) as ffi::c_int;
        let mut i: ffi::c_uint = 0;
        let mut j: ffi::c_uint = 0;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            if *((*bucket).h.items).offset(i as isize) == item {
                (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
                if *((*bucket).item_weights).offset(i as isize) < (*bucket).h.weight {
                    (*bucket).h.weight = ((*bucket).h.weight)
                        .wrapping_sub(*((*bucket).item_weights).offset(i as isize));
                } else {
                    (*bucket).h.weight = 0 as U32;
                }
                j = i;
                while j < (*bucket).h.size {
                    *((*bucket).h.items).offset(j as isize) = *((*bucket).h.items)
                        .offset(j.wrapping_add(1 as ffi::c_uint) as isize);
                    *((*bucket).item_weights).offset(j as isize) = *((*bucket).item_weights)
                        .offset(j.wrapping_add(1 as ffi::c_uint) as isize);
                    j = j.wrapping_add(1);
                }
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if i == (*bucket).h.size {
            return -(2 as ffi::c_int);
        }
        let mut _realloc: *mut ffi::c_void = std::ptr::null_mut::<ffi::c_void>();
        _realloc = realloc(
            (*bucket).h.items as *mut ffi::c_void,
            (::core::mem::size_of::<S32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).h.items = _realloc as *mut S32;
        }
        _realloc = realloc(
            (*bucket).item_weights as *mut ffi::c_void,
            (::core::mem::size_of::<U32>() as ffi::c_ulong).wrapping_mul(newsize as ffi::c_ulong),
        );
        if _realloc.is_null() {
            return -(12 as ffi::c_int);
        } else {
            (*bucket).item_weights = _realloc as *mut U32;
        }
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_bucket_remove_item(
    mut map: *mut CrushMap,
    mut b: *mut CrushBucket,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        match (*b).alg as ffi::c_int {
            1 => crush_remove_uniform_bucket_item(b as *mut CrushBucketUniform, item),
            2 => crush_remove_list_bucket_item(b as *mut CrushBucketList, item),
            3 => crush_remove_tree_bucket_item(b as *mut CrushBucketTree, item),
            4 => crush_remove_straw_bucket_item(map, b as *mut CrushBucketStraw, item),
            5 => crush_remove_straw2_bucket_item(map, b as *mut CrushBucketStraw2, item),
            _ => -1,
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_adjust_uniform_bucket_item_weight(
    mut bucket: *mut CrushBucketUniform,
    mut _item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut diff: ffi::c_int =
            ((weight as U32).wrapping_sub((*bucket).item_weight) * (*bucket).h.size) as ffi::c_int;
        (*bucket).item_weight = weight as U32;
        (*bucket).h.weight = (*bucket).item_weight * (*bucket).h.size;
        diff
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_adjust_list_bucket_item_weight(
    mut bucket: *mut CrushBucketList,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut diff: ffi::c_int = 0;
        let mut i: ffi::c_uint = 0;
        let mut j: ffi::c_uint = 0;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            if *((*bucket).h.items).offset(i as isize) == item {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i == (*bucket).h.size {
            return 0;
        }
        diff = (weight as U32).wrapping_sub(*((*bucket).item_weights).offset(i as isize))
            as ffi::c_int;
        *((*bucket).item_weights).offset(i as isize) = weight as U32;
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(diff as U32);
        j = i;
        while j < (*bucket).h.size {
            let fresh6 = &mut (*((*bucket).sum_weights).offset(j as isize));
            *fresh6 = (*fresh6).wrapping_add(diff as U32);
            j = j.wrapping_add(1);
        }
        diff
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_adjust_tree_bucket_item_weight(
    mut bucket: *mut CrushBucketTree,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut diff: ffi::c_int = 0;
        let mut node: ffi::c_int = 0;
        let mut i: ffi::c_uint = 0;
        let mut j: ffi::c_uint = 0;
        let mut depth: ffi::c_uint = calc_depth((*bucket).h.size as ffi::c_int) as ffi::c_uint;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            if *((*bucket).h.items).offset(i as isize) == item {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i == (*bucket).h.size {
            return 0;
        }
        node = crush_calc_tree_node(i as ffi::c_int);
        diff = (weight as U32).wrapping_sub(*((*bucket).node_weights).offset(node as isize))
            as ffi::c_int;
        *((*bucket).node_weights).offset(node as isize) = weight as U32;
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(diff as U32);
        j = 1 as ffi::c_uint;
        while j < depth {
            node = parent(node);
            let fresh7 = &mut (*((*bucket).node_weights).offset(node as isize));
            *fresh7 = (*fresh7).wrapping_add(diff as U32);
            j = j.wrapping_add(1);
        }
        diff
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_adjust_straw_bucket_item_weight(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut idx: ffi::c_uint = 0;
        let mut diff: ffi::c_int = 0;
        let mut r: ffi::c_int = 0;
        idx = 0 as ffi::c_uint;
        while idx < (*bucket).h.size {
            if *((*bucket).h.items).offset(idx as isize) == item {
                break;
            }
            idx = idx.wrapping_add(1);
        }
        if idx == (*bucket).h.size {
            return 0;
        }
        diff = (weight as U32).wrapping_sub(*((*bucket).item_weights).offset(idx as isize))
            as ffi::c_int;
        *((*bucket).item_weights).offset(idx as isize) = weight as U32;
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(diff as U32);
        r = crush_calc_straw(map, bucket);
        if r < 0 {
            return r;
        }
        diff
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_adjust_straw2_bucket_item_weight(
    mut _map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw2,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut idx: ffi::c_uint = 0;
        let mut diff: ffi::c_int = 0;
        idx = 0 as ffi::c_uint;
        while idx < (*bucket).h.size {
            if *((*bucket).h.items).offset(idx as isize) == item {
                break;
            }
            idx = idx.wrapping_add(1);
        }
        if idx == (*bucket).h.size {
            return 0;
        }
        diff = (weight as U32).wrapping_sub(*((*bucket).item_weights).offset(idx as isize))
            as ffi::c_int;
        *((*bucket).item_weights).offset(idx as isize) = weight as U32;
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(diff as U32);
        diff
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_bucket_adjust_item_weight(
    mut map: *mut CrushMap,
    mut b: *mut CrushBucket,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        match (*b).alg as ffi::c_int {
            1 => {
                crush_adjust_uniform_bucket_item_weight(b as *mut CrushBucketUniform, item, weight)
            }
            2 => crush_adjust_list_bucket_item_weight(b as *mut CrushBucketList, item, weight),
            3 => crush_adjust_tree_bucket_item_weight(b as *mut CrushBucketTree, item, weight),
            4 => {
                crush_adjust_straw_bucket_item_weight(map, b as *mut CrushBucketStraw, item, weight)
            }
            5 => crush_adjust_straw2_bucket_item_weight(
                map,
                b as *mut CrushBucketStraw2,
                item,
                weight,
            ),
            _ => -1,
        }
    }
}
unsafe extern "C" fn crush_reweight_uniform_bucket(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketUniform,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_uint = 0;
        let mut sum: ffi::c_uint = 0 as ffi::c_uint;
        let mut n: ffi::c_uint = 0 as ffi::c_uint;
        let mut leaves: ffi::c_uint = 0 as ffi::c_uint;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            let mut id: ffi::c_int = *((*bucket).h.items).offset(i as isize);
            if id < 0 {
                let mut c: *mut CrushBucket =
                    *((*map).buckets).offset(((-1) - id) as isize);
                crush_reweight_bucket(map, c);
                if crush_addition_is_unsafe(sum, (*c).weight) != 0 {
                    return -(34 as ffi::c_int);
                }
                sum = sum.wrapping_add((*c).weight);
                n = n.wrapping_add(1);
            } else {
                leaves = leaves.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if n > leaves {
            (*bucket).item_weight = sum.wrapping_div(n);
        }
        (*bucket).h.weight = (*bucket).item_weight * (*bucket).h.size;
        0
    }
}
unsafe extern "C" fn crush_reweight_list_bucket(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketList,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_uint = 0;
        (*bucket).h.weight = 0 as U32;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            let mut id: ffi::c_int = *((*bucket).h.items).offset(i as isize);
            if id < 0 {
                let mut c: *mut CrushBucket =
                    *((*map).buckets).offset(((-1) - id) as isize);
                crush_reweight_bucket(map, c);
                *((*bucket).item_weights).offset(i as isize) = (*c).weight;
            }
            if crush_addition_is_unsafe(
                (*bucket).h.weight,
                *((*bucket).item_weights).offset(i as isize),
            ) != 0
            {
                return -(34 as ffi::c_int);
            }
            (*bucket).h.weight =
                ((*bucket).h.weight).wrapping_add(*((*bucket).item_weights).offset(i as isize));
            i = i.wrapping_add(1);
        }
        0
    }
}
unsafe extern "C" fn crush_reweight_tree_bucket(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketTree,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_uint = 0;
        (*bucket).h.weight = 0 as U32;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            let mut node: ffi::c_int = crush_calc_tree_node(i as ffi::c_int);
            let mut id: ffi::c_int = *((*bucket).h.items).offset(i as isize);
            if id < 0 {
                let mut c: *mut CrushBucket =
                    *((*map).buckets).offset(((-1) - id) as isize);
                crush_reweight_bucket(map, c);
                *((*bucket).node_weights).offset(node as isize) = (*c).weight;
            }
            if crush_addition_is_unsafe(
                (*bucket).h.weight,
                *((*bucket).node_weights).offset(node as isize),
            ) != 0
            {
                return -(34 as ffi::c_int);
            }
            (*bucket).h.weight =
                ((*bucket).h.weight).wrapping_add(*((*bucket).node_weights).offset(node as isize));
            i = i.wrapping_add(1);
        }
        0
    }
}
unsafe extern "C" fn crush_reweight_straw_bucket(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_uint = 0;
        (*bucket).h.weight = 0 as U32;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            let mut id: ffi::c_int = *((*bucket).h.items).offset(i as isize);
            if id < 0 {
                let mut c: *mut CrushBucket =
                    *((*map).buckets).offset(((-1) - id) as isize);
                crush_reweight_bucket(map, c);
                *((*bucket).item_weights).offset(i as isize) = (*c).weight;
            }
            if crush_addition_is_unsafe(
                (*bucket).h.weight,
                *((*bucket).item_weights).offset(i as isize),
            ) != 0
            {
                return -(34 as ffi::c_int);
            }
            (*bucket).h.weight =
                ((*bucket).h.weight).wrapping_add(*((*bucket).item_weights).offset(i as isize));
            i = i.wrapping_add(1);
        }
        crush_calc_straw(map, bucket);
        0
    }
}
unsafe extern "C" fn crush_reweight_straw2_bucket(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw2,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_uint = 0;
        (*bucket).h.weight = 0 as U32;
        i = 0 as ffi::c_uint;
        while i < (*bucket).h.size {
            let mut id: ffi::c_int = *((*bucket).h.items).offset(i as isize);
            if id < 0 {
                let mut c: *mut CrushBucket =
                    *((*map).buckets).offset(((-1) - id) as isize);
                crush_reweight_bucket(map, c);
                *((*bucket).item_weights).offset(i as isize) = (*c).weight;
            }
            if crush_addition_is_unsafe(
                (*bucket).h.weight,
                *((*bucket).item_weights).offset(i as isize),
            ) != 0
            {
                return -(34 as ffi::c_int);
            }
            (*bucket).h.weight =
                ((*bucket).h.weight).wrapping_add(*((*bucket).item_weights).offset(i as isize));
            i = i.wrapping_add(1);
        }
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_reweight_bucket(
    map: *mut CrushMap,
    b: *mut CrushBucket,
) -> ffi::c_int {
    match (*b).alg as ffi::c_int {
        1 => crush_reweight_uniform_bucket(map, b as *mut CrushBucketUniform),
        2 => crush_reweight_list_bucket(map, b as *mut CrushBucketList),
        3 => crush_reweight_tree_bucket(map, b as *mut CrushBucketTree),
        4 => crush_reweight_straw_bucket(map, b as *mut CrushBucketStraw),
        5 => crush_reweight_straw2_bucket(map, b as *mut CrushBucketStraw2),
        _ => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_make_choose_args(
    mut map: *mut CrushMap,
    mut num_positions: ffi::c_int,
) -> *mut CrushChooseArg {
    unsafe {
        let mut b: ffi::c_int = 0;
        let mut sum_bucket_size: ffi::c_int = 0;
        let mut bucket_count: ffi::c_int = 0;
        b = 0;
        while b < (*map).max_buckets {
            if !(*((*map).buckets).offset(b as isize)).is_null() {
                sum_bucket_size = (sum_bucket_size as U32)
                    .wrapping_add((**((*map).buckets).offset(b as isize)).size)
                    as ffi::c_int as ffi::c_int;
                bucket_count += 1;
            }
            b += 1;
        }
        let mut size: ffi::c_int = (::core::mem::size_of::<CrushChooseArg>() as ffi::c_ulong)
            .wrapping_mul((*map).max_buckets as ffi::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<CrushWeightSet>() as ffi::c_ulong)
                    .wrapping_mul(bucket_count as ffi::c_ulong)
                    .wrapping_mul(num_positions as ffi::c_ulong),
            )
            .wrapping_add(
                (::core::mem::size_of::<U32>() as ffi::c_ulong)
                    .wrapping_mul(sum_bucket_size as ffi::c_ulong)
                    .wrapping_mul(num_positions as ffi::c_ulong),
            )
            .wrapping_add(
                (::core::mem::size_of::<U32>() as ffi::c_ulong)
                    .wrapping_mul(sum_bucket_size as ffi::c_ulong),
            ) as ffi::c_int;
        let mut space: *mut ffi::c_char = malloc(size as ffi::c_ulong) as *mut ffi::c_char;
        let mut arg: *mut CrushChooseArg = space as *mut CrushChooseArg;
        let mut weight_set: *mut CrushWeightSet =
            arg.offset((*map).max_buckets as isize) as *mut CrushWeightSet;
        let mut weights: *mut U32 =
            weight_set.offset((bucket_count * num_positions) as isize) as *mut U32;
        let mut weight_set_ends: *mut ffi::c_char = weights as *mut ffi::c_char;
        let mut ids: *mut ffi::c_int =
            weights.offset((sum_bucket_size * num_positions) as isize) as *mut ffi::c_int;
        let mut weights_end: *mut ffi::c_char = ids as *mut ffi::c_char;
        let mut ids_end: *mut ffi::c_char =
            ids.offset(sum_bucket_size as isize) as *mut ffi::c_char;
        if space.offset(size as isize) == ids_end {
        } else {
            panic!("Assertion failed: !(space + size != ids_end)");
        }

        b = 0;
        while b < (*map).max_buckets {
            if (*((*map).buckets).offset(b as isize)).is_null() {
                memset(
                    &mut *arg.offset(b as isize) as *mut CrushChooseArg as *mut ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<CrushChooseArg>() as ffi::c_ulong,
                );
            } else {
                let mut bucket: *mut CrushBucketStraw2 =
                    *((*map).buckets).offset(b as isize) as *mut CrushBucketStraw2;
                let mut position: ffi::c_int = 0;
                position = 0;
                while position < num_positions {
                    memcpy(
                        weights as *mut ffi::c_void,
                        (*bucket).item_weights as *const ffi::c_void,
                        (::core::mem::size_of::<U32>() as ffi::c_ulong)
                            .wrapping_mul((*bucket).h.size as ffi::c_ulong),
                    );
                    let fresh8 = &mut (*weight_set.offset(position as isize)).weights;
                    *fresh8 = weights;
                    (*weight_set.offset(position as isize)).size = (*bucket).h.size;
                    weights = weights.offset((*bucket).h.size as isize);
                    position += 1;
                }
                let fresh9 = &mut (*arg.offset(b as isize)).weight_set;
                *fresh9 = weight_set;
                (*arg.offset(b as isize)).weight_set_size = num_positions as U32;
                weight_set = weight_set.offset(position as isize);
                memcpy(
                    ids as *mut ffi::c_void,
                    (*bucket).h.items as *const ffi::c_void,
                    (::core::mem::size_of::<ffi::c_int>() as ffi::c_ulong)
                        .wrapping_mul((*bucket).h.size as ffi::c_ulong),
                );
                let fresh10 = &mut (*arg.offset(b as isize)).ids;
                *fresh10 = ids;
                (*arg.offset(b as isize)).ids_size = (*bucket).h.size;
                ids = ids.offset((*bucket).h.size as isize);
            }
            b += 1;
        }
        if weight_set_ends == weight_set as *mut ffi::c_char {
        } else {
            panic!("Assertion failed: !((char*)weight_set_ends != (char*)weight_set)");
        }

        if weights_end == weights as *mut ffi::c_char {
        } else {
            panic!("Assertion failed: !((char*)weights_end != (char*)weights)");
        }

        if ids as *mut ffi::c_char == ids_end {
        } else {
            panic!("Assertion failed: !((char*)ids != (char*)ids_end)");
        }

        arg
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn crush_destroy_choose_args(args: *mut CrushChooseArg) {
    unsafe {
        free(args as *mut ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn crush_addition_is_unsafe(a: U32, b: U32) -> ffi::c_int {
    if U32::MAX.wrapping_sub(b) < a {
        1
    } else {
        0
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn crush_multiplication_is_unsafe(a: U32, b: U32) -> ffi::c_int {
    if a == 0 {
        return 0;
    }
    if b == 0 {
        return 1;
    }
    if U32::MAX / b < a {
        1
    } else {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_legacy_crush_map(map: *mut CrushMap) {
    (*map).choose_local_tries = 2;
    (*map).choose_local_fallback_tries = 5;
    (*map).choose_total_tries = 19;
    (*map).chooseleaf_descend_once = 0;
    (*map).chooseleaf_vary_r = 0;
    (*map).chooseleaf_stable = 0;
    (*map).straw_calc_version = 0;
    (*map).allowed_bucket_algs = ((1 << CRUSH_BUCKET_UNIFORM as ffi::c_int)
        | (1 << CRUSH_BUCKET_LIST as ffi::c_int)
        | (1 << CRUSH_BUCKET_STRAW as ffi::c_int))
        as U32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_optimal_crush_map(map: *mut CrushMap) {
    (*map).choose_local_tries = 0;
    (*map).choose_local_fallback_tries = 0;
    (*map).choose_total_tries = 50;
    (*map).chooseleaf_descend_once = 1;
    (*map).chooseleaf_vary_r = 1;
    (*map).chooseleaf_stable = 1;
    (*map).allowed_bucket_algs = ((1 << CRUSH_BUCKET_UNIFORM as ffi::c_int)
        | (1 << CRUSH_BUCKET_LIST as ffi::c_int)
        | (1 << CRUSH_BUCKET_STRAW as ffi::c_int)
        | (1 << CRUSH_BUCKET_STRAW2 as ffi::c_int))
        as U32;
}
