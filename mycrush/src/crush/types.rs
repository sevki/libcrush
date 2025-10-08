/// Common types used across all crush modules
/// Extracted from c2rust transpiled code

// Re-export FFI types from core::ffi to reduce libc dependency
pub mod ffi {
    pub use core::ffi::{c_char, c_int, c_uint, c_uchar, c_ushort, c_ulong, c_longlong, c_ulonglong, c_void, c_double};
    
    // c_long is not in core::ffi, we need to define it based on target platform
    #[cfg(target_pointer_width = "64")]
    #[allow(non_camel_case_types)]
    pub type c_long = i64;
    #[cfg(target_pointer_width = "32")]
    #[allow(non_camel_case_types)]
    pub type c_long = i32;
}

// Basic C types - using core::ffi instead of libc
pub type SizeT = ffi::c_ulong;
pub type U8 = ffi::c_uchar;
pub type U16 = ffi::c_ushort;
pub type S32 = ffi::c_int;
pub type U32 = ffi::c_uint;
pub type S64 = ffi::c_longlong;
pub type U64 = ffi::c_ulonglong;

// CRUSH rule step
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushRuleStep {
    pub op: U32,
    pub arg1: S32,
    pub arg2: S32,
}

// CRUSH rule opcodes
pub type CrushOpcodes = U32;
pub const CRUSH_RULE_NOOP: CrushOpcodes = 0;
pub const CRUSH_RULE_TAKE: CrushOpcodes = 1;
pub const CRUSH_RULE_CHOOSE_FIRSTN: CrushOpcodes = 2;
pub const CRUSH_RULE_CHOOSE_INDEP: CrushOpcodes = 3;
pub const CRUSH_RULE_EMIT: CrushOpcodes = 4;
pub const CRUSH_RULE_CHOOSELEAF_FIRSTN: CrushOpcodes = 6;
pub const CRUSH_RULE_CHOOSELEAF_INDEP: CrushOpcodes = 7;
pub const CRUSH_RULE_SET_CHOOSE_TRIES: CrushOpcodes = 8;
pub const CRUSH_RULE_SET_CHOOSELEAF_TRIES: CrushOpcodes = 9;
pub const CRUSH_RULE_SET_CHOOSE_LOCAL_TRIES: CrushOpcodes = 10;
pub const CRUSH_RULE_SET_CHOOSE_LOCAL_FALLBACK_TRIES: CrushOpcodes = 11;
pub const CRUSH_RULE_SET_CHOOSELEAF_VARY_R: CrushOpcodes = 12;
pub const CRUSH_RULE_SET_CHOOSELEAF_STABLE: CrushOpcodes = 13;

// CRUSH rule mask
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushRuleMask {
    pub ruleset: U8,
    pub type_0: U8,
    pub min_size: U8,
    pub max_size: U8,
}

// CRUSH rule
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushRule {
    pub len: U32,
    pub mask: CrushRuleMask,
    pub steps: [CrushRuleStep; 0],
}

// CRUSH bucket algorithms
pub type CrushAlgorithm = U32;
pub const CRUSH_BUCKET_UNIFORM: CrushAlgorithm = 1;
pub const CRUSH_BUCKET_LIST: CrushAlgorithm = 2;
pub const CRUSH_BUCKET_TREE: CrushAlgorithm = 3;
pub const CRUSH_BUCKET_STRAW: CrushAlgorithm = 4;
pub const CRUSH_BUCKET_STRAW2: CrushAlgorithm = 5;

// Base CRUSH bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucket {
    pub id: S32,
    pub type_0: U16,
    pub alg: U8,
    pub hash: U8,
    pub weight: U32,
    pub size: U32,
    pub items: *mut S32,
}

// Weight set for choose args
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushWeightSet {
    pub weights: *mut U32,
    pub size: U32,
}

// Choose args
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushChooseArg {
    pub ids: *mut S32,
    pub ids_size: U32,
    pub weight_set: *mut CrushWeightSet,
    pub weight_set_size: U32,
}

// Uniform bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucketUniform {
    pub h: CrushBucket,
    pub item_weight: U32,
}

// List bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucketList {
    pub h: CrushBucket,
    pub item_weights: *mut U32,
    pub sum_weights: *mut U32,
}

// Tree bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucketTree {
    pub h: CrushBucket,
    pub num_nodes: U8,
    pub node_weights: *mut U32,
}

// Straw bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucketStraw {
    pub h: CrushBucket,
    pub item_weights: *mut U32,
    pub straws: *mut U32,
}

// Straw2 bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucketStraw2 {
    pub h: CrushBucket,
    pub item_weights: *mut U32,
}

// CRUSH map
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushMap {
    pub buckets: *mut *mut CrushBucket,
    pub rules: *mut *mut CrushRule,
    pub max_buckets: S32,
    pub max_rules: U32,
    pub max_devices: S32,
    pub choose_local_tries: U32,
    pub choose_local_fallback_tries: U32,
    pub choose_total_tries: U32,
    pub chooseleaf_descend_once: U32,
    pub chooseleaf_vary_r: U8,
    pub chooseleaf_stable: U8,
    pub working_size: SizeT,
    pub straw_calc_version: U8,
    pub allowed_bucket_algs: U32,
    pub choose_tries: *mut U32,
}

// Work bucket for mapper
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushWorkBucket {
    pub perm_x: U32,
    pub perm_n: U32,
    pub perm: *mut U32,
}

// Work structure for mapper
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushWork {
    pub work: *mut *mut CrushWorkBucket,
}
