#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::crush::helpers::c_slice;
use crate::crush::types::ffi;
use crate::crush::types::*;

unsafe extern "C" {
    fn free(_: *mut ffi::c_void);
}
#[inline]
fn crush_calc_tree_node(i: i32) -> i32 {
    ((i + 1) << 1) - 1
}
pub fn crush_bucket_alg_name(alg: i32) -> &'static str {
    match alg {
        1 => "uniform",
        2 => "list",
        3 => "tree",
        4 => "straw",
        5 => "straw2",
        _ => "unknown",
    }
}

/// Weight of the item at position `p` in bucket `b`, or 0 if `p` is out of range.
///
/// # Safety
/// `b` must point to a live bucket whose `alg` tag matches its concrete type
/// (`CrushBucketUniform`, `CrushBucketList`, ...) and whose weight arrays are
/// sized as that type requires.
pub unsafe fn crush_get_bucket_item_weight(b: *const CrushBucket, p: i32) -> i32 {
    // Safety: guaranteed by the caller.
    let header = unsafe { &*b };
    let Ok(p) = u32::try_from(p) else {
        return 0;
    };
    if p >= header.size {
        return 0;
    }
    let (p, size) = (p as usize, header.size as usize);
    // Safety: for each tag the caller guarantees the concrete bucket type, whose
    // weight array holds `size` entries (`num_nodes` for a tree bucket).
    unsafe {
        match header.alg {
            1 => (*(b as *const CrushBucketUniform)).item_weight as i32,
            2 => c_slice((*(b as *const CrushBucketList)).item_weights, size)[p] as i32,
            3 => {
                let tree = &*(b as *const CrushBucketTree);
                let nodes = c_slice(tree.node_weights, tree.num_nodes as usize);
                nodes
                    .get(crush_calc_tree_node(p as i32) as usize)
                    .map_or(0, |&w| w as i32)
            }
            4 => c_slice((*(b as *const CrushBucketStraw)).item_weights, size)[p] as i32,
            5 => c_slice((*(b as *const CrushBucketStraw2)).item_weights, size)[p] as i32,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wrapper::{BucketAlgorithm, Map};
    use std::ffi::CStr;

    /// Original c2rust-shaped implementations, kept as test oracles.
    unsafe fn crush_get_bucket_item_weight_unsafe_oracle(
        b: *const CrushBucket,
        p: ffi::c_int,
    ) -> ffi::c_int {
        unsafe {
            if p as U32 >= (*b).size {
                return 0;
            }
            match (*b).alg as ffi::c_int {
                1 => (*(b as *mut CrushBucketUniform)).item_weight as ffi::c_int,
                2 => {
                    *((*(b as *mut CrushBucketList)).item_weights).offset(p as isize) as ffi::c_int
                }
                3 => *((*(b as *mut CrushBucketTree)).node_weights)
                    .offset(crush_calc_tree_node(p) as isize) as ffi::c_int,
                4 => {
                    *((*(b as *mut CrushBucketStraw)).item_weights).offset(p as isize) as ffi::c_int
                }
                5 => *((*(b as *mut CrushBucketStraw2)).item_weights).offset(p as isize)
                    as ffi::c_int,
                _ => 0,
            }
        }
    }

    #[test]
    fn alg_names_match_c() {
        for alg in -2..9 {
            let c = unsafe { CStr::from_ptr(crush_sys::crush_bucket_alg_name(alg)) };
            assert_eq!(crush_bucket_alg_name(alg), c.to_str().unwrap());
        }
    }

    #[test]
    fn item_weights_match_oracle_and_c() {
        let items = [10, 11, 12, 13, 14];
        let weights = [0x10000, 0x20000, 0x8000, 0x30000, 0x1000];
        let mut m = Map::new();
        for alg in [
            BucketAlgorithm::Uniform,
            BucketAlgorithm::List,
            BucketAlgorithm::Tree,
            BucketAlgorithm::Straw,
            BucketAlgorithm::Straw2,
        ] {
            // Uniform buckets need equal weights.
            let w = if alg == BucketAlgorithm::Uniform {
                [0x10000; 5]
            } else {
                weights
            };
            let bucket = m.make_bucket(alg, 0, 1, &items, &w).unwrap();
            let b = bucket.as_ptr();
            for p in -2..8 {
                let safe = unsafe { crush_get_bucket_item_weight(b, p) };
                let oracle = unsafe { crush_get_bucket_item_weight_unsafe_oracle(b, p) };
                let c = unsafe {
                    crush_sys::crush_get_bucket_item_weight(b as *const crush_sys::crush_bucket, p)
                };
                assert_eq!(safe, oracle, "{alg:?} p={p}");
                assert_eq!(safe, c, "{alg:?} p={p}");
            }
        }
    }
}
