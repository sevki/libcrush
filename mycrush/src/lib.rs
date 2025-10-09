pub mod crush;
pub use crush::wrapper;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
