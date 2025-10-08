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

/// Error type for CRUSH operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrushError {
    InvalidArgument,
    OutOfMemory,
    NotFound,
    AlreadyExists,
    InvalidOperation,
    Other(i32),
}

// Error code constants (matching POSIX errno values)
pub const EINVAL: i32 = -22;  // Invalid argument
pub const ENOMEM: i32 = -12;  // Out of memory
pub const ENOENT: i32 = -2;   // No such file or directory
pub const EEXIST: i32 = -17;  // File exists

impl CrushError {
    pub fn from_errno(errno: i32) -> Self {
        match errno {
            EINVAL => CrushError::InvalidArgument,
            ENOMEM => CrushError::OutOfMemory,
            ENOENT => CrushError::NotFound,
            EEXIST => CrushError::AlreadyExists,
            other => CrushError::Other(other),
        }
    }

    pub fn to_errno(&self) -> i32 {
        match self {
            CrushError::InvalidArgument => EINVAL,
            CrushError::OutOfMemory => ENOMEM,
            CrushError::NotFound => ENOENT,
            CrushError::AlreadyExists => EEXIST,
            CrushError::InvalidOperation => EINVAL,
            CrushError::Other(code) => *code,
        }
    }
}

impl std::fmt::Display for CrushError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrushError::InvalidArgument => write!(f, "Invalid argument"),
            CrushError::OutOfMemory => write!(f, "Out of memory"),
            CrushError::NotFound => write!(f, "Not found"),
            CrushError::AlreadyExists => write!(f, "Already exists"),
            CrushError::InvalidOperation => write!(f, "Invalid operation"),
            CrushError::Other(code) => write!(f, "Error code: {}", code),
        }
    }
}

impl std::error::Error for CrushError {}

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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CrushOpcodes {
    Noop = 0,
    Take = 1,
    ChooseFirstn = 2,
    ChooseIndep = 3,
    Emit = 4,
    ChooseleafFirstn = 6,
    ChooseleafIndep = 7,
    SetChooseTries = 8,
    SetChooseleafTries = 9,
    SetChooseLocalTries = 10,
    SetChooseLocalFallbackTries = 11,
    SetChooseleafVaryR = 12,
    SetChooseleafStable = 13,
}

// Keep legacy constants for backwards compatibility
pub const CRUSH_RULE_NOOP: u32 = CrushOpcodes::Noop as u32;
pub const CRUSH_RULE_TAKE: u32 = CrushOpcodes::Take as u32;
pub const CRUSH_RULE_CHOOSE_FIRSTN: u32 = CrushOpcodes::ChooseFirstn as u32;
pub const CRUSH_RULE_CHOOSE_INDEP: u32 = CrushOpcodes::ChooseIndep as u32;
pub const CRUSH_RULE_EMIT: u32 = CrushOpcodes::Emit as u32;
pub const CRUSH_RULE_CHOOSELEAF_FIRSTN: u32 = CrushOpcodes::ChooseleafFirstn as u32;
pub const CRUSH_RULE_CHOOSELEAF_INDEP: u32 = CrushOpcodes::ChooseleafIndep as u32;
pub const CRUSH_RULE_SET_CHOOSE_TRIES: u32 = CrushOpcodes::SetChooseTries as u32;
pub const CRUSH_RULE_SET_CHOOSELEAF_TRIES: u32 = CrushOpcodes::SetChooseleafTries as u32;
pub const CRUSH_RULE_SET_CHOOSE_LOCAL_TRIES: u32 = CrushOpcodes::SetChooseLocalTries as u32;
pub const CRUSH_RULE_SET_CHOOSE_LOCAL_FALLBACK_TRIES: u32 = CrushOpcodes::SetChooseLocalFallbackTries as u32;
pub const CRUSH_RULE_SET_CHOOSELEAF_VARY_R: u32 = CrushOpcodes::SetChooseleafVaryR as u32;
pub const CRUSH_RULE_SET_CHOOSELEAF_STABLE: u32 = CrushOpcodes::SetChooseleafStable as u32;

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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CrushAlgorithm {
    Uniform = 1,
    List = 2,
    Tree = 3,
    Straw = 4,
    Straw2 = 5,
}

// Keep legacy constants for backwards compatibility
pub const CRUSH_BUCKET_UNIFORM: u32 = CrushAlgorithm::Uniform as u32;
pub const CRUSH_BUCKET_LIST: u32 = CrushAlgorithm::List as u32;
pub const CRUSH_BUCKET_TREE: u32 = CrushAlgorithm::Tree as u32;
pub const CRUSH_BUCKET_STRAW: u32 = CrushAlgorithm::Straw as u32;
pub const CRUSH_BUCKET_STRAW2: u32 = CrushAlgorithm::Straw2 as u32;

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
