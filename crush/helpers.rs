#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn __assert_fail(
        __assertion: *const std::ffi::c_char,
        __file: *const std::ffi::c_char,
        __line: std::ffi::c_uint,
        __function: *const std::ffi::c_char,
    ) -> !;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
}
pub type __u8 = std::ffi::c_uchar;
pub type __u16 = std::ffi::c_ushort;
pub type __s32 = std::ffi::c_int;
pub type __u32 = std::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_rule_step {
    pub op: __u32,
    pub arg1: __s32,
    pub arg2: __s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_rule_mask {
    pub ruleset: __u8,
    pub type_0: __u8,
    pub min_size: __u8,
    pub max_size: __u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_rule {
    pub len: __u32,
    pub mask: crush_rule_mask,
    pub steps: [crush_rule_step; 0],
}
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
    pub working_size: std::ffi::c_int,
    pub straw_calc_version: __u8,
    pub allowed_bucket_algs: __u32,
    pub choose_tries: *mut __u32,
}
