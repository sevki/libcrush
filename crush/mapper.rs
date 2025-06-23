#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const std::ffi::c_char,
        __file: *const std::ffi::c_char,
        __line: std::ffi::c_uint,
        __function: *const std::ffi::c_char,
    ) -> !;
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn crush_hash32_2(type_0: std::ffi::c_int, a: __u32, b: __u32) -> __u32;
    fn crush_hash32_3(type_0: std::ffi::c_int, a: __u32, b: __u32, c: __u32) -> __u32;
    fn crush_hash32_4(
        type_0: std::ffi::c_int,
        a: __u32,
        b: __u32,
        c: __u32,
        d: __u32,
    ) -> __u32;
}
pub type __u8 = std::ffi::c_uchar;
pub type __u16 = std::ffi::c_ushort;
pub type __s32 = std::ffi::c_int;
pub type __u32 = std::ffi::c_uint;
pub type __s64 = std::ffi::c_longlong;
pub type __u64 = std::ffi::c_ulonglong;
pub type size_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_rule_step {
    pub op: __u32,
    pub arg1: __s32,
    pub arg2: __s32,
}
pub type crush_opcodes = std::ffi::c_uint;
pub const CRUSH_RULE_SET_CHOOSELEAF_STABLE: crush_opcodes = 13;
pub const CRUSH_RULE_SET_CHOOSELEAF_VARY_R: crush_opcodes = 12;
pub const CRUSH_RULE_SET_CHOOSE_LOCAL_FALLBACK_TRIES: crush_opcodes = 11;
pub const CRUSH_RULE_SET_CHOOSE_LOCAL_TRIES: crush_opcodes = 10;
pub const CRUSH_RULE_SET_CHOOSELEAF_TRIES: crush_opcodes = 9;
pub const CRUSH_RULE_SET_CHOOSE_TRIES: crush_opcodes = 8;
pub const CRUSH_RULE_CHOOSELEAF_INDEP: crush_opcodes = 7;
pub const CRUSH_RULE_CHOOSELEAF_FIRSTN: crush_opcodes = 6;
pub const CRUSH_RULE_EMIT: crush_opcodes = 4;
pub const CRUSH_RULE_CHOOSE_INDEP: crush_opcodes = 3;
pub const CRUSH_RULE_CHOOSE_FIRSTN: crush_opcodes = 2;
pub const CRUSH_RULE_TAKE: crush_opcodes = 1;
pub const CRUSH_RULE_NOOP: crush_opcodes = 0;
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
pub type crush_algorithm = std::ffi::c_uint;
pub const CRUSH_BUCKET_STRAW2: crush_algorithm = 5;
pub const CRUSH_BUCKET_STRAW: crush_algorithm = 4;
pub const CRUSH_BUCKET_TREE: crush_algorithm = 3;
pub const CRUSH_BUCKET_LIST: crush_algorithm = 2;
pub const CRUSH_BUCKET_UNIFORM: crush_algorithm = 1;
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
pub struct crush_weight_set {
    pub weights: *mut __u32,
    pub size: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_choose_arg {
    pub ids: *mut std::ffi::c_int,
    pub ids_size: __u32,
    pub weight_set: *mut crush_weight_set,
    pub weight_set_size: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_uniform {
    pub h: crush_bucket,
    pub item_weight: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_list {
    pub h: crush_bucket,
    pub item_weights: *mut __u32,
    pub sum_weights: *mut __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_tree {
    pub h: crush_bucket,
    pub num_nodes: __u8,
    pub node_weights: *mut __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_straw {
    pub h: crush_bucket,
    pub item_weights: *mut __u32,
    pub straws: *mut __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_bucket_straw2 {
    pub h: crush_bucket,
    pub item_weights: *mut __u32,
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
    pub working_size: size_t,
    pub straw_calc_version: __u8,
    pub allowed_bucket_algs: __u32,
    pub choose_tries: *mut __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_work_bucket {
    pub perm_x: __u32,
    pub perm_n: __u32,
    pub perm: *mut __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crush_work {
    pub work: *mut *mut crush_work_bucket,
}
static mut __RH_LH_tbl: [__s64; 258] = [
    0x1000000000000 as std::ffi::c_longlong,
    0 as std::ffi::c_longlong,
    0xfe03f80fe040 as std::ffi::c_longlong,
    0x2dfca16dde1 as std::ffi::c_longlong,
    0xfc0fc0fc0fc1 as std::ffi::c_longlong,
    0x5b9e5a170b4 as std::ffi::c_longlong,
    0xfa232cf25214 as std::ffi::c_longlong,
    0x88e68ea899a as std::ffi::c_longlong,
    0xf83e0f83e0f9 as std::ffi::c_longlong,
    0xb5d69bac77e as std::ffi::c_longlong,
    0xf6603d980f67 as std::ffi::c_longlong,
    0xe26fd5c8555 as std::ffi::c_longlong,
    0xf4898d5f85bc as std::ffi::c_longlong,
    0x10eb389fa29f as std::ffi::c_longlong,
    0xf2b9d6480f2c as std::ffi::c_longlong,
    0x13aa2fdd27f1 as std::ffi::c_longlong,
    0xf0f0f0f0f0f1 as std::ffi::c_longlong,
    0x1663f6fac913 as std::ffi::c_longlong,
    0xef2eb71fc435 as std::ffi::c_longlong,
    0x1918a16e4633 as std::ffi::c_longlong,
    0xed7303b5cc0f as std::ffi::c_longlong,
    0x1bc84240adab as std::ffi::c_longlong,
    0xebbdb2a5c162 as std::ffi::c_longlong,
    0x1e72ec117fa5 as std::ffi::c_longlong,
    0xea0ea0ea0ea1 as std::ffi::c_longlong,
    0x2118b119b4f3 as std::ffi::c_longlong,
    0xe865ac7b7604 as std::ffi::c_longlong,
    0x23b9a32eaa56 as std::ffi::c_longlong,
    0xe6c2b4481cd9 as std::ffi::c_longlong,
    0x2655d3c4f15c as std::ffi::c_longlong,
    0xe525982af70d as std::ffi::c_longlong,
    0x28ed53f307ee as std::ffi::c_longlong,
    0xe38e38e38e39 as std::ffi::c_longlong,
    0x2b803473f7ad as std::ffi::c_longlong,
    0xe1fc780e1fc8 as std::ffi::c_longlong,
    0x2e0e85a9de04 as std::ffi::c_longlong,
    0xe070381c0e08 as std::ffi::c_longlong,
    0x309857a05e07 as std::ffi::c_longlong,
    0xdee95c4ca038 as std::ffi::c_longlong,
    0x331dba0efce1 as std::ffi::c_longlong,
    0xdd67c8a60dd7 as std::ffi::c_longlong,
    0x359ebc5b69d9 as std::ffi::c_longlong,
    0xdbeb61eed19d as std::ffi::c_longlong,
    0x381b6d9bb29b as std::ffi::c_longlong,
    0xda740da740db as std::ffi::c_longlong,
    0x3a93dc9864b2 as std::ffi::c_longlong,
    0xd901b2036407 as std::ffi::c_longlong,
    0x3d0817ce9cd4 as std::ffi::c_longlong,
    0xd79435e50d7a as std::ffi::c_longlong,
    0x3f782d7204d0 as std::ffi::c_longlong,
    0xd62b80d62b81 as std::ffi::c_longlong,
    0x41e42b6ec0c0 as std::ffi::c_longlong,
    0xd4c77b03531e as std::ffi::c_longlong,
    0x444c1f6b4c2d as std::ffi::c_longlong,
    0xd3680d3680d4 as std::ffi::c_longlong,
    0x46b016ca47c1 as std::ffi::c_longlong,
    0xd20d20d20d21 as std::ffi::c_longlong,
    0x49101eac381c as std::ffi::c_longlong,
    0xd0b69fcbd259 as std::ffi::c_longlong,
    0x4b6c43f1366a as std::ffi::c_longlong,
    0xcf6474a8819f as std::ffi::c_longlong,
    0x4dc4933a9337 as std::ffi::c_longlong,
    0xce168a772509 as std::ffi::c_longlong,
    0x501918ec6c11 as std::ffi::c_longlong,
    0xcccccccccccd as std::ffi::c_longlong,
    0x5269e12f346e as std::ffi::c_longlong,
    0xcb8727c065c4 as std::ffi::c_longlong,
    0x54b6f7f1325a as std::ffi::c_longlong,
    0xca4587e6b750 as std::ffi::c_longlong,
    0x570068e7ef5a as std::ffi::c_longlong,
    0xc907da4e8712 as std::ffi::c_longlong,
    0x59463f919dee as std::ffi::c_longlong,
    0xc7ce0c7ce0c8 as std::ffi::c_longlong,
    0x5b8887367433 as std::ffi::c_longlong,
    0xc6980c6980c7 as std::ffi::c_longlong,
    0x5dc74ae9fbec as std::ffi::c_longlong,
    0xc565c87b5f9e as std::ffi::c_longlong,
    0x6002958c5871 as std::ffi::c_longlong,
    0xc4372f855d83 as std::ffi::c_longlong,
    0x623a71cb82c8 as std::ffi::c_longlong,
    0xc30c30c30c31 as std::ffi::c_longlong,
    0x646eea247c5c as std::ffi::c_longlong,
    0xc1e4bbd595f7 as std::ffi::c_longlong,
    0x66a008e4788c as std::ffi::c_longlong,
    0xc0c0c0c0c0c1 as std::ffi::c_longlong,
    0x68cdd829fd81 as std::ffi::c_longlong,
    0xbfa02fe80bfb as std::ffi::c_longlong,
    0x6af861e5fc7d as std::ffi::c_longlong,
    0xbe82fa0be830 as std::ffi::c_longlong,
    0x6d1fafdce20a as std::ffi::c_longlong,
    0xbd6910470767 as std::ffi::c_longlong,
    0x6f43cba79e40 as std::ffi::c_longlong,
    0xbc52640bc527 as std::ffi::c_longlong,
    0x7164beb4a56d as std::ffi::c_longlong,
    0xbb3ee721a54e as std::ffi::c_longlong,
    0x73829248e961 as std::ffi::c_longlong,
    0xba2e8ba2e8bb as std::ffi::c_longlong,
    0x759d4f80cba8 as std::ffi::c_longlong,
    0xb92143fa36f6 as std::ffi::c_longlong,
    0x77b4ff5108d9 as std::ffi::c_longlong,
    0xb81702e05c0c as std::ffi::c_longlong,
    0x79c9aa879d53 as std::ffi::c_longlong,
    0xb70fbb5a19bf as std::ffi::c_longlong,
    0x7bdb59cca388 as std::ffi::c_longlong,
    0xb60b60b60b61 as std::ffi::c_longlong,
    0x7dea15a32c1b as std::ffi::c_longlong,
    0xb509e68a9b95 as std::ffi::c_longlong,
    0x7ff5e66a0ffe as std::ffi::c_longlong,
    0xb40b40b40b41 as std::ffi::c_longlong,
    0x81fed45cbccb as std::ffi::c_longlong,
    0xb30f63528918 as std::ffi::c_longlong,
    0x8404e793fb81 as std::ffi::c_longlong,
    0xb21642c8590c as std::ffi::c_longlong,
    0x86082806b1d5 as std::ffi::c_longlong,
    0xb11fd3b80b12 as std::ffi::c_longlong,
    0x88089d8a9e47 as std::ffi::c_longlong,
    0xb02c0b02c0b1 as std::ffi::c_longlong,
    0x8a064fd50f2a as std::ffi::c_longlong,
    0xaf3addc680b0 as std::ffi::c_longlong,
    0x8c01467b94bb as std::ffi::c_longlong,
    0xae4c415c9883 as std::ffi::c_longlong,
    0x8df988f4ae80 as std::ffi::c_longlong,
    0xad602b580ad7 as std::ffi::c_longlong,
    0x8fef1e987409 as std::ffi::c_longlong,
    0xac7691840ac8 as std::ffi::c_longlong,
    0x91e20ea1393e as std::ffi::c_longlong,
    0xab8f69e2835a as std::ffi::c_longlong,
    0x93d2602c2e5f as std::ffi::c_longlong,
    0xaaaaaaaaaaab as std::ffi::c_longlong,
    0x95c01a39fbd6 as std::ffi::c_longlong,
    0xa9c84a47a080 as std::ffi::c_longlong,
    0x97ab43af59f9 as std::ffi::c_longlong,
    0xa8e83f5717c1 as std::ffi::c_longlong,
    0x9993e355a4e5 as std::ffi::c_longlong,
    0xa80a80a80a81 as std::ffi::c_longlong,
    0x9b79ffdb6c8b as std::ffi::c_longlong,
    0xa72f0539782a as std::ffi::c_longlong,
    0x9d5d9fd5010b as std::ffi::c_longlong,
    0xa655c4392d7c as std::ffi::c_longlong,
    0x9f3ec9bcfb80 as std::ffi::c_longlong,
    0xa57eb50295fb as std::ffi::c_longlong,
    0xa11d83f4c355 as std::ffi::c_longlong,
    0xa4a9cf1d9684 as std::ffi::c_longlong,
    0xa2f9d4c51039 as std::ffi::c_longlong,
    0xa3d70a3d70a4 as std::ffi::c_longlong,
    0xa4d3c25e68dc as std::ffi::c_longlong,
    0xa3065e3fae7d as std::ffi::c_longlong,
    0xa6ab52d99e76 as std::ffi::c_longlong,
    0xa237c32b16d0 as std::ffi::c_longlong,
    0xa8808c384547 as std::ffi::c_longlong,
    0xa16b312ea8fd as std::ffi::c_longlong,
    0xaa5374652a1c as std::ffi::c_longlong,
    0xa0a0a0a0a0a1 as std::ffi::c_longlong,
    0xac241134c4e9 as std::ffi::c_longlong,
    0x9fd809fd80a0 as std::ffi::c_longlong,
    0xadf26865a8a1 as std::ffi::c_longlong,
    0x9f1165e72549 as std::ffi::c_longlong,
    0xafbe7fa0f04d as std::ffi::c_longlong,
    0x9e4cad23dd60 as std::ffi::c_longlong,
    0xb1885c7aa982 as std::ffi::c_longlong,
    0x9d89d89d89d9 as std::ffi::c_longlong,
    0xb35004723c46 as std::ffi::c_longlong,
    0x9cc8e160c3fc as std::ffi::c_longlong,
    0xb5157cf2d078 as std::ffi::c_longlong,
    0x9c09c09c09c1 as std::ffi::c_longlong,
    0xb6d8cb53b0ca as std::ffi::c_longlong,
    0x9b4c6f9ef03b as std::ffi::c_longlong,
    0xb899f4d8ab63 as std::ffi::c_longlong,
    0x9a90e7d95bc7 as std::ffi::c_longlong,
    0xba58feb2703a as std::ffi::c_longlong,
    0x99d722dabde6 as std::ffi::c_longlong,
    0xbc15edfeed32 as std::ffi::c_longlong,
    0x991f1a515886 as std::ffi::c_longlong,
    0xbdd0c7c9a817 as std::ffi::c_longlong,
    0x9868c809868d as std::ffi::c_longlong,
    0xbf89910c1678 as std::ffi::c_longlong,
    0x97b425ed097c as std::ffi::c_longlong,
    0xc1404eadf383 as std::ffi::c_longlong,
    0x97012e025c05 as std::ffi::c_longlong,
    0xc2f5058593d9 as std::ffi::c_longlong,
    0x964fda6c0965 as std::ffi::c_longlong,
    0xc4a7ba58377c as std::ffi::c_longlong,
    0x95a02568095b as std::ffi::c_longlong,
    0xc65871da59dd as std::ffi::c_longlong,
    0x94f2094f2095 as std::ffi::c_longlong,
    0xc80730b00016 as std::ffi::c_longlong,
    0x944580944581 as std::ffi::c_longlong,
    0xc9b3fb6d0559 as std::ffi::c_longlong,
    0x939a85c4093a as std::ffi::c_longlong,
    0xcb5ed69565af as std::ffi::c_longlong,
    0x92f113840498 as std::ffi::c_longlong,
    0xcd07c69d8702 as std::ffi::c_longlong,
    0x924924924925 as std::ffi::c_longlong,
    0xceaecfea8085 as std::ffi::c_longlong,
    0x91a2b3c4d5e7 as std::ffi::c_longlong,
    0xd053f6d26089 as std::ffi::c_longlong,
    0x90fdbc090fdc as std::ffi::c_longlong,
    0xd1f73f9c70c0 as std::ffi::c_longlong,
    0x905a38633e07 as std::ffi::c_longlong,
    0xd398ae817906 as std::ffi::c_longlong,
    0x8fb823ee08fc as std::ffi::c_longlong,
    0xd53847ac00a6 as std::ffi::c_longlong,
    0x8f1779d9fdc4 as std::ffi::c_longlong,
    0xd6d60f388e41 as std::ffi::c_longlong,
    0x8e78356d1409 as std::ffi::c_longlong,
    0xd8720935e643 as std::ffi::c_longlong,
    0x8dda5202376a as std::ffi::c_longlong,
    0xda0c39a54804 as std::ffi::c_longlong,
    0x8d3dcb08d3dd as std::ffi::c_longlong,
    0xdba4a47aa996 as std::ffi::c_longlong,
    0x8ca29c046515 as std::ffi::c_longlong,
    0xdd3b4d9cf24b as std::ffi::c_longlong,
    0x8c08c08c08c1 as std::ffi::c_longlong,
    0xded038e633f3 as std::ffi::c_longlong,
    0x8b70344a139c as std::ffi::c_longlong,
    0xe0636a23e2ee as std::ffi::c_longlong,
    0x8ad8f2fba939 as std::ffi::c_longlong,
    0xe1f4e5170d02 as std::ffi::c_longlong,
    0x8a42f870566a as std::ffi::c_longlong,
    0xe384ad748f0e as std::ffi::c_longlong,
    0x89ae4089ae41 as std::ffi::c_longlong,
    0xe512c6e54998 as std::ffi::c_longlong,
    0x891ac73ae982 as std::ffi::c_longlong,
    0xe69f35065448 as std::ffi::c_longlong,
    0x888888888889 as std::ffi::c_longlong,
    0xe829fb693044 as std::ffi::c_longlong,
    0x87f78087f781 as std::ffi::c_longlong,
    0xe9b31d93f98e as std::ffi::c_longlong,
    0x8767ab5f34e5 as std::ffi::c_longlong,
    0xeb3a9f019750 as std::ffi::c_longlong,
    0x86d905447a35 as std::ffi::c_longlong,
    0xecc08321eb30 as std::ffi::c_longlong,
    0x864b8a7de6d2 as std::ffi::c_longlong,
    0xee44cd59ffab as std::ffi::c_longlong,
    0x85bf37612cef as std::ffi::c_longlong,
    0xefc781043579 as std::ffi::c_longlong,
    0x853408534086 as std::ffi::c_longlong,
    0xf148a170700a as std::ffi::c_longlong,
    0x84a9f9c8084b as std::ffi::c_longlong,
    0xf2c831e44116 as std::ffi::c_longlong,
    0x842108421085 as std::ffi::c_longlong,
    0xf446359b1353 as std::ffi::c_longlong,
    0x839930523fbf as std::ffi::c_longlong,
    0xf5c2afc65447 as std::ffi::c_longlong,
    0x83126e978d50 as std::ffi::c_longlong,
    0xf73da38d9d4a as std::ffi::c_longlong,
    0x828cbfbeb9a1 as std::ffi::c_longlong,
    0xf8b7140edbb1 as std::ffi::c_longlong,
    0x820820820821 as std::ffi::c_longlong,
    0xfa2f045e7832 as std::ffi::c_longlong,
    0x81848da8faf1 as std::ffi::c_longlong,
    0xfba577877d7d as std::ffi::c_longlong,
    0x810204081021 as std::ffi::c_longlong,
    0xfd1a708bbe11 as std::ffi::c_longlong,
    0x808080808081 as std::ffi::c_longlong,
    0xfe8df263f957 as std::ffi::c_longlong,
    0x800000000000 as std::ffi::c_longlong,
    0xffff00000000 as std::ffi::c_longlong,
];
static mut __LL_tbl: [__s64; 256] = [
    0 as std::ffi::c_ulonglong as __s64,
    0x2e2a60a00 as std::ffi::c_ulonglong as __s64,
    0x70cb64ec5 as std::ffi::c_ulonglong as __s64,
    0x9ef50ce67 as std::ffi::c_ulonglong as __s64,
    0xcd1e588fd as std::ffi::c_ulonglong as __s64,
    0xfb4747e9c as std::ffi::c_ulonglong as __s64,
    0x1296fdaf5e as std::ffi::c_ulonglong as __s64,
    0x1579811b58 as std::ffi::c_ulonglong as __s64,
    0x185bfec2a1 as std::ffi::c_ulonglong as __s64,
    0x1b3e76a552 as std::ffi::c_ulonglong as __s64,
    0x1e20e8c380 as std::ffi::c_ulonglong as __s64,
    0x2103551d43 as std::ffi::c_ulonglong as __s64,
    0x23e5bbb2b2 as std::ffi::c_ulonglong as __s64,
    0x26c81c83e4 as std::ffi::c_ulonglong as __s64,
    0x29aa7790f0 as std::ffi::c_ulonglong as __s64,
    0x2c8cccd9ed as std::ffi::c_ulonglong as __s64,
    0x2f6f1c5ef2 as std::ffi::c_ulonglong as __s64,
    0x3251662017 as std::ffi::c_ulonglong as __s64,
    0x3533aa1d71 as std::ffi::c_ulonglong as __s64,
    0x3815e8571a as std::ffi::c_ulonglong as __s64,
    0x3af820cd26 as std::ffi::c_ulonglong as __s64,
    0x3dda537fae as std::ffi::c_ulonglong as __s64,
    0x40bc806ec8 as std::ffi::c_ulonglong as __s64,
    0x439ea79a8c as std::ffi::c_ulonglong as __s64,
    0x4680c90310 as std::ffi::c_ulonglong as __s64,
    0x4962e4a86c as std::ffi::c_ulonglong as __s64,
    0x4c44fa8ab6 as std::ffi::c_ulonglong as __s64,
    0x4f270aaa06 as std::ffi::c_ulonglong as __s64,
    0x5209150672 as std::ffi::c_ulonglong as __s64,
    0x54eb19a013 as std::ffi::c_ulonglong as __s64,
    0x57cd1876fd as std::ffi::c_ulonglong as __s64,
    0x5aaf118b4a as std::ffi::c_ulonglong as __s64,
    0x5d9104dd0f as std::ffi::c_ulonglong as __s64,
    0x6072f26c64 as std::ffi::c_ulonglong as __s64,
    0x6354da3960 as std::ffi::c_ulonglong as __s64,
    0x6636bc441a as std::ffi::c_ulonglong as __s64,
    0x6918988ca8 as std::ffi::c_ulonglong as __s64,
    0x6bfa6f1322 as std::ffi::c_ulonglong as __s64,
    0x6edc3fd79f as std::ffi::c_ulonglong as __s64,
    0x71be0ada35 as std::ffi::c_ulonglong as __s64,
    0x749fd01afd as std::ffi::c_ulonglong as __s64,
    0x77818f9a0c as std::ffi::c_ulonglong as __s64,
    0x7a6349577a as std::ffi::c_ulonglong as __s64,
    0x7d44fd535e as std::ffi::c_ulonglong as __s64,
    0x8026ab8dce as std::ffi::c_ulonglong as __s64,
    0x83085406e3 as std::ffi::c_ulonglong as __s64,
    0x85e9f6beb2 as std::ffi::c_ulonglong as __s64,
    0x88cb93b552 as std::ffi::c_ulonglong as __s64,
    0x8bad2aeadc as std::ffi::c_ulonglong as __s64,
    0x8e8ebc5f65 as std::ffi::c_ulonglong as __s64,
    0x9170481305 as std::ffi::c_ulonglong as __s64,
    0x9451ce05d3 as std::ffi::c_ulonglong as __s64,
    0x97334e37e5 as std::ffi::c_ulonglong as __s64,
    0x9a14c8a953 as std::ffi::c_ulonglong as __s64,
    0x9cf63d5a33 as std::ffi::c_ulonglong as __s64,
    0x9fd7ac4a9d as std::ffi::c_ulonglong as __s64,
    0xa2b07f3458 as std::ffi::c_ulonglong as __s64,
    0xa59a78ea6a as std::ffi::c_ulonglong as __s64,
    0xa87bd699fb as std::ffi::c_ulonglong as __s64,
    0xab5d2e8970 as std::ffi::c_ulonglong as __s64,
    0xae3e80b8e3 as std::ffi::c_ulonglong as __s64,
    0xb11fcd2869 as std::ffi::c_ulonglong as __s64,
    0xb40113d818 as std::ffi::c_ulonglong as __s64,
    0xb6e254c80a as std::ffi::c_ulonglong as __s64,
    0xb9c38ff853 as std::ffi::c_ulonglong as __s64,
    0xbca4c5690c as std::ffi::c_ulonglong as __s64,
    0xbf85f51a4a as std::ffi::c_ulonglong as __s64,
    0xc2671f0c26 as std::ffi::c_ulonglong as __s64,
    0xc548433eb6 as std::ffi::c_ulonglong as __s64,
    0xc82961b211 as std::ffi::c_ulonglong as __s64,
    0xcb0a7a664d as std::ffi::c_ulonglong as __s64,
    0xcdeb8d5b82 as std::ffi::c_ulonglong as __s64,
    0xd0cc9a91c8 as std::ffi::c_ulonglong as __s64,
    0xd3ada20933 as std::ffi::c_ulonglong as __s64,
    0xd68ea3c1dd as std::ffi::c_ulonglong as __s64,
    0xd96f9fbbdb as std::ffi::c_ulonglong as __s64,
    0xdc5095f744 as std::ffi::c_ulonglong as __s64,
    0xdf31867430 as std::ffi::c_ulonglong as __s64,
    0xe2127132b5 as std::ffi::c_ulonglong as __s64,
    0xe4f35632ea as std::ffi::c_ulonglong as __s64,
    0xe7d43574e6 as std::ffi::c_ulonglong as __s64,
    0xeab50ef8c1 as std::ffi::c_ulonglong as __s64,
    0xed95e2be90 as std::ffi::c_ulonglong as __s64,
    0xf076b0c66c as std::ffi::c_ulonglong as __s64,
    0xf35779106a as std::ffi::c_ulonglong as __s64,
    0xf6383b9ca2 as std::ffi::c_ulonglong as __s64,
    0xf918f86b2a as std::ffi::c_ulonglong as __s64,
    0xfbf9af7c1a as std::ffi::c_ulonglong as __s64,
    0xfeda60cf88 as std::ffi::c_ulonglong as __s64,
    0x101bb0c658c as std::ffi::c_ulonglong as __s64,
    0x1049bb23e3c as std::ffi::c_ulonglong as __s64,
    0x1077c5259af as std::ffi::c_ulonglong as __s64,
    0x10a5cecb7fc as std::ffi::c_ulonglong as __s64,
    0x10d3d81593a as std::ffi::c_ulonglong as __s64,
    0x1101e103d7f as std::ffi::c_ulonglong as __s64,
    0x112fe9964e4 as std::ffi::c_ulonglong as __s64,
    0x115df1ccf7e as std::ffi::c_ulonglong as __s64,
    0x118bf9a7d64 as std::ffi::c_ulonglong as __s64,
    0x11ba0126ead as std::ffi::c_ulonglong as __s64,
    0x11e8084a371 as std::ffi::c_ulonglong as __s64,
    0x12160f11bc6 as std::ffi::c_ulonglong as __s64,
    0x1244157d7c3 as std::ffi::c_ulonglong as __s64,
    0x12721b8d77f as std::ffi::c_ulonglong as __s64,
    0x12a02141b10 as std::ffi::c_ulonglong as __s64,
    0x12ce269a28e as std::ffi::c_ulonglong as __s64,
    0x12fc2b96e0f as std::ffi::c_ulonglong as __s64,
    0x132a3037daa as std::ffi::c_ulonglong as __s64,
    0x1358347d177 as std::ffi::c_ulonglong as __s64,
    0x1386386698c as std::ffi::c_ulonglong as __s64,
    0x13b43bf45ff as std::ffi::c_ulonglong as __s64,
    0x13e23f266e9 as std::ffi::c_ulonglong as __s64,
    0x141041fcc5e as std::ffi::c_ulonglong as __s64,
    0x143e4477678 as std::ffi::c_ulonglong as __s64,
    0x146c469654b as std::ffi::c_ulonglong as __s64,
    0x149a48598f0 as std::ffi::c_ulonglong as __s64,
    0x14c849c117c as std::ffi::c_ulonglong as __s64,
    0x14f64accf08 as std::ffi::c_ulonglong as __s64,
    0x15244b7d1a9 as std::ffi::c_ulonglong as __s64,
    0x15524bd1976 as std::ffi::c_ulonglong as __s64,
    0x15804bca687 as std::ffi::c_ulonglong as __s64,
    0x15ae4b678f2 as std::ffi::c_ulonglong as __s64,
    0x15dc4aa90ce as std::ffi::c_ulonglong as __s64,
    0x160a498ee31 as std::ffi::c_ulonglong as __s64,
    0x16384819134 as std::ffi::c_ulonglong as __s64,
    0x166646479ec as std::ffi::c_ulonglong as __s64,
    0x1694441a870 as std::ffi::c_ulonglong as __s64,
    0x16c24191cd7 as std::ffi::c_ulonglong as __s64,
    0x16df6ca19bd as std::ffi::c_ulonglong as __s64,
    0x171e3b6d7aa as std::ffi::c_ulonglong as __s64,
    0x174c37d1e44 as std::ffi::c_ulonglong as __s64,
    0x177a33dab1c as std::ffi::c_ulonglong as __s64,
    0x17a82f87e49 as std::ffi::c_ulonglong as __s64,
    0x17d62ad97e2 as std::ffi::c_ulonglong as __s64,
    0x180425cf7fe as std::ffi::c_ulonglong as __s64,
    0x182b07f3458 as std::ffi::c_ulonglong as __s64,
    0x18601aa8c19 as std::ffi::c_ulonglong as __s64,
    0x188e148c046 as std::ffi::c_ulonglong as __s64,
    0x18bc0e13b52 as std::ffi::c_ulonglong as __s64,
    0x18ea073fd52 as std::ffi::c_ulonglong as __s64,
    0x1918001065d as std::ffi::c_ulonglong as __s64,
    0x1945f88568b as std::ffi::c_ulonglong as __s64,
    0x1973f09edf2 as std::ffi::c_ulonglong as __s64,
    0x19a1e85ccaa as std::ffi::c_ulonglong as __s64,
    0x19cfdfbf2c8 as std::ffi::c_ulonglong as __s64,
    0x19fdd6c6063 as std::ffi::c_ulonglong as __s64,
    0x1a2bcd71593 as std::ffi::c_ulonglong as __s64,
    0x1a59c3c126e as std::ffi::c_ulonglong as __s64,
    0x1a87b9b570b as std::ffi::c_ulonglong as __s64,
    0x1ab5af4e380 as std::ffi::c_ulonglong as __s64,
    0x1ae3a48b7e5 as std::ffi::c_ulonglong as __s64,
    0x1b11996d450 as std::ffi::c_ulonglong as __s64,
    0x1b3f8df38d9 as std::ffi::c_ulonglong as __s64,
    0x1b6d821e595 as std::ffi::c_ulonglong as __s64,
    0x1b9b75eda9b as std::ffi::c_ulonglong as __s64,
    0x1bc96961803 as std::ffi::c_ulonglong as __s64,
    0x1bf75c79de3 as std::ffi::c_ulonglong as __s64,
    0x1c254f36c51 as std::ffi::c_ulonglong as __s64,
    0x1c534198365 as std::ffi::c_ulonglong as __s64,
    0x1c81339e336 as std::ffi::c_ulonglong as __s64,
    0x1caf2548bd9 as std::ffi::c_ulonglong as __s64,
    0x1cdd1697d67 as std::ffi::c_ulonglong as __s64,
    0x1d0b078b7f5 as std::ffi::c_ulonglong as __s64,
    0x1d38f823b9a as std::ffi::c_ulonglong as __s64,
    0x1d66e86086d as std::ffi::c_ulonglong as __s64,
    0x1d94d841e86 as std::ffi::c_ulonglong as __s64,
    0x1dc2c7c7df9 as std::ffi::c_ulonglong as __s64,
    0x1df0b6f26df as std::ffi::c_ulonglong as __s64,
    0x1e1ea5c194e as std::ffi::c_ulonglong as __s64,
    0x1e4c943555d as std::ffi::c_ulonglong as __s64,
    0x1e7a824db23 as std::ffi::c_ulonglong as __s64,
    0x1ea8700aab5 as std::ffi::c_ulonglong as __s64,
    0x1ed65d6c42b as std::ffi::c_ulonglong as __s64,
    0x1f044a7279d as std::ffi::c_ulonglong as __s64,
    0x1f32371d51f as std::ffi::c_ulonglong as __s64,
    0x1f60236ccca as std::ffi::c_ulonglong as __s64,
    0x1f8e0f60eb3 as std::ffi::c_ulonglong as __s64,
    0x1fbbfaf9af3 as std::ffi::c_ulonglong as __s64,
    0x1fe9e63719e as std::ffi::c_ulonglong as __s64,
    0x2017d1192cc as std::ffi::c_ulonglong as __s64,
    0x2045bb9fe94 as std::ffi::c_ulonglong as __s64,
    0x2073a5cb50d as std::ffi::c_ulonglong as __s64,
    0x209c06e6212 as std::ffi::c_ulonglong as __s64,
    0x20cf791026a as std::ffi::c_ulonglong as __s64,
    0x20fd622997c as std::ffi::c_ulonglong as __s64,
    0x212b07f3458 as std::ffi::c_ulonglong as __s64,
    0x2159334a8d8 as std::ffi::c_ulonglong as __s64,
    0x21871b52150 as std::ffi::c_ulonglong as __s64,
    0x21b502fe517 as std::ffi::c_ulonglong as __s64,
    0x21d6a73a78f as std::ffi::c_ulonglong as __s64,
    0x2210d144eee as std::ffi::c_ulonglong as __s64,
    0x223eb7df52c as std::ffi::c_ulonglong as __s64,
    0x226c9e1e713 as std::ffi::c_ulonglong as __s64,
    0x229a84024bb as std::ffi::c_ulonglong as __s64,
    0x22c23679b4e as std::ffi::c_ulonglong as __s64,
    0x22f64eb83a8 as std::ffi::c_ulonglong as __s64,
    0x2324338a51b as std::ffi::c_ulonglong as __s64,
    0x235218012a9 as std::ffi::c_ulonglong as __s64,
    0x237ffc1cc69 as std::ffi::c_ulonglong as __s64,
    0x23a2c3b0ea4 as std::ffi::c_ulonglong as __s64,
    0x23d13ee805b as std::ffi::c_ulonglong as __s64,
    0x24035e9221f as std::ffi::c_ulonglong as __s64,
    0x243788faf25 as std::ffi::c_ulonglong as __s64,
    0x24656b4e735 as std::ffi::c_ulonglong as __s64,
    0x247ed646bfe as std::ffi::c_ulonglong as __s64,
    0x24c12ee3d98 as std::ffi::c_ulonglong as __s64,
    0x24ef1025c1a as std::ffi::c_ulonglong as __s64,
    0x251cf10c799 as std::ffi::c_ulonglong as __s64,
    0x25492644d65 as std::ffi::c_ulonglong as __s64,
    0x2578b1c85ee as std::ffi::c_ulonglong as __s64,
    0x25a6919d8f0 as std::ffi::c_ulonglong as __s64,
    0x25d13ee805b as std::ffi::c_ulonglong as __s64,
    0x26025036716 as std::ffi::c_ulonglong as __s64,
    0x26296453882 as std::ffi::c_ulonglong as __s64,
    0x265e0d62b53 as std::ffi::c_ulonglong as __s64,
    0x268beb701f3 as std::ffi::c_ulonglong as __s64,
    0x26b9c92265e as std::ffi::c_ulonglong as __s64,
    0x26d32f798a9 as std::ffi::c_ulonglong as __s64,
    0x271583758eb as std::ffi::c_ulonglong as __s64,
    0x2743601673b as std::ffi::c_ulonglong as __s64,
    0x27713c5c3b0 as std::ffi::c_ulonglong as __s64,
    0x279f1846e5f as std::ffi::c_ulonglong as __s64,
    0x27ccf3d6761 as std::ffi::c_ulonglong as __s64,
    0x27e6580aecb as std::ffi::c_ulonglong as __s64,
    0x2828a9e44b3 as std::ffi::c_ulonglong as __s64,
    0x28568462932 as std::ffi::c_ulonglong as __s64,
    0x287bdbf5255 as std::ffi::c_ulonglong as __s64,
    0x28b2384de4a as std::ffi::c_ulonglong as __s64,
    0x28d13ee805b as std::ffi::c_ulonglong as __s64,
    0x29035e9221f as std::ffi::c_ulonglong as __s64,
    0x29296453882 as std::ffi::c_ulonglong as __s64,
    0x29699bdfb61 as std::ffi::c_ulonglong as __s64,
    0x29902a37aab as std::ffi::c_ulonglong as __s64,
    0x29c54b864c9 as std::ffi::c_ulonglong as __s64,
    0x29deabd1083 as std::ffi::c_ulonglong as __s64,
    0x2a20f9c0bb5 as std::ffi::c_ulonglong as __s64,
    0x2a4c7605d61 as std::ffi::c_ulonglong as __s64,
    0x2a7bdbf5255 as std::ffi::c_ulonglong as __s64,
    0x2a96056dafc as std::ffi::c_ulonglong as __s64,
    0x2ac3daf14ef as std::ffi::c_ulonglong as __s64,
    0x2af1b019eca as std::ffi::c_ulonglong as __s64,
    0x2b296453882 as std::ffi::c_ulonglong as __s64,
    0x2b5d022d80f as std::ffi::c_ulonglong as __s64,
    0x2b8fa471cb3 as std::ffi::c_ulonglong as __s64,
    0x2ba9012e713 as std::ffi::c_ulonglong as __s64,
    0x2bd6d4901cc as std::ffi::c_ulonglong as __s64,
    0x2c04a796cf6 as std::ffi::c_ulonglong as __s64,
    0x2c327a428a6 as std::ffi::c_ulonglong as __s64,
    0x2c61a5e8f4c as std::ffi::c_ulonglong as __s64,
    0x2c8e1e891f6 as std::ffi::c_ulonglong as __s64,
    0x2cbbf023fc2 as std::ffi::c_ulonglong as __s64,
    0x2ce9c163e6e as std::ffi::c_ulonglong as __s64,
    0x2d179248e13 as std::ffi::c_ulonglong as __s64,
    0x2d4562d2ec6 as std::ffi::c_ulonglong as __s64,
    0x2d73330209d as std::ffi::c_ulonglong as __s64,
    0x2da102d63b0 as std::ffi::c_ulonglong as __s64,
    0x2dced24f814 as std::ffi::c_ulonglong as __s64,
];
#[no_mangle]
pub unsafe extern "C" fn crush_find_rule(
    mut map: *const crush_map,
    mut ruleset: std::ffi::c_int,
    mut type_0: std::ffi::c_int,
    mut size: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i: __u32 = 0;
    i = 0 as std::ffi::c_int as __u32;
    while i < (*map).max_rules {
        if !(*((*map).rules).offset(i as isize)).is_null()
            && (**((*map).rules).offset(i as isize)).mask.ruleset as std::ffi::c_int
                == ruleset
            && (**((*map).rules).offset(i as isize)).mask.type_0 as std::ffi::c_int
                == type_0
            && (**((*map).rules).offset(i as isize)).mask.min_size as std::ffi::c_int
                <= size
            && (**((*map).rules).offset(i as isize)).mask.max_size as std::ffi::c_int
                >= size
        {
            return i as std::ffi::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return -(1 as std::ffi::c_int);
}
unsafe extern "C" fn bucket_perm_choose(
    mut bucket: *const crush_bucket,
    mut work: *mut crush_work_bucket,
    mut x: std::ffi::c_int,
    mut r: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut current_block: u64;
    let mut pr: std::ffi::c_uint = r as __u32 % (*bucket).size;
    let mut i: std::ffi::c_uint = 0;
    let mut s: std::ffi::c_uint = 0;
    if (*work).perm_x != x as __u32 || (*work).perm_n == 0 as std::ffi::c_int as __u32 {
        (*work).perm_x = x as __u32;
        if pr == 0 as std::ffi::c_int as std::ffi::c_uint {
            s = crush_hash32_3(
                (*bucket).hash as std::ffi::c_int,
                x as __u32,
                (*bucket).id as __u32,
                0 as std::ffi::c_int as __u32,
            ) % (*bucket).size;
            *((*work).perm).offset(0 as std::ffi::c_int as isize) = s;
            (*work).perm_n = 0xffff as std::ffi::c_int as __u32;
            current_block = 3275366147856559585;
        } else {
            i = 0 as std::ffi::c_int as std::ffi::c_uint;
            while i < (*bucket).size {
                *((*work).perm).offset(i as isize) = i;
                i = i.wrapping_add(1);
                i;
            }
            (*work).perm_n = 0 as std::ffi::c_int as __u32;
            current_block = 13056961889198038528;
        }
    } else {
        if (*work).perm_n == 0xffff as std::ffi::c_int as __u32 {
            i = 1 as std::ffi::c_int as std::ffi::c_uint;
            while i < (*bucket).size {
                *((*work).perm).offset(i as isize) = i;
                i = i.wrapping_add(1);
                i;
            }
            *((*work).perm)
                .offset(
                    *((*work).perm).offset(0 as std::ffi::c_int as isize) as isize,
                ) = 0 as std::ffi::c_int as __u32;
            (*work).perm_n = 1 as std::ffi::c_int as __u32;
        }
        current_block = 13056961889198038528;
    }
    match current_block {
        13056961889198038528 => {
            i = 0 as std::ffi::c_int as std::ffi::c_uint;
            while i < (*work).perm_n {
                i = i.wrapping_add(1);
                i;
            }
            while (*work).perm_n <= pr {
                let mut p: std::ffi::c_uint = (*work).perm_n;
                if p < ((*bucket).size).wrapping_sub(1 as std::ffi::c_int as __u32) {
                    i = (crush_hash32_3(
                        (*bucket).hash as std::ffi::c_int,
                        x as __u32,
                        (*bucket).id as __u32,
                        p,
                    ))
                        .wrapping_rem(((*bucket).size).wrapping_sub(p));
                    if i != 0 {
                        let mut t: std::ffi::c_uint = *((*work).perm)
                            .offset(p.wrapping_add(i) as isize);
                        *((*work).perm)
                            .offset(
                                p.wrapping_add(i) as isize,
                            ) = *((*work).perm).offset(p as isize);
                        *((*work).perm).offset(p as isize) = t;
                    }
                }
                (*work).perm_n = ((*work).perm_n).wrapping_add(1);
                (*work).perm_n;
            }
            i = 0 as std::ffi::c_int as std::ffi::c_uint;
            while i < (*bucket).size {
                i = i.wrapping_add(1);
                i;
            }
            s = *((*work).perm).offset(pr as isize);
        }
        _ => {}
    }
    return *((*bucket).items).offset(s as isize);
}
unsafe extern "C" fn bucket_uniform_choose(
    mut bucket: *const crush_bucket_uniform,
    mut work: *mut crush_work_bucket,
    mut x: std::ffi::c_int,
    mut r: std::ffi::c_int,
) -> std::ffi::c_int {
    return bucket_perm_choose(&(*bucket).h, work, x, r);
}
unsafe extern "C" fn bucket_list_choose(
    mut bucket: *const crush_bucket_list,
    mut x: std::ffi::c_int,
    mut r: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i: std::ffi::c_int = 0;
    i = ((*bucket).h.size).wrapping_sub(1 as std::ffi::c_int as __u32)
        as std::ffi::c_int;
    while i >= 0 as std::ffi::c_int {
        let mut w: __u64 = crush_hash32_4(
            (*bucket).h.hash as std::ffi::c_int,
            x as __u32,
            *((*bucket).h.items).offset(i as isize) as __u32,
            r as __u32,
            (*bucket).h.id as __u32,
        ) as __u64;
        w &= 0xffff as std::ffi::c_int as __u64;
        w = w * *((*bucket).sum_weights).offset(i as isize) as __u64;
        w = w >> 16 as std::ffi::c_int;
        if w < *((*bucket).item_weights).offset(i as isize) as __u64 {
            return *((*bucket).h.items).offset(i as isize);
        }
        i -= 1;
        i;
    }
    return *((*bucket).h.items).offset(0 as std::ffi::c_int as isize);
}
unsafe extern "C" fn height(mut n: std::ffi::c_int) -> std::ffi::c_int {
    let mut h: std::ffi::c_int = 0 as std::ffi::c_int;
    while n & 1 as std::ffi::c_int == 0 as std::ffi::c_int {
        h += 1;
        h;
        n = n >> 1 as std::ffi::c_int;
    }
    return h;
}
unsafe extern "C" fn left(mut x: std::ffi::c_int) -> std::ffi::c_int {
    let mut h: std::ffi::c_int = height(x);
    return x - ((1 as std::ffi::c_int) << h - 1 as std::ffi::c_int);
}
unsafe extern "C" fn right(mut x: std::ffi::c_int) -> std::ffi::c_int {
    let mut h: std::ffi::c_int = height(x);
    return x + ((1 as std::ffi::c_int) << h - 1 as std::ffi::c_int);
}
unsafe extern "C" fn terminal(mut x: std::ffi::c_int) -> std::ffi::c_int {
    return x & 1 as std::ffi::c_int;
}
unsafe extern "C" fn bucket_tree_choose(
    mut bucket: *const crush_bucket_tree,
    mut x: std::ffi::c_int,
    mut r: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut n: std::ffi::c_int = 0;
    let mut w: __u32 = 0;
    let mut t: __u64 = 0;
    n = (*bucket).num_nodes as std::ffi::c_int >> 1 as std::ffi::c_int;
    while terminal(n) == 0 {
        let mut l: std::ffi::c_int = 0;
        w = *((*bucket).node_weights).offset(n as isize);
        t = crush_hash32_4(
            (*bucket).h.hash as std::ffi::c_int,
            x as __u32,
            n as __u32,
            r as __u32,
            (*bucket).h.id as __u32,
        ) as __u64 * w as __u64;
        t = t >> 32 as std::ffi::c_int;
        l = left(n);
        if t < *((*bucket).node_weights).offset(l as isize) as __u64 {
            n = l;
        } else {
            n = right(n);
        }
    }
    return *((*bucket).h.items).offset((n >> 1 as std::ffi::c_int) as isize);
}
unsafe extern "C" fn bucket_straw_choose(
    mut bucket: *const crush_bucket_straw,
    mut x: std::ffi::c_int,
    mut r: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i: __u32 = 0;
    let mut high: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut high_draw: __u64 = 0 as std::ffi::c_int as __u64;
    let mut draw: __u64 = 0;
    i = 0 as std::ffi::c_int as __u32;
    while i < (*bucket).h.size {
        draw = crush_hash32_3(
            (*bucket).h.hash as std::ffi::c_int,
            x as __u32,
            *((*bucket).h.items).offset(i as isize) as __u32,
            r as __u32,
        ) as __u64;
        draw &= 0xffff as std::ffi::c_int as __u64;
        draw = draw * *((*bucket).straws).offset(i as isize) as __u64;
        if i == 0 as std::ffi::c_int as __u32 || draw > high_draw {
            high = i as std::ffi::c_int;
            high_draw = draw;
        }
        i = i.wrapping_add(1);
        i;
    }
    return *((*bucket).h.items).offset(high as isize);
}
unsafe extern "C" fn crush_ln(mut xin: std::ffi::c_uint) -> __u64 {
    let mut x: std::ffi::c_uint = xin;
    let mut iexpon: std::ffi::c_int = 0;
    let mut index1: std::ffi::c_int = 0;
    let mut index2: std::ffi::c_int = 0;
    let mut RH: __u64 = 0;
    let mut LH: __u64 = 0;
    let mut LL: __u64 = 0;
    let mut xl64: __u64 = 0;
    let mut result: __u64 = 0;
    x = x.wrapping_add(1);
    x;
    iexpon = 15 as std::ffi::c_int;
    if x & 0x18000 as std::ffi::c_int as std::ffi::c_uint == 0 {
        let mut bits: std::ffi::c_int = (x
            & 0x1ffff as std::ffi::c_int as std::ffi::c_uint)
            .leading_zeros() as i32 - 16 as std::ffi::c_int;
        x <<= bits;
        iexpon = 15 as std::ffi::c_int - bits;
    }
    index1 = ((x >> 8 as std::ffi::c_int) << 1 as std::ffi::c_int) as std::ffi::c_int;
    RH = __RH_LH_tbl[(index1 - 256 as std::ffi::c_int) as usize] as __u64;
    LH = __RH_LH_tbl[(index1 + 1 as std::ffi::c_int - 256 as std::ffi::c_int) as usize]
        as __u64;
    xl64 = x as __s64 as __u64 * RH;
    xl64 >>= 48 as std::ffi::c_int;
    result = iexpon as __u64;
    result <<= 12 as std::ffi::c_int + 32 as std::ffi::c_int;
    index2 = (xl64 & 0xff as std::ffi::c_int as __u64) as std::ffi::c_int;
    LL = __LL_tbl[index2 as usize] as __u64;
    LH = LH.wrapping_add(LL);
    LH >>= 48 as std::ffi::c_int - 12 as std::ffi::c_int - 32 as std::ffi::c_int;
    result = result.wrapping_add(LH);
    return result;
}
#[inline]
unsafe extern "C" fn get_choose_arg_weights(
    mut bucket: *const crush_bucket_straw2,
    mut arg: *const crush_choose_arg,
    mut position: std::ffi::c_int,
) -> *mut __u32 {
    if arg.is_null() || ((*arg).weight_set).is_null()
        || (*arg).weight_set_size == 0 as std::ffi::c_int as __u32
    {
        return (*bucket).item_weights;
    }
    if position as __u32 >= (*arg).weight_set_size {
        position = ((*arg).weight_set_size).wrapping_sub(1 as std::ffi::c_int as __u32)
            as std::ffi::c_int;
    }
    return (*((*arg).weight_set).offset(position as isize)).weights;
}
#[inline]
unsafe extern "C" fn get_choose_arg_ids(
    mut bucket: *const crush_bucket_straw2,
    mut arg: *const crush_choose_arg,
) -> *mut std::ffi::c_int {
    if arg.is_null() || ((*arg).ids).is_null() {
        return (*bucket).h.items;
    }
    return (*arg).ids;
}
unsafe extern "C" fn bucket_straw2_choose(
    mut bucket: *const crush_bucket_straw2,
    mut x: std::ffi::c_int,
    mut r: std::ffi::c_int,
    mut arg: *const crush_choose_arg,
    mut position: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i: std::ffi::c_uint = 0;
    let mut high: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut u: std::ffi::c_uint = 0;
    let mut ln: __s64 = 0;
    let mut draw: __s64 = 0;
    let mut high_draw: __s64 = 0 as std::ffi::c_int as __s64;
    let mut weights: *mut __u32 = get_choose_arg_weights(bucket, arg, position);
    let mut ids: *mut std::ffi::c_int = get_choose_arg_ids(bucket, arg);
    i = 0 as std::ffi::c_int as std::ffi::c_uint;
    while i < (*bucket).h.size {
        if *weights.offset(i as isize) != 0 {
            u = crush_hash32_3(
                (*bucket).h.hash as std::ffi::c_int,
                x as __u32,
                *ids.offset(i as isize) as __u32,
                r as __u32,
            );
            u &= 0xffff as std::ffi::c_int as std::ffi::c_uint;
            ln = (crush_ln(u))
                .wrapping_sub(0x1000000000000 as std::ffi::c_longlong as __u64) as __s64;
            draw = ln / *weights.offset(i as isize) as __s64;
        } else {
            draw = -((!(0 as std::ffi::c_ulonglong) >> 1 as std::ffi::c_int) as __s64)
                - 1 as std::ffi::c_int as __s64;
        }
        if i == 0 as std::ffi::c_int as std::ffi::c_uint || draw > high_draw {
            high = i;
            high_draw = draw;
        }
        i = i.wrapping_add(1);
        i;
    }
    return *((*bucket).h.items).offset(high as isize);
}
unsafe extern "C" fn crush_bucket_choose(
    mut in_0: *const crush_bucket,
    mut work: *mut crush_work_bucket,
    mut x: std::ffi::c_int,
    mut r: std::ffi::c_int,
    mut arg: *const crush_choose_arg,
    mut position: std::ffi::c_int,
) -> std::ffi::c_int {
    if !((*in_0).size == 0 as std::ffi::c_int as __u32) {} else {
        __assert_fail(
            b"!(in->size == 0)\0" as *const u8 as *const std::ffi::c_char,
            b"/home/sevki/src/libcrush/crush/mapper.c\0" as *const u8
                as *const std::ffi::c_char,
            378 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 129],
                &[std::ffi::c_char; 129],
            >(
                b"int crush_bucket_choose(const struct crush_bucket *, struct crush_work_bucket *, int, int, const struct crush_choose_arg *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5382: {
        if !((*in_0).size == 0 as std::ffi::c_int as __u32) {} else {
            __assert_fail(
                b"!(in->size == 0)\0" as *const u8 as *const std::ffi::c_char,
                b"/home/sevki/src/libcrush/crush/mapper.c\0" as *const u8
                    as *const std::ffi::c_char,
                378 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 129],
                    &[std::ffi::c_char; 129],
                >(
                    b"int crush_bucket_choose(const struct crush_bucket *, struct crush_work_bucket *, int, int, const struct crush_choose_arg *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    match (*in_0).alg as std::ffi::c_int {
        1 => {
            return bucket_uniform_choose(in_0 as *const crush_bucket_uniform, work, x, r);
        }
        2 => return bucket_list_choose(in_0 as *const crush_bucket_list, x, r),
        3 => return bucket_tree_choose(in_0 as *const crush_bucket_tree, x, r),
        4 => return bucket_straw_choose(in_0 as *const crush_bucket_straw, x, r),
        5 => {
            return bucket_straw2_choose(
                in_0 as *const crush_bucket_straw2,
                x,
                r,
                arg,
                position,
            );
        }
        _ => return *((*in_0).items).offset(0 as std::ffi::c_int as isize),
    };
}
unsafe extern "C" fn is_out(
    mut map: *const crush_map,
    mut weight: *const __u32,
    mut weight_max: std::ffi::c_int,
    mut item: std::ffi::c_int,
    mut x: std::ffi::c_int,
) -> std::ffi::c_int {
    if item >= weight_max {
        return 1 as std::ffi::c_int;
    }
    if *weight.offset(item as isize) >= 0x10000 as std::ffi::c_int as __u32 {
        return 0 as std::ffi::c_int;
    }
    if *weight.offset(item as isize) == 0 as std::ffi::c_int as __u32 {
        return 1 as std::ffi::c_int;
    }
    if (crush_hash32_2(0 as std::ffi::c_int, x as __u32, item as __u32)
        & 0xffff as std::ffi::c_int as __u32) < *weight.offset(item as isize)
    {
        return 0 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn crush_choose_firstn(
    mut map: *const crush_map,
    mut work: *mut crush_work,
    mut bucket: *const crush_bucket,
    mut weight: *const __u32,
    mut weight_max: std::ffi::c_int,
    mut x: std::ffi::c_int,
    mut numrep: std::ffi::c_int,
    mut type_0: std::ffi::c_int,
    mut out: *mut std::ffi::c_int,
    mut outpos: std::ffi::c_int,
    mut out_size: std::ffi::c_int,
    mut tries: std::ffi::c_uint,
    mut recurse_tries: std::ffi::c_uint,
    mut local_retries: std::ffi::c_uint,
    mut local_fallback_retries: std::ffi::c_uint,
    mut recurse_to_leaf: std::ffi::c_int,
    mut vary_r: std::ffi::c_uint,
    mut stable: std::ffi::c_uint,
    mut out2: *mut std::ffi::c_int,
    mut parent_r: std::ffi::c_int,
    mut choose_args: *const crush_choose_arg,
) -> std::ffi::c_int {
    let mut rep: std::ffi::c_int = 0;
    let mut ftotal: std::ffi::c_uint = 0;
    let mut flocal: std::ffi::c_uint = 0;
    let mut retry_descent: std::ffi::c_int = 0;
    let mut retry_bucket: std::ffi::c_int = 0;
    let mut skip_rep: std::ffi::c_int = 0;
    let mut in_0: *const crush_bucket = bucket;
    let mut r: std::ffi::c_int = 0;
    let mut i: std::ffi::c_int = 0;
    let mut item: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut itemtype: std::ffi::c_int = 0;
    let mut collide: std::ffi::c_int = 0;
    let mut reject: std::ffi::c_int = 0;
    let mut count: std::ffi::c_int = out_size;
    rep = if stable != 0 { 0 as std::ffi::c_int } else { outpos };
    while rep < numrep && count > 0 as std::ffi::c_int {
        ftotal = 0 as std::ffi::c_int as std::ffi::c_uint;
        skip_rep = 0 as std::ffi::c_int;
        loop {
            retry_descent = 0 as std::ffi::c_int;
            in_0 = bucket;
            flocal = 0 as std::ffi::c_int as std::ffi::c_uint;
            let mut current_block_55: u64;
            loop {
                collide = 0 as std::ffi::c_int;
                retry_bucket = 0 as std::ffi::c_int;
                r = rep + parent_r;
                r = (r as std::ffi::c_uint).wrapping_add(ftotal) as std::ffi::c_int
                    as std::ffi::c_int;
                if (*in_0).size == 0 as std::ffi::c_int as __u32 {
                    reject = 1 as std::ffi::c_int;
                    current_block_55 = 11884163591463421855;
                } else {
                    if local_fallback_retries > 0 as std::ffi::c_int as std::ffi::c_uint
                        && flocal >= (*in_0).size >> 1 as std::ffi::c_int
                        && flocal > local_fallback_retries
                    {
                        item = bucket_perm_choose(
                            in_0,
                            *((*work).work)
                                .offset((-(1 as std::ffi::c_int) - (*in_0).id) as isize),
                            x,
                            r,
                        );
                    } else {
                        item = crush_bucket_choose(
                            in_0,
                            *((*work).work)
                                .offset((-(1 as std::ffi::c_int) - (*in_0).id) as isize),
                            x,
                            r,
                            if !choose_args.is_null() {
                                &*choose_args
                                    .offset((-(1 as std::ffi::c_int) - (*in_0).id) as isize)
                            } else {
                                0 as *const crush_choose_arg
                            },
                            outpos,
                        );
                    }
                    if item >= (*map).max_devices {
                        skip_rep = 1 as std::ffi::c_int;
                        break;
                    } else {
                        if item < 0 as std::ffi::c_int {
                            itemtype = (**((*map).buckets)
                                .offset((-(1 as std::ffi::c_int) - item) as isize))
                                .type_0 as std::ffi::c_int;
                        } else {
                            itemtype = 0 as std::ffi::c_int;
                        }
                        if itemtype != type_0 {
                            if item >= 0 as std::ffi::c_int
                                || -(1 as std::ffi::c_int) - item >= (*map).max_buckets
                            {
                                skip_rep = 1 as std::ffi::c_int;
                                break;
                            } else {
                                in_0 = *((*map).buckets)
                                    .offset((-(1 as std::ffi::c_int) - item) as isize);
                                retry_bucket = 1 as std::ffi::c_int;
                            }
                            current_block_55 = 13109137661213826276;
                        } else {
                            i = 0 as std::ffi::c_int;
                            while i < outpos {
                                if *out.offset(i as isize) == item {
                                    collide = 1 as std::ffi::c_int;
                                    break;
                                } else {
                                    i += 1;
                                    i;
                                }
                            }
                            reject = 0 as std::ffi::c_int;
                            if collide == 0 && recurse_to_leaf != 0 {
                                if item < 0 as std::ffi::c_int {
                                    let mut sub_r: std::ffi::c_int = 0;
                                    if vary_r != 0 {
                                        sub_r = r
                                            >> vary_r
                                                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
                                    } else {
                                        sub_r = 0 as std::ffi::c_int;
                                    }
                                    if crush_choose_firstn(
                                        map,
                                        work,
                                        *((*map).buckets)
                                            .offset((-(1 as std::ffi::c_int) - item) as isize),
                                        weight,
                                        weight_max,
                                        x,
                                        (if stable != 0 {
                                            1 as std::ffi::c_int
                                        } else {
                                            outpos + 1 as std::ffi::c_int
                                        }),
                                        0 as std::ffi::c_int,
                                        out2,
                                        outpos,
                                        count,
                                        recurse_tries,
                                        0 as std::ffi::c_int as std::ffi::c_uint,
                                        local_retries,
                                        local_fallback_retries,
                                        0 as std::ffi::c_int,
                                        vary_r,
                                        stable,
                                        0 as *mut std::ffi::c_int,
                                        sub_r,
                                        choose_args,
                                    ) <= outpos
                                    {
                                        reject = 1 as std::ffi::c_int;
                                    }
                                } else {
                                    *out2.offset(outpos as isize) = item;
                                }
                            }
                            if reject == 0 && collide == 0 {
                                if itemtype == 0 as std::ffi::c_int {
                                    reject = is_out(map, weight, weight_max, item, x);
                                }
                            }
                            current_block_55 = 11884163591463421855;
                        }
                    }
                }
                match current_block_55 {
                    11884163591463421855 => {
                        if reject != 0 || collide != 0 {
                            ftotal = ftotal.wrapping_add(1);
                            ftotal;
                            flocal = flocal.wrapping_add(1);
                            flocal;
                            if collide != 0 && flocal <= local_retries {
                                retry_bucket = 1 as std::ffi::c_int;
                            } else if local_fallback_retries
                                > 0 as std::ffi::c_int as std::ffi::c_uint
                                && flocal
                                    <= ((*in_0).size).wrapping_add(local_fallback_retries)
                            {
                                retry_bucket = 1 as std::ffi::c_int;
                            } else if ftotal < tries {
                                retry_descent = 1 as std::ffi::c_int;
                            } else {
                                skip_rep = 1 as std::ffi::c_int;
                            }
                        }
                    }
                    _ => {}
                }
                if !(retry_bucket != 0) {
                    break;
                }
            }
            if !(retry_descent != 0) {
                break;
            }
        }
        if !(skip_rep != 0) {
            *out.offset(outpos as isize) = item;
            outpos += 1;
            outpos;
            count -= 1;
            count;
            if !((*map).choose_tries).is_null() && ftotal <= (*map).choose_total_tries {
                let ref mut fresh0 = *((*map).choose_tries).offset(ftotal as isize);
                *fresh0 = (*fresh0).wrapping_add(1);
                *fresh0;
            }
        }
        rep += 1;
        rep;
    }
    return outpos;
}
unsafe extern "C" fn crush_choose_indep(
    mut map: *const crush_map,
    mut work: *mut crush_work,
    mut bucket: *const crush_bucket,
    mut weight: *const __u32,
    mut weight_max: std::ffi::c_int,
    mut x: std::ffi::c_int,
    mut left_0: std::ffi::c_int,
    mut numrep: std::ffi::c_int,
    mut type_0: std::ffi::c_int,
    mut out: *mut std::ffi::c_int,
    mut outpos: std::ffi::c_int,
    mut tries: std::ffi::c_uint,
    mut recurse_tries: std::ffi::c_uint,
    mut recurse_to_leaf: std::ffi::c_int,
    mut out2: *mut std::ffi::c_int,
    mut parent_r: std::ffi::c_int,
    mut choose_args: *const crush_choose_arg,
) {
    let mut in_0: *const crush_bucket = bucket;
    let mut endpos: std::ffi::c_int = outpos + left_0;
    let mut rep: std::ffi::c_int = 0;
    let mut ftotal: std::ffi::c_uint = 0;
    let mut r: std::ffi::c_int = 0;
    let mut i: std::ffi::c_int = 0;
    let mut item: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut itemtype: std::ffi::c_int = 0;
    let mut collide: std::ffi::c_int = 0;
    rep = outpos;
    while rep < endpos {
        *out.offset(rep as isize) = 0x7ffffffe as std::ffi::c_int;
        if !out2.is_null() {
            *out2.offset(rep as isize) = 0x7ffffffe as std::ffi::c_int;
        }
        rep += 1;
        rep;
    }
    ftotal = 0 as std::ffi::c_int as std::ffi::c_uint;
    while left_0 > 0 as std::ffi::c_int && ftotal < tries {
        rep = outpos;
        while rep < endpos {
            if !(*out.offset(rep as isize) != 0x7ffffffe as std::ffi::c_int) {
                in_0 = bucket;
                loop {
                    r = rep + parent_r;
                    if (*in_0).alg as std::ffi::c_int
                        == CRUSH_BUCKET_UNIFORM as std::ffi::c_int
                        && (*in_0).size % numrep as __u32
                            == 0 as std::ffi::c_int as __u32
                    {
                        r = (r as std::ffi::c_uint)
                            .wrapping_add(
                                ((numrep + 1 as std::ffi::c_int) as std::ffi::c_uint)
                                    .wrapping_mul(ftotal),
                            ) as std::ffi::c_int as std::ffi::c_int;
                    } else {
                        r = (r as std::ffi::c_uint)
                            .wrapping_add(
                                (numrep as std::ffi::c_uint).wrapping_mul(ftotal),
                            ) as std::ffi::c_int as std::ffi::c_int;
                    }
                    if (*in_0).size == 0 as std::ffi::c_int as __u32 {
                        break;
                    }
                    item = crush_bucket_choose(
                        in_0,
                        *((*work).work)
                            .offset((-(1 as std::ffi::c_int) - (*in_0).id) as isize),
                        x,
                        r,
                        if !choose_args.is_null() {
                            &*choose_args
                                .offset((-(1 as std::ffi::c_int) - (*in_0).id) as isize)
                        } else {
                            0 as *const crush_choose_arg
                        },
                        outpos,
                    );
                    if item >= (*map).max_devices {
                        *out.offset(rep as isize) = 0x7fffffff as std::ffi::c_int;
                        if !out2.is_null() {
                            *out2.offset(rep as isize) = 0x7fffffff as std::ffi::c_int;
                        }
                        left_0 -= 1;
                        left_0;
                        break;
                    } else {
                        if item < 0 as std::ffi::c_int {
                            itemtype = (**((*map).buckets)
                                .offset((-(1 as std::ffi::c_int) - item) as isize))
                                .type_0 as std::ffi::c_int;
                        } else {
                            itemtype = 0 as std::ffi::c_int;
                        }
                        if itemtype != type_0 {
                            if item >= 0 as std::ffi::c_int
                                || -(1 as std::ffi::c_int) - item >= (*map).max_buckets
                            {
                                *out.offset(rep as isize) = 0x7fffffff as std::ffi::c_int;
                                if !out2.is_null() {
                                    *out2.offset(rep as isize) = 0x7fffffff as std::ffi::c_int;
                                }
                                left_0 -= 1;
                                left_0;
                                break;
                            } else {
                                in_0 = *((*map).buckets)
                                    .offset((-(1 as std::ffi::c_int) - item) as isize);
                            }
                        } else {
                            collide = 0 as std::ffi::c_int;
                            i = outpos;
                            while i < endpos {
                                if *out.offset(i as isize) == item {
                                    collide = 1 as std::ffi::c_int;
                                    break;
                                } else {
                                    i += 1;
                                    i;
                                }
                            }
                            if collide != 0 {
                                break;
                            }
                            if recurse_to_leaf != 0 {
                                if item < 0 as std::ffi::c_int {
                                    crush_choose_indep(
                                        map,
                                        work,
                                        *((*map).buckets)
                                            .offset((-(1 as std::ffi::c_int) - item) as isize),
                                        weight,
                                        weight_max,
                                        x,
                                        1 as std::ffi::c_int,
                                        numrep,
                                        0 as std::ffi::c_int,
                                        out2,
                                        rep,
                                        recurse_tries,
                                        0 as std::ffi::c_int as std::ffi::c_uint,
                                        0 as std::ffi::c_int,
                                        0 as *mut std::ffi::c_int,
                                        r,
                                        choose_args,
                                    );
                                    if *out2.offset(rep as isize)
                                        == 0x7fffffff as std::ffi::c_int
                                    {
                                        break;
                                    }
                                } else {
                                    *out2.offset(rep as isize) = item;
                                }
                            }
                            if itemtype == 0 as std::ffi::c_int
                                && is_out(map, weight, weight_max, item, x) != 0
                            {
                                break;
                            }
                            *out.offset(rep as isize) = item;
                            left_0 -= 1;
                            left_0;
                            break;
                        }
                    }
                }
            }
            rep += 1;
            rep;
        }
        ftotal = ftotal.wrapping_add(1);
        ftotal;
    }
    rep = outpos;
    while rep < endpos {
        if *out.offset(rep as isize) == 0x7ffffffe as std::ffi::c_int {
            *out.offset(rep as isize) = 0x7fffffff as std::ffi::c_int;
        }
        if !out2.is_null() && *out2.offset(rep as isize) == 0x7ffffffe as std::ffi::c_int
        {
            *out2.offset(rep as isize) = 0x7fffffff as std::ffi::c_int;
        }
        rep += 1;
        rep;
    }
    if !((*map).choose_tries).is_null() && ftotal <= (*map).choose_total_tries {
        let ref mut fresh1 = *((*map).choose_tries).offset(ftotal as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        *fresh1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_init_workspace(
    mut m: *const crush_map,
    mut v: *mut std::ffi::c_void,
) {
    let mut w: *mut crush_work = v as *mut crush_work;
    let mut point: *mut std::ffi::c_char = v as *mut std::ffi::c_char;
    let mut b: __s32 = 0;
    point = point
        .offset(::core::mem::size_of::<crush_work>() as std::ffi::c_ulong as isize);
    (*w).work = point as *mut *mut crush_work_bucket;
    point = point
        .offset(
            ((*m).max_buckets as std::ffi::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut crush_work_bucket>() as std::ffi::c_ulong,
                ) as isize,
        );
    b = 0 as std::ffi::c_int;
    while b < (*m).max_buckets {
        if !(*((*m).buckets).offset(b as isize)).is_null() {
            let ref mut fresh2 = *((*w).work).offset(b as isize);
            *fresh2 = point as *mut crush_work_bucket;
            match (**((*m).buckets).offset(b as isize)).alg as std::ffi::c_int {
                _ => {}
            }
            point = point
                .offset(
                    ::core::mem::size_of::<crush_work_bucket>() as std::ffi::c_ulong
                        as isize,
                );
            (**((*w).work).offset(b as isize)).perm_x = 0 as std::ffi::c_int as __u32;
            (**((*w).work).offset(b as isize)).perm_n = 0 as std::ffi::c_int as __u32;
            let ref mut fresh3 = (**((*w).work).offset(b as isize)).perm;
            *fresh3 = point as *mut __u32;
            point = point
                .offset(
                    ((**((*m).buckets).offset(b as isize)).size as std::ffi::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<__u32>() as std::ffi::c_ulong,
                        ) as isize,
                );
        }
        b += 1;
        b;
    }
    if !(point.offset_from(w as *mut std::ffi::c_char) as std::ffi::c_long as size_t
        != (*m).working_size)
    {} else {
        __assert_fail(
            b"!((char *)point - (char *)w != m->working_size)\0" as *const u8
                as *const std::ffi::c_char,
            b"/home/sevki/src/libcrush/crush/mapper.c\0" as *const u8
                as *const std::ffi::c_char,
            870 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[std::ffi::c_char; 60],
            >(b"void crush_init_workspace(const struct crush_map *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_6671: {
        if !(point.offset_from(w as *mut std::ffi::c_char) as std::ffi::c_long as size_t
            != (*m).working_size)
        {} else {
            __assert_fail(
                b"!((char *)point - (char *)w != m->working_size)\0" as *const u8
                    as *const std::ffi::c_char,
                b"/home/sevki/src/libcrush/crush/mapper.c\0" as *const u8
                    as *const std::ffi::c_char,
                870 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[std::ffi::c_char; 60],
                >(b"void crush_init_workspace(const struct crush_map *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn crush_do_rule(
    mut map: *const crush_map,
    mut ruleno: std::ffi::c_int,
    mut x: std::ffi::c_int,
    mut result: *mut std::ffi::c_int,
    mut result_max: std::ffi::c_int,
    mut weight: *const __u32,
    mut weight_max: std::ffi::c_int,
    mut cwin: *mut std::ffi::c_void,
    mut choose_args: *const crush_choose_arg,
) -> std::ffi::c_int {
    let mut result_len: std::ffi::c_int = 0;
    let mut cw: *mut crush_work = cwin as *mut crush_work;
    let mut a: *mut std::ffi::c_int = (cw as *mut std::ffi::c_char)
        .offset((*map).working_size as isize) as *mut std::ffi::c_int;
    let mut b: *mut std::ffi::c_int = a.offset(result_max as isize);
    let mut c: *mut std::ffi::c_int = b.offset(result_max as isize);
    let mut w: *mut std::ffi::c_int = a;
    let mut o: *mut std::ffi::c_int = b;
    let mut recurse_to_leaf: std::ffi::c_int = 0;
    let mut wsize: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut osize: std::ffi::c_int = 0;
    let mut tmp: *mut std::ffi::c_int = 0 as *mut std::ffi::c_int;
    let mut rule: *const crush_rule = 0 as *const crush_rule;
    let mut step: __u32 = 0;
    let mut i: std::ffi::c_int = 0;
    let mut j: std::ffi::c_int = 0;
    let mut numrep: std::ffi::c_int = 0;
    let mut out_size: std::ffi::c_int = 0;
    let mut choose_tries: std::ffi::c_int = ((*map).choose_total_tries)
        .wrapping_add(1 as std::ffi::c_int as __u32) as std::ffi::c_int;
    let mut choose_leaf_tries: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut choose_local_retries: std::ffi::c_int = (*map).choose_local_tries
        as std::ffi::c_int;
    let mut choose_local_fallback_retries: std::ffi::c_int = (*map)
        .choose_local_fallback_tries as std::ffi::c_int;
    let mut vary_r: std::ffi::c_int = (*map).chooseleaf_vary_r as std::ffi::c_int;
    let mut stable: std::ffi::c_int = (*map).chooseleaf_stable as std::ffi::c_int;
    if ruleno as __u32 >= (*map).max_rules {
        return 0 as std::ffi::c_int;
    }
    rule = *((*map).rules).offset(ruleno as isize);
    result_len = 0 as std::ffi::c_int;
    step = 0 as std::ffi::c_int as __u32;
    while step < (*rule).len {
        let mut firstn: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut curstep: *const crush_rule_step = &*((*rule).steps)
            .as_ptr()
            .offset(step as isize) as *const crush_rule_step;
        let mut current_block_59: u64;
        match (*curstep).op {
            1 => {
                if (*curstep).arg1 >= 0 as std::ffi::c_int
                    && (*curstep).arg1 < (*map).max_devices
                    || -(1 as std::ffi::c_int) - (*curstep).arg1 >= 0 as std::ffi::c_int
                        && -(1 as std::ffi::c_int) - (*curstep).arg1 < (*map).max_buckets
                        && !(*((*map).buckets)
                            .offset(
                                (-(1 as std::ffi::c_int) - (*curstep).arg1) as isize,
                            ))
                            .is_null()
                {
                    *w.offset(0 as std::ffi::c_int as isize) = (*curstep).arg1;
                    wsize = 1 as std::ffi::c_int;
                }
                current_block_59 = 15462640364611497761;
            }
            8 => {
                if (*curstep).arg1 > 0 as std::ffi::c_int {
                    choose_tries = (*curstep).arg1;
                }
                current_block_59 = 15462640364611497761;
            }
            9 => {
                if (*curstep).arg1 > 0 as std::ffi::c_int {
                    choose_leaf_tries = (*curstep).arg1;
                }
                current_block_59 = 15462640364611497761;
            }
            10 => {
                if (*curstep).arg1 >= 0 as std::ffi::c_int {
                    choose_local_retries = (*curstep).arg1;
                }
                current_block_59 = 15462640364611497761;
            }
            11 => {
                if (*curstep).arg1 >= 0 as std::ffi::c_int {
                    choose_local_fallback_retries = (*curstep).arg1;
                }
                current_block_59 = 15462640364611497761;
            }
            12 => {
                if (*curstep).arg1 >= 0 as std::ffi::c_int {
                    vary_r = (*curstep).arg1;
                }
                current_block_59 = 15462640364611497761;
            }
            13 => {
                if (*curstep).arg1 >= 0 as std::ffi::c_int {
                    stable = (*curstep).arg1;
                }
                current_block_59 = 15462640364611497761;
            }
            6 | 2 => {
                firstn = 1 as std::ffi::c_int;
                current_block_59 = 17267622262379957581;
            }
            7 | 3 => {
                current_block_59 = 17267622262379957581;
            }
            4 => {
                i = 0 as std::ffi::c_int;
                while i < wsize && result_len < result_max {
                    *result.offset(result_len as isize) = *w.offset(i as isize);
                    result_len += 1;
                    result_len;
                    i += 1;
                    i;
                }
                wsize = 0 as std::ffi::c_int;
                current_block_59 = 15462640364611497761;
            }
            _ => {
                current_block_59 = 15462640364611497761;
            }
        }
        match current_block_59 {
            17267622262379957581 => {
                if !(wsize == 0 as std::ffi::c_int) {
                    recurse_to_leaf = ((*curstep).op
                        == CRUSH_RULE_CHOOSELEAF_FIRSTN as std::ffi::c_int as __u32
                        || (*curstep).op
                            == CRUSH_RULE_CHOOSELEAF_INDEP as std::ffi::c_int as __u32)
                        as std::ffi::c_int;
                    osize = 0 as std::ffi::c_int;
                    let mut current_block_45: u64;
                    i = 0 as std::ffi::c_int;
                    while i < wsize {
                        let mut bno: std::ffi::c_int = 0;
                        numrep = (*curstep).arg1;
                        if numrep <= 0 as std::ffi::c_int {
                            numrep += result_max;
                            if numrep <= 0 as std::ffi::c_int {
                                current_block_45 = 3934796541983872331;
                            } else {
                                current_block_45 = 6717214610478484138;
                            }
                        } else {
                            current_block_45 = 6717214610478484138;
                        }
                        match current_block_45 {
                            6717214610478484138 => {
                                j = 0 as std::ffi::c_int;
                                bno = -(1 as std::ffi::c_int) - *w.offset(i as isize);
                                if !(bno < 0 as std::ffi::c_int
                                    || bno >= (*map).max_buckets)
                                {
                                    if firstn != 0 {
                                        let mut recurse_tries: std::ffi::c_int = 0;
                                        if choose_leaf_tries != 0 {
                                            recurse_tries = choose_leaf_tries;
                                        } else if (*map).chooseleaf_descend_once != 0 {
                                            recurse_tries = 1 as std::ffi::c_int;
                                        } else {
                                            recurse_tries = choose_tries;
                                        }
                                        osize
                                            += crush_choose_firstn(
                                                map,
                                                cw,
                                                *((*map).buckets).offset(bno as isize),
                                                weight,
                                                weight_max,
                                                x,
                                                numrep,
                                                (*curstep).arg2,
                                                o.offset(osize as isize),
                                                j,
                                                result_max - osize,
                                                choose_tries as std::ffi::c_uint,
                                                recurse_tries as std::ffi::c_uint,
                                                choose_local_retries as std::ffi::c_uint,
                                                choose_local_fallback_retries as std::ffi::c_uint,
                                                recurse_to_leaf,
                                                vary_r as std::ffi::c_uint,
                                                stable as std::ffi::c_uint,
                                                c.offset(osize as isize),
                                                0 as std::ffi::c_int,
                                                choose_args,
                                            );
                                    } else {
                                        out_size = if numrep < result_max - osize {
                                            numrep
                                        } else {
                                            result_max - osize
                                        };
                                        crush_choose_indep(
                                            map,
                                            cw,
                                            *((*map).buckets).offset(bno as isize),
                                            weight,
                                            weight_max,
                                            x,
                                            out_size,
                                            numrep,
                                            (*curstep).arg2,
                                            o.offset(osize as isize),
                                            j,
                                            choose_tries as std::ffi::c_uint,
                                            (if choose_leaf_tries != 0 {
                                                choose_leaf_tries
                                            } else {
                                                1 as std::ffi::c_int
                                            }) as std::ffi::c_uint,
                                            recurse_to_leaf,
                                            c.offset(osize as isize),
                                            0 as std::ffi::c_int,
                                            choose_args,
                                        );
                                        osize += out_size;
                                    }
                                }
                            }
                            _ => {}
                        }
                        i += 1;
                        i;
                    }
                    if recurse_to_leaf != 0 {
                        memcpy(
                            o as *mut std::ffi::c_void,
                            c as *const std::ffi::c_void,
                            (osize as std::ffi::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<std::ffi::c_int>()
                                        as std::ffi::c_ulong,
                                ),
                        );
                    }
                    tmp = o;
                    o = w;
                    w = tmp;
                    wsize = osize;
                }
            }
            _ => {}
        }
        step = step.wrapping_add(1);
        step;
    }
    return result_len;
}
