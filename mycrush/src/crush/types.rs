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
pub type crush_opcodes = libc::c_uint;
pub const CRUSH_RULE_NOOP: crush_opcodes = 0;
pub const CRUSH_RULE_TAKE: crush_opcodes = 1;
pub const CRUSH_RULE_CHOOSE_FIRSTN: crush_opcodes = 2;
pub const CRUSH_RULE_CHOOSE_INDEP: crush_opcodes = 3;
pub const CRUSH_RULE_EMIT: crush_opcodes = 4;
pub const CRUSH_RULE_CHOOSELEAF_FIRSTN: crush_opcodes = 6;
pub const CRUSH_RULE_CHOOSELEAF_INDEP: crush_opcodes = 7;
pub const CRUSH_RULE_SET_CHOOSE_TRIES: crush_opcodes = 8;
pub const CRUSH_RULE_SET_CHOOSELEAF_TRIES: crush_opcodes = 9;
pub const CRUSH_RULE_SET_CHOOSE_LOCAL_TRIES: crush_opcodes = 10;
pub const CRUSH_RULE_SET_CHOOSE_LOCAL_FALLBACK_TRIES: crush_opcodes = 11;
pub const CRUSH_RULE_SET_CHOOSELEAF_VARY_R: crush_opcodes = 12;
pub const CRUSH_RULE_SET_CHOOSELEAF_STABLE: crush_opcodes = 13;

// CRUSH rule mask
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_rule_mask {
    pub ruleset: __u8,
    pub type_0: __u8,
    pub min_size: __u8,
    pub max_size: __u8,
}

// CRUSH rule
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_rule {
    pub len: __u32,
    pub mask: crush_rule_mask,
    pub steps: [crush_rule_step; 0],
}

// CRUSH bucket algorithms
pub type crush_algorithm = libc::c_uint;
pub const CRUSH_BUCKET_UNIFORM: crush_algorithm = 1;
pub const CRUSH_BUCKET_LIST: crush_algorithm = 2;
pub const CRUSH_BUCKET_TREE: crush_algorithm = 3;
pub const CRUSH_BUCKET_STRAW: crush_algorithm = 4;
pub const CRUSH_BUCKET_STRAW2: crush_algorithm = 5;

// Base CRUSH bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket {
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
pub struct crush_weight_set {
    pub weights: *mut __u32,
    pub size: __u32,
}

// Choose args
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_choose_arg {
    pub ids: *mut libc::c_int,
    pub ids_size: __u32,
    pub weight_set: *mut crush_weight_set,
    pub weight_set_size: __u32,
}

// Uniform bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_uniform {
    pub h: crush_bucket,
    pub item_weight: __u32,
}

// List bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_list {
    pub h: crush_bucket,
    pub item_weights: *mut __u32,
    pub sum_weights: *mut __u32,
}

// Tree bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_tree {
    pub h: crush_bucket,
    pub num_nodes: __u8,
    pub node_weights: *mut __u32,
}

// Straw bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_straw {
    pub h: crush_bucket,
    pub item_weights: *mut __u32,
    pub straws: *mut __u32,
}

// Straw2 bucket
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_straw2 {
    pub h: crush_bucket,
    pub item_weights: *mut __u32,
}

// CRUSH map
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_map {
    pub buckets: *mut *mut crush_bucket,
    pub rules: *mut *mut crush_rule,
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
pub struct crush_work_bucket {
    pub perm_x: __u32,
    pub perm_n: __u32,
    pub perm: *mut __u32,
}

// Work structure for mapper
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_work {
    pub work: *mut *mut crush_work_bucket,
}
