use crate::crush::builder::*;
use crate::crush::crush::*;
use crate::crush::helpers::crush_find_roots;
use crate::crush::mapper::{crush_do_rule, crush_init_workspace};
use crate::crush::types::*;

use std::ptr;
use std::slice;

pub use crate::crush::builder::crush_bucket_add_item;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BucketAlgorithm {
    Uniform,
    List,
    Tree,
    Straw,
    Straw2,
}

impl BucketAlgorithm {
    fn to_c(self) -> i32 {
        match self {
            BucketAlgorithm::Uniform => CRUSH_BUCKET_UNIFORM as i32,
            BucketAlgorithm::List => CRUSH_BUCKET_LIST as i32,
            BucketAlgorithm::Tree => CRUSH_BUCKET_TREE as i32,
            BucketAlgorithm::Straw => CRUSH_BUCKET_STRAW as i32,
            BucketAlgorithm::Straw2 => CRUSH_BUCKET_STRAW2 as i32,
        }
    }

    fn from_c(alg: u8) -> Option<Self> {
        match alg {
            x if x == CRUSH_BUCKET_UNIFORM as u8 => Some(BucketAlgorithm::Uniform),
            x if x == CRUSH_BUCKET_LIST as u8 => Some(BucketAlgorithm::List),
            x if x == CRUSH_BUCKET_TREE as u8 => Some(BucketAlgorithm::Tree),
            x if x == CRUSH_BUCKET_STRAW as u8 => Some(BucketAlgorithm::Straw),
            x if x == CRUSH_BUCKET_STRAW2 as u8 => Some(BucketAlgorithm::Straw2),
            _ => None,
        }
    }
}

// Wrapper for crush_map
#[repr(transparent)]
pub struct Map {
    pub ptr: *mut CrushMap,
}

impl Map {
    pub fn new() -> Self {
        unsafe {
            let ptr = crush_create();
            assert!(!ptr.is_null());
            Map { ptr }
        }
    }

    pub fn new_legacy() -> Self {
        let map = Self::new();
        unsafe {
            set_legacy_crush_map(map.ptr);
        }
        map
    }

    pub fn choose_local_tries(&self) -> u32 {
        unsafe { (*self.ptr).choose_local_tries }
    }

    pub fn choose_local_fallback_tries(&self) -> u32 {
        unsafe { (*self.ptr).choose_local_fallback_tries }
    }

    pub fn choose_total_tries(&self) -> u32 {
        unsafe { (*self.ptr).choose_total_tries }
    }

    pub fn chooseleaf_descend_once(&self) -> u32 {
        unsafe { (*self.ptr).chooseleaf_descend_once }
    }

    pub fn chooseleaf_vary_r(&self) -> u8 {
        unsafe { (*self.ptr).chooseleaf_vary_r }
    }

    pub fn chooseleaf_stable(&self) -> u8 {
        unsafe { (*self.ptr).chooseleaf_stable }
    }

    pub fn straw_calc_version(&self) -> u8 {
        unsafe { (*self.ptr).straw_calc_version }
    }

    pub fn allowed_bucket_algs(&self) -> u32 {
        unsafe { (*self.ptr).allowed_bucket_algs }
    }

    pub fn has_rules(&self) -> bool {
        unsafe { !(*self.ptr).rules.is_null() }
    }

    pub fn max_buckets(&self) -> i32 {
        unsafe { (*self.ptr).max_buckets }
    }

    pub fn make_bucket(
        &mut self,
        alg: BucketAlgorithm,
        hash: i32,
        type_: i32,
        items: &[i32],
        weights: &[i32],
    ) -> Result<Bucket, &'static str> {
        if items.len() != weights.len() {
            return Err("items and weights must have the same length");
        }

        unsafe {
            let ptr = crush_make_bucket(
                self.ptr,
                alg.to_c(),
                hash,
                type_,
                items.len() as i32,
                items.as_ptr() as *mut i32,
                weights.as_ptr() as *mut i32,
            );

            if ptr.is_null() {
                Err("Failed to create bucket")
            } else {
                Ok(Bucket {
                    ptr,
                    _map: ptr::null_mut(),
                })
            }
        }
    }

    pub fn add_bucket(&mut self, id: i32, bucket: Bucket) -> Result<i32, i32> {
        let mut bucketno = id;
        unsafe {
            let result = crush_add_bucket(self.ptr, id, bucket.ptr, &mut bucketno);
            if result == 0 {
                // Transfer ownership to the map
                std::mem::forget(bucket);
                Ok(bucketno)
            } else {
                Err(result)
            }
        }
    }

    pub fn find_roots(&self) -> Result<Vec<i32>, i32> {
        unsafe {
            let mut roots: *mut i32 = ptr::null_mut();
            let count = crush_find_roots(self.ptr, &mut roots);

            if count < 0 {
                Err(count)
            } else if count == 0 {
                Ok(vec![])
            } else {
                let result = slice::from_raw_parts(roots, count as usize).to_vec();
                libc::free(roots as *mut ffi::c_void);
                Ok(result)
            }
        }
    }

    pub fn finalize(&mut self) {
        unsafe {
            crush_finalize(self.ptr);
        }
    }

    pub fn make_choose_args(&self, num_positions: i32) -> ChooseArgs {
        unsafe {
            let ptr = crush_make_choose_args(self.ptr, num_positions);
            ChooseArgs {
                ptr,
                _num_positions: num_positions,
            }
        }
    }

    pub fn add_rule(&mut self, rule: Rule, ruleno: i32) -> i32 {
        unsafe {
            let result = crush_add_rule(self.ptr, rule.ptr, ruleno);
            std::mem::forget(rule); // Map now owns the rule
            result
        }
    }

    pub fn do_rule(
        &self,
        ruleno: i32,
        x: i32,
        result: &mut [i32],
        weights: &[u32],
        choose_args: Option<&ChooseArgs>,
    ) -> Result<usize, &'static str> {
        unsafe {
            // Implement crush_work_size inline function
            let cwin_size =
                (*self.ptr).working_size as usize + (result.len() * 3 * std::mem::size_of::<u32>());
            let mut cwin = vec![0u8; cwin_size];
            crush_init_workspace(self.ptr, cwin.as_mut_ptr() as *mut std::os::raw::c_void);

            let choose_args_ptr = choose_args.map(|ca| ca.ptr).unwrap_or(ptr::null_mut());

            let result_len = crush_do_rule(
                self.ptr,
                ruleno,
                x,
                result.as_mut_ptr(),
                result.len() as i32,
                weights.as_ptr(),
                weights.len() as i32,
                cwin.as_mut_ptr() as *mut std::os::raw::c_void,
                choose_args_ptr,
            );

            if result_len < 0 {
                Err("crush_do_rule failed")
            } else {
                Ok(result_len as usize)
            }
        }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                crush_destroy(self.ptr);
            }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Bucket {
    ptr: *mut CrushBucket,
    _map: *mut CrushMap,
}

impl Bucket {
    pub fn algorithm(&self) -> Option<BucketAlgorithm> {
        unsafe { BucketAlgorithm::from_c((*self.ptr).alg) }
    }

    pub fn weight(&self) -> u32 {
        unsafe { (*self.ptr).weight }
    }

    pub fn size(&self) -> u32 {
        unsafe { (*self.ptr).size }
    }

    pub fn add_item(&mut self, map: &mut Map, item: i32, weight: i32) -> Result<(), i32> {
        unsafe {
            let result = crush_bucket_add_item(map.ptr, self.ptr, item, weight);
            if result == 0 { Ok(()) } else { Err(result) }
        }
    }
}

impl Drop for Bucket {
    fn drop(&mut self) {
        // Only destroy if not owned by a map
        if self._map.is_null() {
            unsafe {
                crush_destroy_bucket(self.ptr);
            }
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    ptr: *mut CrushRule,
}

#[derive(Debug, Clone, Copy)]
pub enum RuleStep {
    Noop,
    Take(i32),
    ChooseFirstN(i32, i32),
    ChooseIndep(i32, i32),
    Emit,
    ChooseLeafFirstN(i32, i32),
    ChooseLeafIndep(i32, i32),
    SetChooseTries(i32),
    SetChooseLeafTries(i32),
    SetChooseLocalTries(i32),
    SetChooseLocalFallbackTries(i32),
    SetChooseLeafVaryR(i32),
    SetChooseLeafStable(i32),
}

impl RuleStep {
    fn to_crush_parts(self) -> (i32, i32, i32) {
        match self {
            RuleStep::Noop => (CRUSH_RULE_NOOP as i32, 0, 0),
            RuleStep::Take(x) => (CRUSH_RULE_TAKE as i32, x, 0),
            RuleStep::ChooseFirstN(n, t) => (CRUSH_RULE_CHOOSE_FIRSTN as i32, n, t),
            RuleStep::ChooseIndep(n, t) => (CRUSH_RULE_CHOOSE_INDEP as i32, n, t),
            RuleStep::Emit => (CRUSH_RULE_EMIT as i32, 0, 0),
            RuleStep::ChooseLeafFirstN(n, t) => (CRUSH_RULE_CHOOSELEAF_FIRSTN as i32, n, t),
            RuleStep::ChooseLeafIndep(n, t) => (CRUSH_RULE_CHOOSELEAF_INDEP as i32, n, t),
            RuleStep::SetChooseTries(n) => (CRUSH_RULE_SET_CHOOSE_TRIES as i32, n, 0),
            RuleStep::SetChooseLeafTries(n) => (CRUSH_RULE_SET_CHOOSELEAF_TRIES as i32, n, 0),
            RuleStep::SetChooseLocalTries(n) => (CRUSH_RULE_SET_CHOOSE_LOCAL_TRIES as i32, n, 0),
            RuleStep::SetChooseLocalFallbackTries(n) => {
                (CRUSH_RULE_SET_CHOOSE_LOCAL_FALLBACK_TRIES as i32, n, 0)
            }
            RuleStep::SetChooseLeafVaryR(n) => (CRUSH_RULE_SET_CHOOSELEAF_VARY_R as i32, n, 0),
            RuleStep::SetChooseLeafStable(n) => (CRUSH_RULE_SET_CHOOSELEAF_STABLE as i32, n, 0),
        }
    }
}

impl Rule {
    pub fn new(
        steps_count: i32,
        ruleset: i32,
        rule_type: i32,
        min_size: i32,
        max_size: i32,
    ) -> Self {
        unsafe {
            let ptr = crush_make_rule(steps_count, ruleset, rule_type, min_size, max_size);
            assert!(!ptr.is_null());
            Rule { ptr }
        }
    }

    pub fn len(&self) -> u32 {
        unsafe { (*self.ptr).len }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set_step(&mut self, pos: i32, step: RuleStep) {
        let (op, arg1, arg2) = step.to_crush_parts();
        unsafe {
            crush_rule_set_step(self.ptr, pos, op, arg1, arg2);
        }
    }
}

impl Drop for Rule {
    fn drop(&mut self) {
        unsafe {
            crush_destroy_rule(self.ptr);
        }
    }
}

#[derive(Debug)]
pub struct ChooseArgs {
    ptr: *mut CrushChooseArg,
    _num_positions: i32,
}

impl ChooseArgs {
    pub fn get_weight_set(&self, bucket_id: i32, position: usize) -> Option<WeightSet<'_>> {
        unsafe {
            let arg = &*self.ptr.offset(-1 - bucket_id as isize);
            if arg.weight_set.is_null() || position >= arg.weight_set_size as usize {
                return None;
            }

            let weight_set = &*arg.weight_set.add(position);
            Some(WeightSet {
                weights: slice::from_raw_parts(weight_set.weights, weight_set.size as usize),
            })
        }
    }

    pub fn get_ids(&self, bucket_id: i32) -> Option<&[i32]> {
        unsafe {
            let arg = &*self.ptr.offset(-1 - bucket_id as isize);
            if arg.ids.is_null() {
                return None;
            }

            Some(slice::from_raw_parts(arg.ids, arg.ids_size as usize))
        }
    }

    pub fn set_weight_set_size(&mut self, bucket_id: i32, size: u32) {
        unsafe {
            let arg = &mut *self.ptr.offset(-1 - bucket_id as isize);
            arg.weight_set_size = size;
        }
    }

    pub fn clear_weight_set(&mut self, bucket_id: i32) {
        unsafe {
            let arg = &mut *self.ptr.offset(-1 - bucket_id as isize);
            arg.weight_set = ptr::null_mut();
            arg.weight_set_size = 0;
            arg.ids = ptr::null_mut();
            arg.ids_size = 0;
        }
    }

    pub fn swap_weights(
        &mut self,
        bucket_id: i32,
        position: usize,
        idx1: usize,
        idx2: usize,
    ) -> Result<(), &'static str> {
        unsafe {
            let arg = &mut *self.ptr.offset(-1 - bucket_id as isize);
            if arg.weight_set.is_null() || position >= arg.weight_set_size as usize {
                return Err("Invalid position");
            }

            let weight_set = &mut *arg.weight_set.add(position);
            let weights = slice::from_raw_parts_mut(weight_set.weights, weight_set.size as usize);

            if idx1 >= weights.len() || idx2 >= weights.len() {
                return Err("Index out of bounds");
            }

            weights.swap(idx1, idx2);
            Ok(())
        }
    }

    pub fn set_id(&mut self, bucket_id: i32, idx: usize, new_id: i32) -> Result<(), &'static str> {
        unsafe {
            let arg = &mut *self.ptr.offset(-1 - bucket_id as isize);
            if arg.ids.is_null() {
                return Err("No ids array");
            }

            let ids = slice::from_raw_parts_mut(arg.ids, arg.ids_size as usize);
            if idx >= ids.len() {
                return Err("Index out of bounds");
            }

            ids[idx] = new_id;
            Ok(())
        }
    }
}

impl Drop for ChooseArgs {
    fn drop(&mut self) {
        unsafe {
            crush_destroy_choose_args(self.ptr);
        }
    }
}

pub struct WeightSet<'a> {
    pub weights: &'a [u32],
}

// Helper function for testing
pub fn is_multiplication_unsafe(a: i64, b: i64) -> bool {
    unsafe { crush_multiplication_is_unsafe(a as u32, b as u32) != 0 }
}
