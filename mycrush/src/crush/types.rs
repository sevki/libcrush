/// Common types used across all crush modules
/// Extracted from c2rust transpiled code
use ::libc;

// Basic C types
pub type size_t = libc::c_ulong;
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;
pub type __u32 = libc::c_uint;
pub type __s64 = libc::c_longlong;
pub type __u64 = libc::c_ulonglong;

// CRUSH rule step
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_rule_step {
    pub op: __u32,
    pub arg1: __s32,
    pub arg2: __s32,
}

// CRUSH rule opcodes
pub type CrushOpcodes = libc::c_uint;
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
    pub ruleset: __u8,
    pub type_0: __u8,
    pub min_size: __u8,
    pub max_size: __u8,
}

// CRUSH rule
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushRule {
    pub len: __u32,
    pub mask: CrushRuleMask,
    pub steps: [crush_rule_step; 0],
}

// CRUSH bucket algorithms
pub type CrushAlgorithm = libc::c_uint;
pub const CRUSH_BUCKET_UNIFORM: CrushAlgorithm = 1;
pub const CRUSH_BUCKET_LIST: CrushAlgorithm = 2;
pub const CRUSH_BUCKET_TREE: CrushAlgorithm = 3;
pub const CRUSH_BUCKET_STRAW: CrushAlgorithm = 4;
pub const CRUSH_BUCKET_STRAW2: CrushAlgorithm = 5;

// Base CRUSH bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucket {
    pub id: __s32,
    pub type_0: __u16,
    pub alg: __u8,
    pub hash: __u8,
    pub weight: __u32,
    pub size: __u32,
    pub items: *mut __s32,
}

// Weight set for choose args
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushWeightSet {
    pub weights: *mut __u32,
    pub size: __u32,
}

// Choose args
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushChooseArg {
    pub ids: *mut libc::c_int,
    pub ids_size: __u32,
    pub weight_set: *mut CrushWeightSet,
    pub weight_set_size: __u32,
}

// Uniform bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucketUniform {
    pub h: CrushBucket,
    pub item_weight: __u32,
}

// List bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucketList {
    pub h: CrushBucket,
    pub item_weights: *mut __u32,
    pub sum_weights: *mut __u32,
}

// Tree bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucketTree {
    pub h: CrushBucket,
    pub num_nodes: __u8,
    pub node_weights: *mut __u32,
}

// Straw bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucketStraw {
    pub h: CrushBucket,
    pub item_weights: *mut __u32,
    pub straws: *mut __u32,
}

// Straw2 bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushBucketStraw2 {
    pub h: CrushBucket,
    pub item_weights: *mut __u32,
}

// CRUSH map
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushMap {
    pub buckets: *mut *mut CrushBucket,
    pub rules: *mut *mut CrushRule,
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

// Work bucket for mapper
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushWorkBucket {
    pub perm_x: __u32,
    pub perm_n: __u32,
    pub perm: *mut __u32,
}

// Work structure for mapper
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrushWork {
    pub work: *mut *mut CrushWorkBucket,
}
