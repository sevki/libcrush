//! WASM bindings for the mycrush library
//!
//! This module provides JavaScript-friendly bindings for the CRUSH algorithm.

use wasm_bindgen::prelude::*;

use crate::crush::wrapper::{BucketAlgorithm, Map};

/// A WASM-friendly wrapper for the CRUSH Map
#[wasm_bindgen]
pub struct WasmCrushMap {
    inner: Map,
}

#[wasm_bindgen]
impl WasmCrushMap {
    /// Create a new CRUSH map
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: Map::new() }
    }

    /// Create a new legacy CRUSH map
    #[wasm_bindgen(js_name = newLegacy)]
    pub fn new_legacy() -> Self {
        Self {
            inner: Map::new_legacy(),
        }
    }

    /// Get choose_local_tries parameter
    #[wasm_bindgen(js_name = chooseLocalTries)]
    pub fn choose_local_tries(&self) -> u32 {
        self.inner.choose_local_tries()
    }

    /// Get choose_local_fallback_tries parameter
    #[wasm_bindgen(js_name = chooseLocalFallbackTries)]
    pub fn choose_local_fallback_tries(&self) -> u32 {
        self.inner.choose_local_fallback_tries()
    }

    /// Get choose_total_tries parameter
    #[wasm_bindgen(js_name = chooseTotalTries)]
    pub fn choose_total_tries(&self) -> u32 {
        self.inner.choose_total_tries()
    }

    /// Get chooseleaf_descend_once parameter
    #[wasm_bindgen(js_name = chooseleafDescendOnce)]
    pub fn chooseleaf_descend_once(&self) -> u32 {
        self.inner.chooseleaf_descend_once()
    }

    /// Get chooseleaf_vary_r parameter
    #[wasm_bindgen(js_name = chooseleafVaryR)]
    pub fn chooseleaf_vary_r(&self) -> u8 {
        self.inner.chooseleaf_vary_r()
    }

    /// Get chooseleaf_stable parameter
    #[wasm_bindgen(js_name = chooseleafStable)]
    pub fn chooseleaf_stable(&self) -> u8 {
        self.inner.chooseleaf_stable()
    }

    /// Get straw_calc_version parameter
    #[wasm_bindgen(js_name = strawCalcVersion)]
    pub fn straw_calc_version(&self) -> u8 {
        self.inner.straw_calc_version()
    }

    /// Get allowed_bucket_algs parameter
    #[wasm_bindgen(js_name = allowedBucketAlgs)]
    pub fn allowed_bucket_algs(&self) -> u32 {
        self.inner.allowed_bucket_algs()
    }

    /// Check if the map has any rules
    #[wasm_bindgen(js_name = hasRules)]
    pub fn has_rules(&self) -> bool {
        self.inner.has_rules()
    }

    /// Finalize the map (must be called before using do_rule)
    pub fn finalize(&mut self) {
        self.inner.finalize()
    }

    /// Find root buckets in the map
    #[wasm_bindgen(js_name = findRoots)]
    pub fn find_roots(&self) -> Result<Vec<i32>, String> {
        self.inner
            .find_roots()
            .map_err(|e| format!("Failed to find roots: error code {}", e))
    }
}

/// Helper function to check if multiplication would overflow
#[wasm_bindgen(js_name = isMultiplicationUnsafe)]
pub fn is_multiplication_unsafe(a: i32, b: i32) -> bool {
    crate::crush::wrapper::is_multiplication_unsafe(a as i64, b as i64)
}

/// Get the version of the mycrush library
#[wasm_bindgen(js_name = getVersion)]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Bucket algorithm types
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmBucketAlgorithm {
    Uniform = 0,
    List = 1,
    Tree = 2,
    Straw = 3,
    Straw2 = 4,
}

impl From<WasmBucketAlgorithm> for BucketAlgorithm {
    fn from(alg: WasmBucketAlgorithm) -> Self {
        match alg {
            WasmBucketAlgorithm::Uniform => BucketAlgorithm::Uniform,
            WasmBucketAlgorithm::List => BucketAlgorithm::List,
            WasmBucketAlgorithm::Tree => BucketAlgorithm::Tree,
            WasmBucketAlgorithm::Straw => BucketAlgorithm::Straw,
            WasmBucketAlgorithm::Straw2 => BucketAlgorithm::Straw2,
        }
    }
}
