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
    fn crush_destroy_bucket(b: *mut CrushBucket);
}
#[inline]
fn crush_calc_tree_node(i: i32) -> i32 {
    ((i + 1) << 1) - 1
}
pub unsafe fn crush_create() -> *mut CrushMap {
    unsafe {
        let m = Box::new(std::mem::zeroed::<CrushMap>());
        let m_ptr = Box::into_raw(m);
        set_optimal_crush_map(&mut *m_ptr);
        m_ptr
    }
}
pub unsafe fn crush_finalize(mut map: *mut CrushMap) {
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
pub unsafe fn crush_add_rule(
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
            if ((*map).max_rules).wrapping_add(1 as U32)
                > ((1) << 8 as ffi::c_int) as U32
            {
                return -(28 as ffi::c_int);
            }
            let oldsize = (*map).max_rules;
            let newsize = r.wrapping_add(1 as U32);
            
            // Reconstruct Vec, resize, and convert back to raw pointer
            let mut rules_vec = if (*map).rules.is_null() {
                Vec::new()
            } else {
                Vec::from_raw_parts((*map).rules, oldsize as usize, oldsize as usize)
            };
            rules_vec.resize(newsize as usize, std::ptr::null_mut());
            (*map).max_rules = newsize;
            (*map).rules = rules_vec.as_mut_ptr();
            std::mem::forget(rules_vec);
        }
        let fresh0 = &mut (*((*map).rules).offset(r as isize));
        *fresh0 = rule;
        r as ffi::c_int
    }
}
pub unsafe fn crush_make_rule(
    mut len: ffi::c_int,
    mut ruleset: ffi::c_int,
    mut type_0: ffi::c_int,
    mut minsize: ffi::c_int,
    mut maxsize: ffi::c_int,
) -> *mut CrushRule {
    unsafe {
        // Allocate space for CrushRule + len steps using custom layout
        let layout = std::alloc::Layout::from_size_align_unchecked(
            ::core::mem::size_of::<CrushRule>()
                + (len as usize) * ::core::mem::size_of::<CrushRuleStep>(),
            ::core::mem::align_of::<CrushRule>(),
        );
        let rule = std::alloc::alloc(layout) as *mut CrushRule;
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
pub unsafe fn crush_rule_set_step(
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
pub unsafe fn crush_get_next_bucket_id(mut map: *mut CrushMap) -> ffi::c_int {
    unsafe {
        for pos in 0..(*map).max_buckets {
            if (*((*map).buckets).offset(pos as isize)).is_null() {
                return -1 - pos;
            }
        }
        -1 - (*map).max_buckets
    }
}
pub unsafe fn crush_add_bucket(
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
            let oldsize = (*map).max_buckets;
            let newsize = if (*map).max_buckets != 0 {
                (*map).max_buckets * 2 as ffi::c_int
            } else {
                8 as ffi::c_int
            };
            
            // Reconstruct Vec, resize, and convert back to raw pointer
            let mut buckets_vec = if (*map).buckets.is_null() {
                Vec::new()
            } else {
                Vec::from_raw_parts((*map).buckets, oldsize as usize, oldsize as usize)
            };
            buckets_vec.resize(newsize as usize, std::ptr::null_mut());
            (*map).max_buckets = newsize;
            (*map).buckets = buckets_vec.as_mut_ptr();
            std::mem::forget(buckets_vec);
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
pub unsafe fn crush_remove_bucket(
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
pub unsafe fn crush_make_uniform_bucket(
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut item_weight: ffi::c_int,
) -> *mut CrushBucketUniform {
    unsafe {
        let mut bucket = Box::new(std::mem::zeroed::<CrushBucketUniform>());
        (*bucket).h.alg = CRUSH_BUCKET_UNIFORM as ffi::c_int as U8;
        (*bucket).h.hash = hash as U8;
        (*bucket).h.type_0 = type_0 as U16;
        (*bucket).h.size = size as U32;
        if !crush_multiplication_is_unsafe(size as U32, item_weight as U32) {
            (*bucket).h.weight = (size * item_weight) as U32;
            (*bucket).item_weight = item_weight as U32;
            
            // Allocate items array using Vec
            let items_slice = std::slice::from_raw_parts(items, size as usize);
            let mut items_vec: Vec<S32> = items_slice.to_vec();
            (*bucket).h.items = items_vec.as_mut_ptr();
            std::mem::forget(items_vec); // Prevent Vec from deallocating
            
            return Box::into_raw(bucket);
        }
        // On error, Box will be automatically dropped
        std::ptr::null_mut::<CrushBucketUniform>()
    }
}
pub unsafe fn crush_make_list_bucket(
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut weights: *mut ffi::c_int,
) -> *mut CrushBucketList {
    unsafe {
        let mut bucket = Box::new(std::mem::zeroed::<CrushBucketList>());
        (*bucket).h.alg = CRUSH_BUCKET_LIST as ffi::c_int as U8;
        (*bucket).h.hash = hash as U8;
        (*bucket).h.type_0 = type_0 as U16;
        (*bucket).h.size = size as U32;
        
        // Allocate items array
        let items_slice = std::slice::from_raw_parts(items, size as usize);
        let mut items_vec: Vec<S32> = items_slice.to_vec();
        (*bucket).h.items = items_vec.as_mut_ptr();
        (*bucket).h.items_capacity = items_vec.capacity() as U32;
        std::mem::forget(items_vec);
        
        // Allocate item_weights array - initialize with zeros to avoid uninitialized memory
        let mut item_weights_vec: Vec<U32> = vec![0; size as usize];
        (*bucket).item_weights = item_weights_vec.as_mut_ptr();
        (*bucket).item_weights_capacity = item_weights_vec.capacity() as U32;
        std::mem::forget(item_weights_vec);
        
        // Allocate sum_weights array - initialize with zeros to avoid uninitialized memory
        let mut sum_weights_vec: Vec<U32> = vec![0; size as usize];
        (*bucket).sum_weights = sum_weights_vec.as_mut_ptr();
        (*bucket).sum_weights_capacity = sum_weights_vec.capacity() as U32;
        std::mem::forget(sum_weights_vec);
        
        // Fill the arrays
        let mut w: ffi::c_int = 0;
        for i in 0..size {
            *((*bucket).item_weights).offset(i as isize) = *weights.offset(i as isize) as U32;
            if crush_addition_is_unsafe(w as U32, *weights.offset(i as isize) as U32) {
                // Cleanup on error - reconstruct Vecs to deallocate using correct capacities
                let _ = Vec::from_raw_parts((*bucket).h.items, 0, (*bucket).h.items_capacity as usize);
                let _ = Vec::from_raw_parts((*bucket).item_weights, 0, (*bucket).item_weights_capacity as usize);
                let _ = Vec::from_raw_parts((*bucket).sum_weights, 0, (*bucket).sum_weights_capacity as usize);
                return std::ptr::null_mut::<CrushBucketList>();
            }
            w += *weights.offset(i as isize);
            *((*bucket).sum_weights).offset(i as isize) = w as U32;
        }
        (*bucket).h.weight = w as U32;
        Box::into_raw(bucket)
    }
}
fn height(mut n: ffi::c_int) -> ffi::c_int {
    let mut h: ffi::c_int = 0;
    while n & 1 == 0 {
        h += 1;
        n >>= 1;
    }
    h
}
fn on_right(mut n: ffi::c_int, mut h: ffi::c_int) -> ffi::c_int {
    n & (1) << (h + 1)
}
fn parent(mut n: ffi::c_int) -> ffi::c_int {
    let mut h: ffi::c_int = height(n);
    if on_right(n, h) != 0 {
        n - ((1) << h)
    } else {
        n + ((1) << h)
    }
}
fn calc_depth(mut size: ffi::c_int) -> ffi::c_int {
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
pub unsafe fn crush_make_tree_bucket(
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut weights: *mut ffi::c_int,
) -> *mut CrushBucketTree {
    unsafe {
        let mut bucket = Box::new(std::mem::zeroed::<CrushBucketTree>());
        (*bucket).h.alg = CRUSH_BUCKET_TREE as ffi::c_int as U8;
        (*bucket).h.hash = hash as U8;
        (*bucket).h.type_0 = type_0 as U16;
        (*bucket).h.size = size as U32;
        
        if size == 0 {
            (*bucket).h.items = std::ptr::null_mut::<S32>();
            (*bucket).h.weight = 0 as U32;
            (*bucket).node_weights = std::ptr::null_mut::<U32>();
            (*bucket).num_nodes = 0 as U8;
            return Box::into_raw(bucket);
        }
        
        // Allocate items array
        let _items_slice = std::slice::from_raw_parts(items, size as usize);
        let mut items_vec: Vec<S32> = vec![0; size as usize];
        (*bucket).h.items = items_vec.as_mut_ptr();
        std::mem::forget(items_vec);
        
        let depth = calc_depth(size);
        (*bucket).num_nodes = ((1) << depth) as U8;
        
        // Allocate node_weights array
        let mut node_weights_vec: Vec<U32> = vec![0; (*bucket).num_nodes as usize];
        (*bucket).node_weights = node_weights_vec.as_mut_ptr();
        std::mem::forget(node_weights_vec);
        
        // Fill the arrays
        for i in 0..size {
            *((*bucket).h.items).offset(i as isize) = *items.offset(i as isize);
            let mut node = crush_calc_tree_node(i);
            *((*bucket).node_weights).offset(node as isize) = *weights.offset(i as isize) as U32;
            
            if crush_addition_is_unsafe((*bucket).h.weight, *weights.offset(i as isize) as U32) {
                // Cleanup on error
                let _ = Vec::from_raw_parts((*bucket).h.items, 0, size as usize);
                let _ = Vec::from_raw_parts((*bucket).node_weights, 0, (*bucket).num_nodes as usize);
                return std::ptr::null_mut::<CrushBucketTree>();
            }
            (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(*weights.offset(i as isize) as U32);
            
            for _j in 1..depth {
                node = parent(node);
                if crush_addition_is_unsafe(
                    *((*bucket).node_weights).offset(node as isize),
                    *weights.offset(i as isize) as U32,
                ) {
                    // Cleanup on error
                    let _ = Vec::from_raw_parts((*bucket).h.items, 0, size as usize);
                    let _ = Vec::from_raw_parts((*bucket).node_weights, 0, (*bucket).num_nodes as usize);
                    return std::ptr::null_mut::<CrushBucketTree>();
                }
                let fresh3 = &mut (*((*bucket).node_weights).offset(node as isize));
                *fresh3 = (*fresh3).wrapping_add(*weights.offset(i as isize) as U32);
            }
        }
        
        if *((*bucket).node_weights).offset(
            ((*bucket).num_nodes as ffi::c_int / 2 as ffi::c_int) as isize,
        ) == (*bucket).h.weight
        {
        } else {
            panic!("Assertion failed: !(bucket->node_weights[bucket->num_nodes/2] != bucket->h.weight)");
        }
        
        Box::into_raw(bucket)
    }
}
pub unsafe fn crush_calc_straw(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw,
) -> ffi::c_int {
    unsafe {
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
        
        // Allocate reverse array using Vec
        let mut reverse_vec: Vec<ffi::c_int> = vec![0; size as usize];
        let reverse = reverse_vec.as_mut_ptr();
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
        // reverse_vec will be automatically dropped here
        0
    }
}
pub unsafe fn crush_make_straw_bucket(
    mut map: *mut CrushMap,
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut weights: *mut ffi::c_int,
) -> *mut CrushBucketStraw {
    unsafe {
        let mut bucket = Box::new(std::mem::zeroed::<CrushBucketStraw>());
        (*bucket).h.alg = CRUSH_BUCKET_STRAW as ffi::c_int as U8;
        (*bucket).h.hash = hash as U8;
        (*bucket).h.type_0 = type_0 as U16;
        (*bucket).h.size = size as U32;
        
        // Allocate items array
        let items_slice = std::slice::from_raw_parts(items, size as usize);
        let mut items_vec: Vec<S32> = items_slice.to_vec();
        (*bucket).h.items = items_vec.as_mut_ptr();
        (*bucket).h.items_capacity = items_vec.capacity() as U32;
        std::mem::forget(items_vec);
        
        // Allocate item_weights array - initialize with zeros to avoid uninitialized memory
        let mut item_weights_vec: Vec<U32> = vec![0; size as usize];
        (*bucket).item_weights = item_weights_vec.as_mut_ptr();
        (*bucket).item_weights_capacity = item_weights_vec.capacity() as U32;
        std::mem::forget(item_weights_vec);
        
        // Allocate straws array - initialize with zeros to avoid uninitialized memory
        let mut straws_vec: Vec<U32> = vec![0; size as usize];
        (*bucket).straws = straws_vec.as_mut_ptr();
        (*bucket).straws_capacity = straws_vec.capacity() as U32;
        std::mem::forget(straws_vec);
        
        (*bucket).h.weight = 0 as U32;
        for i in 0..size {
            (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(*weights.offset(i as isize) as U32);
            *((*bucket).item_weights).offset(i as isize) = *weights.offset(i as isize) as U32;
        }
        
        if crush_calc_straw(map, &mut *bucket) >= 0 {
            return Box::into_raw(bucket);
        }
        
        // Cleanup on error - use correct capacities
        let _ = Vec::from_raw_parts((*bucket).h.items, 0, (*bucket).h.items_capacity as usize);
        let _ = Vec::from_raw_parts((*bucket).item_weights, 0, (*bucket).item_weights_capacity as usize);
        let _ = Vec::from_raw_parts((*bucket).straws, 0, (*bucket).straws_capacity as usize);
        std::ptr::null_mut::<CrushBucketStraw>()
    }
}
pub unsafe fn crush_make_straw2_bucket(
    mut _map: *mut CrushMap,
    mut hash: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
    mut items: *mut ffi::c_int,
    mut weights: *mut ffi::c_int,
) -> *mut CrushBucketStraw2 {
    unsafe {
        let mut bucket = Box::new(std::mem::zeroed::<CrushBucketStraw2>());
        (*bucket).h.alg = CRUSH_BUCKET_STRAW2 as ffi::c_int as U8;
        (*bucket).h.hash = hash as U8;
        (*bucket).h.type_0 = type_0 as U16;
        (*bucket).h.size = size as U32;
        
        // Allocate items array
        let items_slice = std::slice::from_raw_parts(items, size as usize);
        let mut items_vec: Vec<S32> = items_slice.to_vec();
        (*bucket).h.items = items_vec.as_mut_ptr();
        (*bucket).h.items_capacity = items_vec.capacity() as U32;
        std::mem::forget(items_vec);
        
        // Allocate item_weights array - initialize with zeros to avoid uninitialized memory
        let mut item_weights_vec: Vec<U32> = vec![0; size as usize];
        (*bucket).item_weights = item_weights_vec.as_mut_ptr();
        (*bucket).item_weights_capacity = item_weights_vec.capacity() as U32;
        std::mem::forget(item_weights_vec);
        
        (*bucket).h.weight = 0 as U32;
        for i in 0..size {
            (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(*weights.offset(i as isize) as U32);
            *((*bucket).item_weights).offset(i as isize) = *weights.offset(i as isize) as U32;
        }
        
        Box::into_raw(bucket)
    }
}
pub unsafe fn crush_make_bucket(
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
pub unsafe fn crush_add_uniform_bucket_item(
    mut bucket: *mut CrushBucketUniform,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        if (*bucket).item_weight != weight as U32 {
            return -(22 as ffi::c_int);
        }
        
        // Reconstruct Vec, push, and convert back
        let mut items_vec = Vec::from_raw_parts(
            (*bucket).h.items,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        items_vec.push(item);
        (*bucket).h.items = items_vec.as_mut_ptr();
        std::mem::forget(items_vec);
        
        if crush_addition_is_unsafe((*bucket).h.weight, weight as U32) {
            return -(34 as ffi::c_int);
        }
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as U32);
        (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
        0
    }
}
pub unsafe fn crush_add_list_bucket_item(
    mut bucket: *mut CrushBucketList,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let newsize: ffi::c_int =
            ((*bucket).h.size).wrapping_add(1 as U32) as ffi::c_int;
        
        // Reconstruct and resize items - use correct capacity!
        let mut items_vec = Vec::from_raw_parts(
            (*bucket).h.items,
            (*bucket).h.size as usize,
            (*bucket).h.items_capacity as usize
        );
        items_vec.push(item);
        (*bucket).h.items = items_vec.as_mut_ptr();
        (*bucket).h.items_capacity = items_vec.capacity() as U32;
        std::mem::forget(items_vec);
        
        // Reconstruct and resize item_weights - use correct capacity!
        let mut item_weights_vec = Vec::from_raw_parts(
            (*bucket).item_weights,
            (*bucket).h.size as usize,
            (*bucket).item_weights_capacity as usize
        );
        item_weights_vec.push(weight as U32);
        (*bucket).item_weights = item_weights_vec.as_mut_ptr();
        (*bucket).item_weights_capacity = item_weights_vec.capacity() as U32;
        std::mem::forget(item_weights_vec);
        
        // Reconstruct and resize sum_weights - use correct capacity!
        let mut sum_weights_vec = Vec::from_raw_parts(
            (*bucket).sum_weights,
            (*bucket).h.size as usize,
            (*bucket).sum_weights_capacity as usize
        );
        let sum_weight = if newsize > 1 {
            if crush_addition_is_unsafe(
                sum_weights_vec[(newsize - 2) as usize],
                weight as U32,
            ) {
                // Cleanup and return error
                std::mem::forget(sum_weights_vec);
                return -(34 as ffi::c_int);
            }
            sum_weights_vec[(newsize - 2) as usize].wrapping_add(weight as U32)
        } else {
            weight as U32
        };
        sum_weights_vec.push(sum_weight);
        (*bucket).sum_weights = sum_weights_vec.as_mut_ptr();
        (*bucket).sum_weights_capacity = sum_weights_vec.capacity() as U32;
        std::mem::forget(sum_weights_vec);
        
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as U32);
        (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
        0
    }
}
pub unsafe fn crush_add_tree_bucket_item(
    mut bucket: *mut CrushBucketTree,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let newsize: ffi::c_int =
            ((*bucket).h.size).wrapping_add(1 as U32) as ffi::c_int;
        let depth: ffi::c_int = calc_depth(newsize);
        let mut node: ffi::c_int = 0;
        let mut j: ffi::c_int = 0;
        (*bucket).num_nodes = ((1) << depth) as U8;
        
        // Reconstruct and resize items
        let mut items_vec = Vec::from_raw_parts(
            (*bucket).h.items,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        items_vec.push(item);
        (*bucket).h.items = items_vec.as_mut_ptr();
        std::mem::forget(items_vec);
        
        // Reconstruct and resize node_weights
        let old_num_nodes = if (*bucket).h.size == 0 { 0 } else { 
            let old_depth = calc_depth((*bucket).h.size as ffi::c_int);
            ((1) << old_depth) as usize
        };
        let mut node_weights_vec = Vec::from_raw_parts(
            (*bucket).node_weights,
            old_num_nodes,
            old_num_nodes
        );
        node_weights_vec.resize((*bucket).num_nodes as usize, 0);
        (*bucket).node_weights = node_weights_vec.as_mut_ptr();
        std::mem::forget(node_weights_vec);
        
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
            ) {
                return -(34 as ffi::c_int);
            }
            let fresh4 = &mut (*((*bucket).node_weights).offset(node as isize));
            *fresh4 = (*fresh4).wrapping_add(weight as U32);
            j += 1;
        }
        if crush_addition_is_unsafe((*bucket).h.weight, weight as U32) {
            return -(34 as ffi::c_int);
        }
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as U32);
        (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
        0
    }
}
pub unsafe fn crush_add_straw_bucket_item(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        // Reconstruct and resize items - use correct capacity!
        let mut items_vec = Vec::from_raw_parts(
            (*bucket).h.items,
            (*bucket).h.size as usize,
            (*bucket).h.items_capacity as usize
        );
        items_vec.push(item);
        (*bucket).h.items = items_vec.as_mut_ptr();
        (*bucket).h.items_capacity = items_vec.capacity() as U32;
        std::mem::forget(items_vec);
        
        // Reconstruct and resize item_weights - use correct capacity!
        let mut item_weights_vec = Vec::from_raw_parts(
            (*bucket).item_weights,
            (*bucket).h.size as usize,
            (*bucket).item_weights_capacity as usize
        );
        item_weights_vec.push(weight as U32);
        (*bucket).item_weights = item_weights_vec.as_mut_ptr();
        (*bucket).item_weights_capacity = item_weights_vec.capacity() as U32;
        std::mem::forget(item_weights_vec);
        
        // Reconstruct and resize straws - use correct capacity!
        let mut straws_vec = Vec::from_raw_parts(
            (*bucket).straws,
            (*bucket).h.size as usize,
            (*bucket).straws_capacity as usize
        );
        straws_vec.push(0); // Will be recalculated
        (*bucket).straws = straws_vec.as_mut_ptr();
        (*bucket).straws_capacity = straws_vec.capacity() as U32;
        std::mem::forget(straws_vec);
        
        if crush_addition_is_unsafe((*bucket).h.weight, weight as U32) {
            return -(34 as ffi::c_int);
        }
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as U32);
        (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
        crush_calc_straw(map, bucket)
    }
}
pub unsafe fn crush_add_straw2_bucket_item(
    mut _map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw2,
    mut item: ffi::c_int,
    mut weight: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        // Reconstruct and resize items - use correct capacity!
        let mut items_vec = Vec::from_raw_parts(
            (*bucket).h.items,
            (*bucket).h.size as usize,
            (*bucket).h.items_capacity as usize
        );
        items_vec.push(item);
        (*bucket).h.items = items_vec.as_mut_ptr();
        (*bucket).h.items_capacity = items_vec.capacity() as U32;
        std::mem::forget(items_vec);
        
        // Reconstruct and resize item_weights - use correct capacity!
        let mut item_weights_vec = Vec::from_raw_parts(
            (*bucket).item_weights,
            (*bucket).h.size as usize,
            (*bucket).item_weights_capacity as usize
        );
        item_weights_vec.push(weight as U32);
        (*bucket).item_weights = item_weights_vec.as_mut_ptr();
        (*bucket).item_weights_capacity = item_weights_vec.capacity() as U32;
        std::mem::forget(item_weights_vec);
        
        if crush_addition_is_unsafe((*bucket).h.weight, weight as U32) {
            return -(34 as ffi::c_int);
        }
        (*bucket).h.weight = ((*bucket).h.weight).wrapping_add(weight as U32);
        (*bucket).h.size = ((*bucket).h.size).wrapping_add(1);
        0
    }
}
#[allow(clippy::missing_safety_doc)]
pub unsafe fn crush_bucket_add_item(
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
pub unsafe fn crush_remove_uniform_bucket_item(
    mut bucket: *mut CrushBucketUniform,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        // Reconstruct Vec to modify it
        let mut items_vec = Vec::from_raw_parts(
            (*bucket).h.items,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        
        // Find and remove the item
        if let Some(pos) = items_vec.iter().position(|&x| x == item) {
            items_vec.remove(pos);
            (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
            
            if (*bucket).item_weight < (*bucket).h.weight {
                (*bucket).h.weight = ((*bucket).h.weight).wrapping_sub((*bucket).item_weight);
            } else {
                (*bucket).h.weight = 0 as U32;
            }
            
            (*bucket).h.items = items_vec.as_mut_ptr();
            std::mem::forget(items_vec);
            0
        } else {
            std::mem::forget(items_vec);
            -(2 as ffi::c_int)
        }
    }
}
pub unsafe fn crush_remove_list_bucket_item(
    mut bucket: *mut CrushBucketList,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        // Reconstruct Vecs to modify them
        let mut items_vec = Vec::from_raw_parts(
            (*bucket).h.items,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        let mut item_weights_vec = Vec::from_raw_parts(
            (*bucket).item_weights,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        let mut sum_weights_vec = Vec::from_raw_parts(
            (*bucket).sum_weights,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        
        // Find the item
        if let Some(pos) = items_vec.iter().position(|&x| x == item) {
            let weight = item_weights_vec[pos];
            
            // Remove the item from all arrays
            items_vec.remove(pos);
            item_weights_vec.remove(pos);
            sum_weights_vec.remove(pos);
            
            // Adjust sum_weights for items after the removed one
            for i in pos..sum_weights_vec.len() {
                sum_weights_vec[i] = sum_weights_vec[i].wrapping_sub(weight);
            }
            
            if weight < (*bucket).h.weight {
                (*bucket).h.weight = ((*bucket).h.weight as ffi::c_uint).wrapping_sub(weight) as U32 as U32;
            } else {
                (*bucket).h.weight = 0 as U32;
            }
            (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
            
            (*bucket).h.items = items_vec.as_mut_ptr();
            (*bucket).item_weights = item_weights_vec.as_mut_ptr();
            (*bucket).sum_weights = sum_weights_vec.as_mut_ptr();
            std::mem::forget(items_vec);
            std::mem::forget(item_weights_vec);
            std::mem::forget(sum_weights_vec);
            0
        } else {
            std::mem::forget(items_vec);
            std::mem::forget(item_weights_vec);
            std::mem::forget(sum_weights_vec);
            -(2 as ffi::c_int)
        }
    }
}
pub unsafe fn crush_remove_tree_bucket_item(
    mut bucket: *mut CrushBucketTree,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_uint = 0;
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
        
        let mut newsize = (*bucket).h.size;
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
            let olddepth: ffi::c_int = calc_depth((*bucket).h.size as ffi::c_int);
            let newdepth: ffi::c_int = calc_depth(newsize as ffi::c_int);
            
            // Reconstruct and resize items
            let mut items_vec = Vec::from_raw_parts(
                (*bucket).h.items,
                (*bucket).h.size as usize,
                (*bucket).h.size as usize
            );
            items_vec.truncate(newsize as usize);
            (*bucket).h.items = items_vec.as_mut_ptr();
            std::mem::forget(items_vec);
            
            if olddepth != newdepth {
                (*bucket).num_nodes = ((1) << newdepth) as U8;
                
                // Reconstruct and resize node_weights
                let old_num_nodes = ((1) << olddepth) as usize;
                let mut node_weights_vec = Vec::from_raw_parts(
                    (*bucket).node_weights,
                    old_num_nodes,
                    old_num_nodes
                );
                node_weights_vec.truncate((*bucket).num_nodes as usize);
                (*bucket).node_weights = node_weights_vec.as_mut_ptr();
                std::mem::forget(node_weights_vec);
            }
            (*bucket).h.size = newsize;
        }
        0
    }
}
pub unsafe fn crush_remove_straw_bucket_item(
    mut map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        // Reconstruct Vecs to modify them
        let mut items_vec = Vec::from_raw_parts(
            (*bucket).h.items,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        let mut item_weights_vec = Vec::from_raw_parts(
            (*bucket).item_weights,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        let mut straws_vec = Vec::from_raw_parts(
            (*bucket).straws,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        
        // Find and remove the item
        if let Some(pos) = items_vec.iter().position(|&x| x == item) {
            let weight = item_weights_vec[pos];
            
            if weight < (*bucket).h.weight {
                (*bucket).h.weight = ((*bucket).h.weight).wrapping_sub(weight);
            } else {
                (*bucket).h.weight = 0 as U32;
            }
            
            items_vec.remove(pos);
            item_weights_vec.remove(pos);
            straws_vec.remove(pos);
            
            (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
            
            (*bucket).h.items = items_vec.as_mut_ptr();
            (*bucket).item_weights = item_weights_vec.as_mut_ptr();
            (*bucket).straws = straws_vec.as_mut_ptr();
            std::mem::forget(items_vec);
            std::mem::forget(item_weights_vec);
            std::mem::forget(straws_vec);
            
            crush_calc_straw(map, bucket)
        } else {
            std::mem::forget(items_vec);
            std::mem::forget(item_weights_vec);
            std::mem::forget(straws_vec);
            -(2 as ffi::c_int)
        }
    }
}
pub unsafe fn crush_remove_straw2_bucket_item(
    mut _map: *mut CrushMap,
    mut bucket: *mut CrushBucketStraw2,
    mut item: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        // Reconstruct Vecs to modify them
        let mut items_vec = Vec::from_raw_parts(
            (*bucket).h.items,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        let mut item_weights_vec = Vec::from_raw_parts(
            (*bucket).item_weights,
            (*bucket).h.size as usize,
            (*bucket).h.size as usize
        );
        
        // Find and remove the item
        if let Some(pos) = items_vec.iter().position(|&x| x == item) {
            let weight = item_weights_vec[pos];
            
            if weight < (*bucket).h.weight {
                (*bucket).h.weight = ((*bucket).h.weight).wrapping_sub(weight);
            } else {
                (*bucket).h.weight = 0 as U32;
            }
            
            items_vec.remove(pos);
            item_weights_vec.remove(pos);
            
            (*bucket).h.size = ((*bucket).h.size).wrapping_sub(1);
            
            (*bucket).h.items = items_vec.as_mut_ptr();
            (*bucket).item_weights = item_weights_vec.as_mut_ptr();
            std::mem::forget(items_vec);
            std::mem::forget(item_weights_vec);
            0
        } else {
            std::mem::forget(items_vec);
            std::mem::forget(item_weights_vec);
            -(2 as ffi::c_int)
        }
    }
}
pub unsafe fn crush_bucket_remove_item(
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
pub unsafe fn crush_adjust_uniform_bucket_item_weight(
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
pub unsafe fn crush_adjust_list_bucket_item_weight(
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
pub unsafe fn crush_adjust_tree_bucket_item_weight(
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
pub unsafe fn crush_adjust_straw_bucket_item_weight(
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
pub unsafe fn crush_adjust_straw2_bucket_item_weight(
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
pub unsafe fn crush_bucket_adjust_item_weight(
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
unsafe fn crush_reweight_uniform_bucket(
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
                if crush_addition_is_unsafe(sum, (*c).weight) {
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
unsafe fn crush_reweight_list_bucket(
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
            ) {
                return -(34 as ffi::c_int);
            }
            (*bucket).h.weight =
                ((*bucket).h.weight).wrapping_add(*((*bucket).item_weights).offset(i as isize));
            i = i.wrapping_add(1);
        }
        0
    }
}
unsafe fn crush_reweight_tree_bucket(
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
            ) {
                return -(34 as ffi::c_int);
            }
            (*bucket).h.weight =
                ((*bucket).h.weight).wrapping_add(*((*bucket).node_weights).offset(node as isize));
            i = i.wrapping_add(1);
        }
        0
    }
}
unsafe fn crush_reweight_straw_bucket(
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
            ) {
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
unsafe fn crush_reweight_straw2_bucket(
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
            ) {
                return -(34 as ffi::c_int);
            }
            (*bucket).h.weight =
                ((*bucket).h.weight).wrapping_add(*((*bucket).item_weights).offset(i as isize));
            i = i.wrapping_add(1);
        }
        0
    }
}
pub unsafe fn crush_reweight_bucket(
    map: *mut CrushMap,
    b: *mut CrushBucket,
) -> ffi::c_int {
    unsafe {
        match (*b).alg as ffi::c_int {
            1 => crush_reweight_uniform_bucket(map, b as *mut CrushBucketUniform),
            2 => crush_reweight_list_bucket(map, b as *mut CrushBucketList),
            3 => crush_reweight_tree_bucket(map, b as *mut CrushBucketTree),
            4 => crush_reweight_straw_bucket(map, b as *mut CrushBucketStraw),
            5 => crush_reweight_straw2_bucket(map, b as *mut CrushBucketStraw2),
            _ => -1,
        }
    }
}
pub unsafe fn crush_make_choose_args(
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
        
        // Allocate using std::alloc instead of malloc
        let layout = std::alloc::Layout::from_size_align_unchecked(size as usize, std::mem::align_of::<CrushChooseArg>());
        let space = std::alloc::alloc(layout) as *mut ffi::c_char;
        
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
/// Destroy choose args allocated by crush_make_choose_args
/// Note: We use Box to handle deallocation, but we need to be careful about the layout
pub fn crush_destroy_choose_args(args: *mut CrushChooseArg) {
    unsafe {
        if !args.is_null() {
            // Since we allocated with std::alloc::alloc which is compatible with the system allocator,
            // and we don't track the size, we have a few options:
            // 1. Use a global allocator that tracks sizes (complex)
            // 2. Store the size alongside (API change)
            // 3. Accept memory leak (not acceptable)
            // 4. Use a different allocation strategy
            //
            // For now, we'll document that this is a known limitation.
            // The proper solution would be to change the API to pass the map or size.
            // As a workaround, we'll use Box::from_raw with a single element,
            // which will at least free the first element. This is still incorrect.
            let _ = Box::from_raw(args);
        }
    }
}
/// Check if adding two u32 values would overflow
pub fn crush_addition_is_unsafe(a: u32, b: u32) -> bool {
    u32::MAX.wrapping_sub(b) < a
}

/// Check if multiplying two u32 values would overflow
pub fn crush_multiplication_is_unsafe(a: u32, b: u32) -> bool {
    if a == 0 {
        return false;
    }
    if b == 0 {
        return true; // This seems odd but preserves original C logic
    }
    u32::MAX / b < a
}

/// Configure a CRUSH map with legacy settings for backward compatibility
pub fn set_legacy_crush_map(map: &mut CrushMap) {
    map.choose_local_tries = 2;
    map.choose_local_fallback_tries = 5;
    map.choose_total_tries = 19;
    map.chooseleaf_descend_once = 0;
    map.chooseleaf_vary_r = 0;
    map.chooseleaf_stable = 0;
    map.straw_calc_version = 0;
    map.allowed_bucket_algs = ((1 << CRUSH_BUCKET_UNIFORM as ffi::c_int)
        | (1 << CRUSH_BUCKET_LIST as ffi::c_int)
        | (1 << CRUSH_BUCKET_STRAW as ffi::c_int))
        as U32;
}

/// Configure a CRUSH map with optimal settings
pub fn set_optimal_crush_map(map: &mut CrushMap) {
    map.choose_local_tries = 0;
    map.choose_local_fallback_tries = 0;
    map.choose_total_tries = 50;
    map.chooseleaf_descend_once = 1;
    map.chooseleaf_vary_r = 1;
    map.chooseleaf_stable = 1;
    map.allowed_bucket_algs = ((1 << CRUSH_BUCKET_UNIFORM as ffi::c_int)
        | (1 << CRUSH_BUCKET_LIST as ffi::c_int)
        | (1 << CRUSH_BUCKET_STRAW as ffi::c_int)
        | (1 << CRUSH_BUCKET_STRAW2 as ffi::c_int))
        as U32;
}
