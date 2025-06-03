use num_derive::FromPrimitive;
use num_traits::FromPrimitive as NumTraitsFromPrimitive; // Alias to avoid potential conflicts

#[derive(Debug, Clone, PartialEq, FromPrimitive)]
#[allow(non_camel_case_types)] // To keep original C-style names for opcodes if needed, though Rust style is preferred.
                               // For FromPrimitive, the derive macro should handle it.
pub enum CrushOpcode {
    Noop = 0,
    Take = 1,
    ChooseFirstN = 2,
    ChooseIndep = 3,
    Emit = 4,
    ChooseLeafFirstN = 6,
    ChooseLeafIndep = 7,
    SetChooseTries = 8,
    SetChooseLeafTries = 9,
    SetChooseLocalTries = 10,
    SetChooseLocalFallbackTries = 11,
    SetChooseLeafVaryR = 12,
    SetChooseLeafStable = 13,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrushAlgorithm {
    Uniform = 1,
    List = 2,
    Tree = 3,
    Straw = 4,
    Straw2 = 5,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushRuleMask {
    pub ruleset: u8,
    pub r#type: u8,
    pub min_size: u8,
    pub max_size: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushRuleStep {
    pub op: u32,
    pub arg1: i32,
    pub arg2: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushRule {
    pub len: u32,
    pub mask: CrushRuleMask,
    pub steps: Vec<CrushRuleStep>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushBucketCommon {
    pub id: i32,
    pub r#type: u16,
    pub alg: u8,
    pub hash: u8,
    pub weight: u32,
    pub size: u32,
    pub items: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushBucketUniform {
    pub common: CrushBucketCommon,
    pub item_weight: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushBucketList {
    pub common: CrushBucketCommon,
    pub item_weights: Vec<u32>,
    pub sum_weights: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushBucketStraw2 {
    pub common: CrushBucketCommon,
    pub item_weights: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrushBucket {
    Uniform(CrushBucketUniform),
    List(CrushBucketList),
    Straw2(CrushBucketStraw2),
    // NOTE: Tree and Straw buckets are not included as per common usage and simplification
}

impl CrushBucket {
    pub fn common_mut(&mut self) -> &mut CrushBucketCommon {
        match self {
            CrushBucket::Uniform(b) => &mut b.common,
            CrushBucket::List(b) => &mut b.common,
            CrushBucket::Straw2(b) => &mut b.common,
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.common_mut().id = id;
    }

    // Helper to get common part immutably, useful for builder or other functions
    pub fn common(&self) -> &CrushBucketCommon {
        match self {
            CrushBucket::Uniform(b) => &b.common,
            CrushBucket::List(b) => &b.common,
            CrushBucket::Straw2(b) => &b.common,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushWeightSet {
    pub weights: Vec<u32>,
    pub size: u32, // Corresponds to the C struct's size, might be redundant with Vec::len()
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushChooseArg {
    pub ids: Vec<i32>,
    pub ids_size: u32, // Corresponds to the C struct's size, might be redundant
    pub weight_set: Vec<CrushWeightSet>,
    pub weight_set_size: u32, // Corresponds to the C struct's size, might be redundant
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushChooseArgMap {
    pub args: Vec<CrushChooseArg>,
    pub size: u32, // Corresponds to the C struct's size, might be redundant
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrushMap {
    pub buckets: Vec<Option<CrushBucket>>,
    pub rules: Vec<Option<CrushRule>>,
    pub max_buckets: i32,
    pub max_rules: u32,
    pub max_devices: i32,

    // Tunables
    pub choose_local_tries: u32,
    pub choose_local_fallback_tries: u32,
    pub choose_total_tries: u32,
    pub chooseleaf_descend_once: u32,
    pub chooseleaf_vary_r: u8,
    pub chooseleaf_stable: u8,
    pub straw_calc_version: u8, // from #ifndef __KERNEL__ block
    pub allowed_bucket_algs: u32, // from #ifndef __KERNEL__ block
    // pub choose_tries: Vec<u32>, // This was a pointer in C, needs decision on how to represent if needed

    // C-specific fields, omitted for now or represented differently:
    // working_size: usize, (related to crush_work, C-specific memory management)
    // choose_tries: *mut u32 (pointer, if needed, should be Vec<u32> or similar)
}


// Add a dummy function to make it a library
pub fn hello() {
    println!("Hello from mycrush!");
}

pub mod hash;
pub mod mapper;
pub mod builder;
pub mod helpers;

pub fn crush_get_bucket_item_weight(bucket: &CrushBucket, pos: i32) -> Option<u32> {
    if pos < 0 {
        return None;
    }
    let pos_usize = pos as usize;

    match bucket {
        CrushBucket::Uniform(b) => {
            if pos_usize < b.common.items.len() {
                Some(b.item_weight)
            } else {
                None
            }
        }
        CrushBucket::List(b) => {
            if pos_usize < b.common.items.len() && pos_usize < b.item_weights.len() {
                Some(b.item_weights[pos_usize])
            } else {
                None
            }
        }
        CrushBucket::Straw2(b) => {
            if pos_usize < b.common.items.len() && pos_usize < b.item_weights.len() {
                Some(b.item_weights[pos_usize])
            } else {
                None
            }
        }
    }
}
