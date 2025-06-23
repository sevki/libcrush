// Copyright 2023 Jules AI
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Defines the core data structures for the CRUSH algorithm, reimplemented in safe Rust.

use crate::hash::HashAlgorithm; // Assuming HashAlgorithm will be defined or moved here eventually

// For now, let's define a simple HashType, can be expanded from hash.rs later
// For simplicity, using the same enum as in hash.rs, assuming it might be moved or pub used.
// If hash::HashAlgorithm is kept separate, this might need to be a local simplified version or a direct import.

/// Enumerates the different operation codes for CRUSH rule steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RuleOp {
    Noop = 0,
    Take = 1,
    ChooseFirstN = 2,
    ChooseIndep = 3,
    Emit = 4,
    // Opcode 5 (SetRecurseBucketType) is missing in the provided transpiled files,
    // but present in some versions of Ceph. Adhering to transpiled files for now.
    ChooseLeafFirstN = 6,
    ChooseLeafIndep = 7,
    SetChooseTries = 8,
    SetChooseLeafTries = 9,
    SetChooseLocalTries = 10,
    SetChooseLocalFallbackTries = 11,
    SetChooseLeafVaryR = 12,
    SetChooseLeafStable = 13,
}

impl RuleOp {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(RuleOp::Noop),
            1 => Some(RuleOp::Take),
            2 => Some(RuleOp::ChooseFirstN),
            3 => Some(RuleOp::ChooseIndep),
            4 => Some(RuleOp::Emit),
            6 => Some(RuleOp::ChooseLeafFirstN),
            7 => Some(RuleOp::ChooseLeafIndep),
            8 => Some(RuleOp::SetChooseTries),
            9 => Some(RuleOp::SetChooseLeafTries),
            10 => Some(RuleOp::SetChooseLocalTries),
            11 => Some(RuleOp::SetChooseLocalFallbackTries),
            12 => Some(RuleOp::SetChooseLeafVaryR),
            13 => Some(RuleOp::SetChooseLeafStable),
            _ => None,
        }
    }
}


/// Represents a single step in a CRUSH rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleStep {
    pub op: RuleOp,
    pub arg1: i32,   // Argument 1
    pub arg2: i32,   // Argument 2
}

impl RuleStep {
    pub fn new(op: RuleOp, arg1: i32, arg2: i32) -> Self {
        RuleStep { op, arg1, arg2 }
    }
}

/// Defines the conditions under which a CRUSH rule applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleMask {
    pub ruleset: u8,
    pub type_0: u8,    // 'type' is a keyword, so using type_0
    pub min_size: u8,
    pub max_size: u8,
}

/// Represents a CRUSH rule, which is a sequence of steps.
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub mask: RuleMask,
    pub steps: Vec<RuleStep>,
}

impl Rule {
    /// Creates a new CRUSH rule.
    ///
    /// # Arguments
    /// * `num_steps`: The initial capacity for the steps vector. The C version's `len`
    ///                parameter for `crush_make_rule` effectively set the number of steps.
    /// * `ruleset`: The ruleset this rule belongs to.
    /// * `rule_type`: The type of rule (application-specific).
    /// * `min_size`: The minimum number of replicas this rule should produce.
    /// * `max_size`: The maximum number of replicas this rule should produce.
    pub fn new(num_steps: usize, ruleset: u8, rule_type: u8, min_size: u8, max_size: u8) -> Self {
        Rule {
            mask: RuleMask {
                ruleset,
                type_0: rule_type,
                min_size,
                max_size,
            },
            steps: Vec::with_capacity(num_steps),
        }
    }

    /// Sets a step in the rule at a specific index.
    /// If `index` is equal to the current number of steps, the new step is appended.
    /// Panics if `index` is greater than the current number of steps.
    pub fn set_step(&mut self, index: usize, op: RuleOp, arg1: i32, arg2: i32) {
        let step = RuleStep::new(op, arg1, arg2);
        if index == self.steps.len() {
            self.steps.push(step);
        } else if index < self.steps.len() {
            self.steps[index] = step;
        } else {
            // This behavior needs to match C: C would allocate 'len' steps,
            // and set_step would access within that.
            // For Vec, if we want to set at an arbitrary index up to initial capacity,
            // we might need to pre-fill with placeholder steps or use a different approach.
            // For now, let's assume steps are added/set in a way that respects Vec's current length or capacity.
            // A common pattern is to fill up to capacity.
            // If the C code relies on setting any of the `len` steps at any time,
            // then `steps` should be initialized with `vec![default_step; num_steps]`.
            // Let's assume for now that steps are set sequentially or replace existing ones.
            // The C `crush_make_rule` allocates space for `len` steps.
            // `crush_rule_set_step` asserts `n < rule->len`.
            // So, the `Vec` should be initialized with default steps if we want to mimic by-index setting.
            // Alternative: `add_step` method.
            // Given the C API, initializing `steps` to `num_steps` with default values is closer.
            // However, the current `Rule::new` uses `with_capacity`. If we want to set by index,
            // the vector must already have elements up to that index.
            panic!("set_step: index out of bounds. Current len: {}, index: {}", self.steps.len(), index);
        }
    }

    /// Adds a step to the end of the rule.
    pub fn add_step(&mut self, op: RuleOp, arg1: i32, arg2: i32) {
        self.steps.push(RuleStep::new(op, arg1, arg2));
    }

    /// Fills the steps vector up to `num_steps` with a default RuleStep (e.g., NOOP),
    /// then allows setting steps by index. This is closer to the C API's behavior
    /// where `crush_make_rule` allocates for `len` steps.
    pub fn new_with_len(num_steps: usize, ruleset: u8, rule_type: u8, min_size: u8, max_size: u8) -> Self {
        let default_step = RuleStep::new(RuleOp::Noop, 0, 0);
        Rule {
            mask: RuleMask {
                ruleset,
                type_0: rule_type,
                min_size,
                max_size,
            },
            steps: vec![default_step; num_steps],
        }
    }

    /// Sets a step in the rule at a specific index. Requires `index < self.steps.len()`.
    /// This version is for use with `new_with_len`.
    pub fn set_step_at(&mut self, index: usize, op: RuleOp, arg1: i32, arg2: i32) {
        if index < self.steps.len() {
            self.steps[index] = RuleStep::new(op, arg1, arg2);
        } else {
            panic!("set_step_at: index out of bounds. Vec len: {}, index: {}", self.steps.len(), index);
        }
    }


    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// Enumerates the different CRUSH bucket algorithms.
/// (Mirrors `crush_algorithm` constants)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BucketAlgorithm {
    Uniform = 1,
    List = 2,
    Tree = 3,
    Straw = 4,
    Straw2 = 5,
}

impl BucketAlgorithm {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(BucketAlgorithm::Uniform),
            2 => Some(BucketAlgorithm::List),
            3 => Some(BucketAlgorithm::Tree),
            4 => Some(BucketAlgorithm::Straw),
            5 => Some(BucketAlgorithm::Straw2),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            BucketAlgorithm::Uniform => "uniform",
            BucketAlgorithm::List => "list",
            BucketAlgorithm::Tree => "tree",
            BucketAlgorithm::Straw => "straw",
            BucketAlgorithm::Straw2 => "straw2",
        }
    }
}


/// Holds the specific data for each type of CRUSH bucket.
#[derive(Debug, Clone, PartialEq)]
pub enum BucketContents {
    Uniform {
        item_weight: u32,
    },
    List {
        item_weights: Vec<u32>,
        sum_weights: Vec<u32>, // Cumulative sum of weights
    },
    Tree {
        // num_nodes is implicitly items.len() for leaves, or derived for tree structure
        node_weights: Vec<u32>, // For a tree, this stores weights at different levels
                                // The original `num_nodes` for tree bucket might relate to internal tree structure size
                                // which might be managed differently or implicitly by vec length.
                                // For now, storing the direct correspondent of node_weights.
                                // `num_nodes` from C struct might define the total size of the `node_weights` array.
    },
    Straw {
        item_weights: Vec<u32>,
        straws: Vec<u32>,
    },
    Straw2 {
        item_weights: Vec<u32>,
    },
}

/// Represents a CRUSH bucket, which can be of various algorithms.
#[derive(Debug, Clone, PartialEq)]
pub struct CrushBucket {
    pub id: i32,          // Bucket ID, negative for non-device buckets
    pub bucket_type: u16, // Application-specific type identifier (original `type_0`)
    pub alg: BucketAlgorithm,
    pub hash_alg_id: u8, // Identifier for the hash algorithm to use (e.g., 0 for rjenkins1)
    pub weight: u32,      // Total weight of the bucket
    pub items: Vec<i32>,  // List of item IDs (devices or other buckets)
    pub contents: BucketContents,
}

impl CrushBucket {
    pub fn size(&self) -> usize {
        self.items.len()
    }
}

/// Represents the entire CRUSH map, including buckets, rules, and tunables.
#[derive(Debug, Clone, PartialEq)]
pub struct CrushMap {
    // Buckets are stored in a Vec. The index corresponds to `-(bucket_id)-1`.
    // This matches how they are indexed in the C version's `map->buckets` array.
    // `None` indicates a missing or undefined bucket at that slot.
    pub buckets: Vec<Option<Box<CrushBucket>>>,

    // Rules are also stored in a Vec, indexed by rule ID.
    pub rules: Vec<Option<Box<Rule>>>,

    // Maximum device ID seen. This might be better calculated dynamically or managed differently.
    pub max_devices: i32,

    // Tunables
    pub choose_local_tries: u32,
    pub choose_local_fallback_tries: u32,
    pub choose_total_tries: u32,
    pub chooseleaf_descend_once: bool, // Changed from u32 to bool
    pub chooseleaf_vary_r: bool,       // Changed from u8 to bool
    pub chooseleaf_stable: bool,       // Changed from u8 to bool
    pub straw_calc_version: u8,
    // Represents the bitmask of allowed bucket algorithms.
    // Could also be a HashSet<BucketAlgorithm> for more idiomatic Rust.
    pub allowed_bucket_algs_mask: u32,

    // This seems to be for collecting statistics on choose operations.
    // Size is related to choose_total_tries.
    pub choose_tries_stats: Vec<u32>,
}

impl CrushMap {
    pub fn new() -> Self {
        // Initialize with some defaults, similar to crush_create + set_optimal_crush_map
        // Or provide a builder pattern.
        // For now, a basic constructor.
        CrushMap {
            buckets: Vec::new(),
            rules: Vec::new(),
            max_devices: 0,
            choose_local_tries: 0,
            choose_local_fallback_tries: 0,
            choose_total_tries: 50,
            chooseleaf_descend_once: true,
            chooseleaf_vary_r: true,
            chooseleaf_stable: true,
            straw_calc_version: 1, // Optimal default
            allowed_bucket_algs_mask: (1 << BucketAlgorithm::Uniform as u8) |
                                      (1 << BucketAlgorithm::List as u8) |
                                      (1 << BucketAlgorithm::Straw as u8) |
                                      (1 << BucketAlgorithm::Straw2 as u8),
            choose_tries_stats: Vec::new(), // Initialize as empty, maybe size later
        }
    }

    // Helper to get a bucket by its negative ID, mapping to index
    pub fn get_bucket_by_id(&self, id: i32) -> Option<&CrushBucket> {
        if id >= 0 { return None; } // Only negative IDs for buckets
        let idx = (-id - 1) as usize;
        self.buckets.get(idx).and_then(|opt_b| opt_b.as_deref())
    }

    pub fn get_bucket_by_id_mut(&mut self, id: i32) -> Option<&mut CrushBucket> {
        if id >= 0 { return None; }
        let idx = (-id - 1) as usize;
        self.buckets.get_mut(idx).and_then(|opt_b| opt_b.as_deref_mut())
    }
     // Helper to get a rule by its ID (index)
    pub fn get_rule_by_id(&self, id: u32) -> Option<&Rule> {
        self.rules.get(id as usize).and_then(|opt_r| opt_r.as_deref())
    }

    pub fn get_rule_by_id_mut(&mut self, id: u32) -> Option<&mut Rule> {
        self.rules.get_mut(id as usize).and_then(|opt_r| opt_r.as_deref_mut())
    }
}

// Default implementation for CrushMap
impl Default for CrushMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Used for `crush_choose_arg` related structures.
/// These are more complex and relate to preparing arguments for specific
/// `choose` operations, often involving temporary or modified views of weights/ids.
/// For now, defining basic correspondents.
#[derive(Debug, Clone, PartialEq)]
pub struct WeightSet {
    pub weights: Vec<u32>, // Owned Vec for simplicity, C version uses pointers into a larger allocation
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChooseArgBucketParams {
    pub ids: Vec<i32>, // Can be a slice if referencing map data, or owned if modified
    pub weight_sets: Vec<WeightSet>, // One WeightSet per 'position' or 'replica'
}

// The full crush_choose_arg in C is an array of these, indexed by bucket.
// This might be better represented as a HashMap<i32, ChooseArgBucketParams> in Rust,
// or managed differently when porting the choose logic.
// For now, these are the building blocks.
