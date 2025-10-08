#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::crush::types::*;
use crate::crush::types::ffi;
use crate::crush::hash::{crush_hash32_2, crush_hash32_3, crush_hash32_4};

unsafe extern "C" {
    fn memcpy(_: *mut ffi::c_void, _: *const ffi::c_void, _: ffi::c_ulong) -> *mut ffi::c_void;
}
static mut __RH_LH_tbl: [S64; 258] = [
    0x1000000000000 as ffi::c_longlong,
    0 as ffi::c_longlong,
    0xfe03f80fe040 as ffi::c_longlong,
    0x2dfca16dde1 as ffi::c_longlong,
    0xfc0fc0fc0fc1 as ffi::c_longlong,
    0x5b9e5a170b4 as ffi::c_longlong,
    0xfa232cf25214 as ffi::c_longlong,
    0x88e68ea899a as ffi::c_longlong,
    0xf83e0f83e0f9 as ffi::c_longlong,
    0xb5d69bac77e as ffi::c_longlong,
    0xf6603d980f67 as ffi::c_longlong,
    0xe26fd5c8555 as ffi::c_longlong,
    0xf4898d5f85bc as ffi::c_longlong,
    0x10eb389fa29f as ffi::c_longlong,
    0xf2b9d6480f2c as ffi::c_longlong,
    0x13aa2fdd27f1 as ffi::c_longlong,
    0xf0f0f0f0f0f1 as ffi::c_longlong,
    0x1663f6fac913 as ffi::c_longlong,
    0xef2eb71fc435 as ffi::c_longlong,
    0x1918a16e4633 as ffi::c_longlong,
    0xed7303b5cc0f as ffi::c_longlong,
    0x1bc84240adab as ffi::c_longlong,
    0xebbdb2a5c162 as ffi::c_longlong,
    0x1e72ec117fa5 as ffi::c_longlong,
    0xea0ea0ea0ea1 as ffi::c_longlong,
    0x2118b119b4f3 as ffi::c_longlong,
    0xe865ac7b7604 as ffi::c_longlong,
    0x23b9a32eaa56 as ffi::c_longlong,
    0xe6c2b4481cd9 as ffi::c_longlong,
    0x2655d3c4f15c as ffi::c_longlong,
    0xe525982af70d as ffi::c_longlong,
    0x28ed53f307ee as ffi::c_longlong,
    0xe38e38e38e39 as ffi::c_longlong,
    0x2b803473f7ad as ffi::c_longlong,
    0xe1fc780e1fc8 as ffi::c_longlong,
    0x2e0e85a9de04 as ffi::c_longlong,
    0xe070381c0e08 as ffi::c_longlong,
    0x309857a05e07 as ffi::c_longlong,
    0xdee95c4ca038 as ffi::c_longlong,
    0x331dba0efce1 as ffi::c_longlong,
    0xdd67c8a60dd7 as ffi::c_longlong,
    0x359ebc5b69d9 as ffi::c_longlong,
    0xdbeb61eed19d as ffi::c_longlong,
    0x381b6d9bb29b as ffi::c_longlong,
    0xda740da740db as ffi::c_longlong,
    0x3a93dc9864b2 as ffi::c_longlong,
    0xd901b2036407 as ffi::c_longlong,
    0x3d0817ce9cd4 as ffi::c_longlong,
    0xd79435e50d7a as ffi::c_longlong,
    0x3f782d7204d0 as ffi::c_longlong,
    0xd62b80d62b81 as ffi::c_longlong,
    0x41e42b6ec0c0 as ffi::c_longlong,
    0xd4c77b03531e as ffi::c_longlong,
    0x444c1f6b4c2d as ffi::c_longlong,
    0xd3680d3680d4 as ffi::c_longlong,
    0x46b016ca47c1 as ffi::c_longlong,
    0xd20d20d20d21 as ffi::c_longlong,
    0x49101eac381c as ffi::c_longlong,
    0xd0b69fcbd259 as ffi::c_longlong,
    0x4b6c43f1366a as ffi::c_longlong,
    0xcf6474a8819f as ffi::c_longlong,
    0x4dc4933a9337 as ffi::c_longlong,
    0xce168a772509 as ffi::c_longlong,
    0x501918ec6c11 as ffi::c_longlong,
    0xcccccccccccd as ffi::c_longlong,
    0x5269e12f346e as ffi::c_longlong,
    0xcb8727c065c4 as ffi::c_longlong,
    0x54b6f7f1325a as ffi::c_longlong,
    0xca4587e6b750 as ffi::c_longlong,
    0x570068e7ef5a as ffi::c_longlong,
    0xc907da4e8712 as ffi::c_longlong,
    0x59463f919dee as ffi::c_longlong,
    0xc7ce0c7ce0c8 as ffi::c_longlong,
    0x5b8887367433 as ffi::c_longlong,
    0xc6980c6980c7 as ffi::c_longlong,
    0x5dc74ae9fbec as ffi::c_longlong,
    0xc565c87b5f9e as ffi::c_longlong,
    0x6002958c5871 as ffi::c_longlong,
    0xc4372f855d83 as ffi::c_longlong,
    0x623a71cb82c8 as ffi::c_longlong,
    0xc30c30c30c31 as ffi::c_longlong,
    0x646eea247c5c as ffi::c_longlong,
    0xc1e4bbd595f7 as ffi::c_longlong,
    0x66a008e4788c as ffi::c_longlong,
    0xc0c0c0c0c0c1 as ffi::c_longlong,
    0x68cdd829fd81 as ffi::c_longlong,
    0xbfa02fe80bfb as ffi::c_longlong,
    0x6af861e5fc7d as ffi::c_longlong,
    0xbe82fa0be830 as ffi::c_longlong,
    0x6d1fafdce20a as ffi::c_longlong,
    0xbd6910470767 as ffi::c_longlong,
    0x6f43cba79e40 as ffi::c_longlong,
    0xbc52640bc527 as ffi::c_longlong,
    0x7164beb4a56d as ffi::c_longlong,
    0xbb3ee721a54e as ffi::c_longlong,
    0x73829248e961 as ffi::c_longlong,
    0xba2e8ba2e8bb as ffi::c_longlong,
    0x759d4f80cba8 as ffi::c_longlong,
    0xb92143fa36f6 as ffi::c_longlong,
    0x77b4ff5108d9 as ffi::c_longlong,
    0xb81702e05c0c as ffi::c_longlong,
    0x79c9aa879d53 as ffi::c_longlong,
    0xb70fbb5a19bf as ffi::c_longlong,
    0x7bdb59cca388 as ffi::c_longlong,
    0xb60b60b60b61 as ffi::c_longlong,
    0x7dea15a32c1b as ffi::c_longlong,
    0xb509e68a9b95 as ffi::c_longlong,
    0x7ff5e66a0ffe as ffi::c_longlong,
    0xb40b40b40b41 as ffi::c_longlong,
    0x81fed45cbccb as ffi::c_longlong,
    0xb30f63528918 as ffi::c_longlong,
    0x8404e793fb81 as ffi::c_longlong,
    0xb21642c8590c as ffi::c_longlong,
    0x86082806b1d5 as ffi::c_longlong,
    0xb11fd3b80b12 as ffi::c_longlong,
    0x88089d8a9e47 as ffi::c_longlong,
    0xb02c0b02c0b1 as ffi::c_longlong,
    0x8a064fd50f2a as ffi::c_longlong,
    0xaf3addc680b0 as ffi::c_longlong,
    0x8c01467b94bb as ffi::c_longlong,
    0xae4c415c9883 as ffi::c_longlong,
    0x8df988f4ae80 as ffi::c_longlong,
    0xad602b580ad7 as ffi::c_longlong,
    0x8fef1e987409 as ffi::c_longlong,
    0xac7691840ac8 as ffi::c_longlong,
    0x91e20ea1393e as ffi::c_longlong,
    0xab8f69e2835a as ffi::c_longlong,
    0x93d2602c2e5f as ffi::c_longlong,
    0xaaaaaaaaaaab as ffi::c_longlong,
    0x95c01a39fbd6 as ffi::c_longlong,
    0xa9c84a47a080 as ffi::c_longlong,
    0x97ab43af59f9 as ffi::c_longlong,
    0xa8e83f5717c1 as ffi::c_longlong,
    0x9993e355a4e5 as ffi::c_longlong,
    0xa80a80a80a81 as ffi::c_longlong,
    0x9b79ffdb6c8b as ffi::c_longlong,
    0xa72f0539782a as ffi::c_longlong,
    0x9d5d9fd5010b as ffi::c_longlong,
    0xa655c4392d7c as ffi::c_longlong,
    0x9f3ec9bcfb80 as ffi::c_longlong,
    0xa57eb50295fb as ffi::c_longlong,
    0xa11d83f4c355 as ffi::c_longlong,
    0xa4a9cf1d9684 as ffi::c_longlong,
    0xa2f9d4c51039 as ffi::c_longlong,
    0xa3d70a3d70a4 as ffi::c_longlong,
    0xa4d3c25e68dc as ffi::c_longlong,
    0xa3065e3fae7d as ffi::c_longlong,
    0xa6ab52d99e76 as ffi::c_longlong,
    0xa237c32b16d0 as ffi::c_longlong,
    0xa8808c384547 as ffi::c_longlong,
    0xa16b312ea8fd as ffi::c_longlong,
    0xaa5374652a1c as ffi::c_longlong,
    0xa0a0a0a0a0a1 as ffi::c_longlong,
    0xac241134c4e9 as ffi::c_longlong,
    0x9fd809fd80a0 as ffi::c_longlong,
    0xadf26865a8a1 as ffi::c_longlong,
    0x9f1165e72549 as ffi::c_longlong,
    0xafbe7fa0f04d as ffi::c_longlong,
    0x9e4cad23dd60 as ffi::c_longlong,
    0xb1885c7aa982 as ffi::c_longlong,
    0x9d89d89d89d9 as ffi::c_longlong,
    0xb35004723c46 as ffi::c_longlong,
    0x9cc8e160c3fc as ffi::c_longlong,
    0xb5157cf2d078 as ffi::c_longlong,
    0x9c09c09c09c1 as ffi::c_longlong,
    0xb6d8cb53b0ca as ffi::c_longlong,
    0x9b4c6f9ef03b as ffi::c_longlong,
    0xb899f4d8ab63 as ffi::c_longlong,
    0x9a90e7d95bc7 as ffi::c_longlong,
    0xba58feb2703a as ffi::c_longlong,
    0x99d722dabde6 as ffi::c_longlong,
    0xbc15edfeed32 as ffi::c_longlong,
    0x991f1a515886 as ffi::c_longlong,
    0xbdd0c7c9a817 as ffi::c_longlong,
    0x9868c809868d as ffi::c_longlong,
    0xbf89910c1678 as ffi::c_longlong,
    0x97b425ed097c as ffi::c_longlong,
    0xc1404eadf383 as ffi::c_longlong,
    0x97012e025c05 as ffi::c_longlong,
    0xc2f5058593d9 as ffi::c_longlong,
    0x964fda6c0965 as ffi::c_longlong,
    0xc4a7ba58377c as ffi::c_longlong,
    0x95a02568095b as ffi::c_longlong,
    0xc65871da59dd as ffi::c_longlong,
    0x94f2094f2095 as ffi::c_longlong,
    0xc80730b00016 as ffi::c_longlong,
    0x944580944581 as ffi::c_longlong,
    0xc9b3fb6d0559 as ffi::c_longlong,
    0x939a85c4093a as ffi::c_longlong,
    0xcb5ed69565af as ffi::c_longlong,
    0x92f113840498 as ffi::c_longlong,
    0xcd07c69d8702 as ffi::c_longlong,
    0x924924924925 as ffi::c_longlong,
    0xceaecfea8085 as ffi::c_longlong,
    0x91a2b3c4d5e7 as ffi::c_longlong,
    0xd053f6d26089 as ffi::c_longlong,
    0x90fdbc090fdc as ffi::c_longlong,
    0xd1f73f9c70c0 as ffi::c_longlong,
    0x905a38633e07 as ffi::c_longlong,
    0xd398ae817906 as ffi::c_longlong,
    0x8fb823ee08fc as ffi::c_longlong,
    0xd53847ac00a6 as ffi::c_longlong,
    0x8f1779d9fdc4 as ffi::c_longlong,
    0xd6d60f388e41 as ffi::c_longlong,
    0x8e78356d1409 as ffi::c_longlong,
    0xd8720935e643 as ffi::c_longlong,
    0x8dda5202376a as ffi::c_longlong,
    0xda0c39a54804 as ffi::c_longlong,
    0x8d3dcb08d3dd as ffi::c_longlong,
    0xdba4a47aa996 as ffi::c_longlong,
    0x8ca29c046515 as ffi::c_longlong,
    0xdd3b4d9cf24b as ffi::c_longlong,
    0x8c08c08c08c1 as ffi::c_longlong,
    0xded038e633f3 as ffi::c_longlong,
    0x8b70344a139c as ffi::c_longlong,
    0xe0636a23e2ee as ffi::c_longlong,
    0x8ad8f2fba939 as ffi::c_longlong,
    0xe1f4e5170d02 as ffi::c_longlong,
    0x8a42f870566a as ffi::c_longlong,
    0xe384ad748f0e as ffi::c_longlong,
    0x89ae4089ae41 as ffi::c_longlong,
    0xe512c6e54998 as ffi::c_longlong,
    0x891ac73ae982 as ffi::c_longlong,
    0xe69f35065448 as ffi::c_longlong,
    0x888888888889 as ffi::c_longlong,
    0xe829fb693044 as ffi::c_longlong,
    0x87f78087f781 as ffi::c_longlong,
    0xe9b31d93f98e as ffi::c_longlong,
    0x8767ab5f34e5 as ffi::c_longlong,
    0xeb3a9f019750 as ffi::c_longlong,
    0x86d905447a35 as ffi::c_longlong,
    0xecc08321eb30 as ffi::c_longlong,
    0x864b8a7de6d2 as ffi::c_longlong,
    0xee44cd59ffab as ffi::c_longlong,
    0x85bf37612cef as ffi::c_longlong,
    0xefc781043579 as ffi::c_longlong,
    0x853408534086 as ffi::c_longlong,
    0xf148a170700a as ffi::c_longlong,
    0x84a9f9c8084b as ffi::c_longlong,
    0xf2c831e44116 as ffi::c_longlong,
    0x842108421085 as ffi::c_longlong,
    0xf446359b1353 as ffi::c_longlong,
    0x839930523fbf as ffi::c_longlong,
    0xf5c2afc65447 as ffi::c_longlong,
    0x83126e978d50 as ffi::c_longlong,
    0xf73da38d9d4a as ffi::c_longlong,
    0x828cbfbeb9a1 as ffi::c_longlong,
    0xf8b7140edbb1 as ffi::c_longlong,
    0x820820820821 as ffi::c_longlong,
    0xfa2f045e7832 as ffi::c_longlong,
    0x81848da8faf1 as ffi::c_longlong,
    0xfba577877d7d as ffi::c_longlong,
    0x810204081021 as ffi::c_longlong,
    0xfd1a708bbe11 as ffi::c_longlong,
    0x808080808081 as ffi::c_longlong,
    0xfe8df263f957 as ffi::c_longlong,
    0x800000000000 as ffi::c_longlong,
    0xffff00000000 as ffi::c_longlong,
];
static mut __LL_tbl: [S64; 256] = [
    0 as ffi::c_ulonglong as S64,
    0x2e2a60a00 as ffi::c_ulonglong as S64,
    0x70cb64ec5 as ffi::c_ulonglong as S64,
    0x9ef50ce67 as ffi::c_ulonglong as S64,
    0xcd1e588fd as ffi::c_ulonglong as S64,
    0xfb4747e9c as ffi::c_ulonglong as S64,
    0x1296fdaf5e as ffi::c_ulonglong as S64,
    0x1579811b58 as ffi::c_ulonglong as S64,
    0x185bfec2a1 as ffi::c_ulonglong as S64,
    0x1b3e76a552 as ffi::c_ulonglong as S64,
    0x1e20e8c380 as ffi::c_ulonglong as S64,
    0x2103551d43 as ffi::c_ulonglong as S64,
    0x23e5bbb2b2 as ffi::c_ulonglong as S64,
    0x26c81c83e4 as ffi::c_ulonglong as S64,
    0x29aa7790f0 as ffi::c_ulonglong as S64,
    0x2c8cccd9ed as ffi::c_ulonglong as S64,
    0x2f6f1c5ef2 as ffi::c_ulonglong as S64,
    0x3251662017 as ffi::c_ulonglong as S64,
    0x3533aa1d71 as ffi::c_ulonglong as S64,
    0x3815e8571a as ffi::c_ulonglong as S64,
    0x3af820cd26 as ffi::c_ulonglong as S64,
    0x3dda537fae as ffi::c_ulonglong as S64,
    0x40bc806ec8 as ffi::c_ulonglong as S64,
    0x439ea79a8c as ffi::c_ulonglong as S64,
    0x4680c90310 as ffi::c_ulonglong as S64,
    0x4962e4a86c as ffi::c_ulonglong as S64,
    0x4c44fa8ab6 as ffi::c_ulonglong as S64,
    0x4f270aaa06 as ffi::c_ulonglong as S64,
    0x5209150672 as ffi::c_ulonglong as S64,
    0x54eb19a013 as ffi::c_ulonglong as S64,
    0x57cd1876fd as ffi::c_ulonglong as S64,
    0x5aaf118b4a as ffi::c_ulonglong as S64,
    0x5d9104dd0f as ffi::c_ulonglong as S64,
    0x6072f26c64 as ffi::c_ulonglong as S64,
    0x6354da3960 as ffi::c_ulonglong as S64,
    0x6636bc441a as ffi::c_ulonglong as S64,
    0x6918988ca8 as ffi::c_ulonglong as S64,
    0x6bfa6f1322 as ffi::c_ulonglong as S64,
    0x6edc3fd79f as ffi::c_ulonglong as S64,
    0x71be0ada35 as ffi::c_ulonglong as S64,
    0x749fd01afd as ffi::c_ulonglong as S64,
    0x77818f9a0c as ffi::c_ulonglong as S64,
    0x7a6349577a as ffi::c_ulonglong as S64,
    0x7d44fd535e as ffi::c_ulonglong as S64,
    0x8026ab8dce as ffi::c_ulonglong as S64,
    0x83085406e3 as ffi::c_ulonglong as S64,
    0x85e9f6beb2 as ffi::c_ulonglong as S64,
    0x88cb93b552 as ffi::c_ulonglong as S64,
    0x8bad2aeadc as ffi::c_ulonglong as S64,
    0x8e8ebc5f65 as ffi::c_ulonglong as S64,
    0x9170481305 as ffi::c_ulonglong as S64,
    0x9451ce05d3 as ffi::c_ulonglong as S64,
    0x97334e37e5 as ffi::c_ulonglong as S64,
    0x9a14c8a953 as ffi::c_ulonglong as S64,
    0x9cf63d5a33 as ffi::c_ulonglong as S64,
    0x9fd7ac4a9d as ffi::c_ulonglong as S64,
    0xa2b07f3458 as ffi::c_ulonglong as S64,
    0xa59a78ea6a as ffi::c_ulonglong as S64,
    0xa87bd699fb as ffi::c_ulonglong as S64,
    0xab5d2e8970 as ffi::c_ulonglong as S64,
    0xae3e80b8e3 as ffi::c_ulonglong as S64,
    0xb11fcd2869 as ffi::c_ulonglong as S64,
    0xb40113d818 as ffi::c_ulonglong as S64,
    0xb6e254c80a as ffi::c_ulonglong as S64,
    0xb9c38ff853 as ffi::c_ulonglong as S64,
    0xbca4c5690c as ffi::c_ulonglong as S64,
    0xbf85f51a4a as ffi::c_ulonglong as S64,
    0xc2671f0c26 as ffi::c_ulonglong as S64,
    0xc548433eb6 as ffi::c_ulonglong as S64,
    0xc82961b211 as ffi::c_ulonglong as S64,
    0xcb0a7a664d as ffi::c_ulonglong as S64,
    0xcdeb8d5b82 as ffi::c_ulonglong as S64,
    0xd0cc9a91c8 as ffi::c_ulonglong as S64,
    0xd3ada20933 as ffi::c_ulonglong as S64,
    0xd68ea3c1dd as ffi::c_ulonglong as S64,
    0xd96f9fbbdb as ffi::c_ulonglong as S64,
    0xdc5095f744 as ffi::c_ulonglong as S64,
    0xdf31867430 as ffi::c_ulonglong as S64,
    0xe2127132b5 as ffi::c_ulonglong as S64,
    0xe4f35632ea as ffi::c_ulonglong as S64,
    0xe7d43574e6 as ffi::c_ulonglong as S64,
    0xeab50ef8c1 as ffi::c_ulonglong as S64,
    0xed95e2be90 as ffi::c_ulonglong as S64,
    0xf076b0c66c as ffi::c_ulonglong as S64,
    0xf35779106a as ffi::c_ulonglong as S64,
    0xf6383b9ca2 as ffi::c_ulonglong as S64,
    0xf918f86b2a as ffi::c_ulonglong as S64,
    0xfbf9af7c1a as ffi::c_ulonglong as S64,
    0xfeda60cf88 as ffi::c_ulonglong as S64,
    0x101bb0c658c as ffi::c_ulonglong as S64,
    0x1049bb23e3c as ffi::c_ulonglong as S64,
    0x1077c5259af as ffi::c_ulonglong as S64,
    0x10a5cecb7fc as ffi::c_ulonglong as S64,
    0x10d3d81593a as ffi::c_ulonglong as S64,
    0x1101e103d7f as ffi::c_ulonglong as S64,
    0x112fe9964e4 as ffi::c_ulonglong as S64,
    0x115df1ccf7e as ffi::c_ulonglong as S64,
    0x118bf9a7d64 as ffi::c_ulonglong as S64,
    0x11ba0126ead as ffi::c_ulonglong as S64,
    0x11e8084a371 as ffi::c_ulonglong as S64,
    0x12160f11bc6 as ffi::c_ulonglong as S64,
    0x1244157d7c3 as ffi::c_ulonglong as S64,
    0x12721b8d77f as ffi::c_ulonglong as S64,
    0x12a02141b10 as ffi::c_ulonglong as S64,
    0x12ce269a28e as ffi::c_ulonglong as S64,
    0x12fc2b96e0f as ffi::c_ulonglong as S64,
    0x132a3037daa as ffi::c_ulonglong as S64,
    0x1358347d177 as ffi::c_ulonglong as S64,
    0x1386386698c as ffi::c_ulonglong as S64,
    0x13b43bf45ff as ffi::c_ulonglong as S64,
    0x13e23f266e9 as ffi::c_ulonglong as S64,
    0x141041fcc5e as ffi::c_ulonglong as S64,
    0x143e4477678 as ffi::c_ulonglong as S64,
    0x146c469654b as ffi::c_ulonglong as S64,
    0x149a48598f0 as ffi::c_ulonglong as S64,
    0x14c849c117c as ffi::c_ulonglong as S64,
    0x14f64accf08 as ffi::c_ulonglong as S64,
    0x15244b7d1a9 as ffi::c_ulonglong as S64,
    0x15524bd1976 as ffi::c_ulonglong as S64,
    0x15804bca687 as ffi::c_ulonglong as S64,
    0x15ae4b678f2 as ffi::c_ulonglong as S64,
    0x15dc4aa90ce as ffi::c_ulonglong as S64,
    0x160a498ee31 as ffi::c_ulonglong as S64,
    0x16384819134 as ffi::c_ulonglong as S64,
    0x166646479ec as ffi::c_ulonglong as S64,
    0x1694441a870 as ffi::c_ulonglong as S64,
    0x16c24191cd7 as ffi::c_ulonglong as S64,
    0x16df6ca19bd as ffi::c_ulonglong as S64,
    0x171e3b6d7aa as ffi::c_ulonglong as S64,
    0x174c37d1e44 as ffi::c_ulonglong as S64,
    0x177a33dab1c as ffi::c_ulonglong as S64,
    0x17a82f87e49 as ffi::c_ulonglong as S64,
    0x17d62ad97e2 as ffi::c_ulonglong as S64,
    0x180425cf7fe as ffi::c_ulonglong as S64,
    0x182b07f3458 as ffi::c_ulonglong as S64,
    0x18601aa8c19 as ffi::c_ulonglong as S64,
    0x188e148c046 as ffi::c_ulonglong as S64,
    0x18bc0e13b52 as ffi::c_ulonglong as S64,
    0x18ea073fd52 as ffi::c_ulonglong as S64,
    0x1918001065d as ffi::c_ulonglong as S64,
    0x1945f88568b as ffi::c_ulonglong as S64,
    0x1973f09edf2 as ffi::c_ulonglong as S64,
    0x19a1e85ccaa as ffi::c_ulonglong as S64,
    0x19cfdfbf2c8 as ffi::c_ulonglong as S64,
    0x19fdd6c6063 as ffi::c_ulonglong as S64,
    0x1a2bcd71593 as ffi::c_ulonglong as S64,
    0x1a59c3c126e as ffi::c_ulonglong as S64,
    0x1a87b9b570b as ffi::c_ulonglong as S64,
    0x1ab5af4e380 as ffi::c_ulonglong as S64,
    0x1ae3a48b7e5 as ffi::c_ulonglong as S64,
    0x1b11996d450 as ffi::c_ulonglong as S64,
    0x1b3f8df38d9 as ffi::c_ulonglong as S64,
    0x1b6d821e595 as ffi::c_ulonglong as S64,
    0x1b9b75eda9b as ffi::c_ulonglong as S64,
    0x1bc96961803 as ffi::c_ulonglong as S64,
    0x1bf75c79de3 as ffi::c_ulonglong as S64,
    0x1c254f36c51 as ffi::c_ulonglong as S64,
    0x1c534198365 as ffi::c_ulonglong as S64,
    0x1c81339e336 as ffi::c_ulonglong as S64,
    0x1caf2548bd9 as ffi::c_ulonglong as S64,
    0x1cdd1697d67 as ffi::c_ulonglong as S64,
    0x1d0b078b7f5 as ffi::c_ulonglong as S64,
    0x1d38f823b9a as ffi::c_ulonglong as S64,
    0x1d66e86086d as ffi::c_ulonglong as S64,
    0x1d94d841e86 as ffi::c_ulonglong as S64,
    0x1dc2c7c7df9 as ffi::c_ulonglong as S64,
    0x1df0b6f26df as ffi::c_ulonglong as S64,
    0x1e1ea5c194e as ffi::c_ulonglong as S64,
    0x1e4c943555d as ffi::c_ulonglong as S64,
    0x1e7a824db23 as ffi::c_ulonglong as S64,
    0x1ea8700aab5 as ffi::c_ulonglong as S64,
    0x1ed65d6c42b as ffi::c_ulonglong as S64,
    0x1f044a7279d as ffi::c_ulonglong as S64,
    0x1f32371d51f as ffi::c_ulonglong as S64,
    0x1f60236ccca as ffi::c_ulonglong as S64,
    0x1f8e0f60eb3 as ffi::c_ulonglong as S64,
    0x1fbbfaf9af3 as ffi::c_ulonglong as S64,
    0x1fe9e63719e as ffi::c_ulonglong as S64,
    0x2017d1192cc as ffi::c_ulonglong as S64,
    0x2045bb9fe94 as ffi::c_ulonglong as S64,
    0x2073a5cb50d as ffi::c_ulonglong as S64,
    0x209c06e6212 as ffi::c_ulonglong as S64,
    0x20cf791026a as ffi::c_ulonglong as S64,
    0x20fd622997c as ffi::c_ulonglong as S64,
    0x212b07f3458 as ffi::c_ulonglong as S64,
    0x2159334a8d8 as ffi::c_ulonglong as S64,
    0x21871b52150 as ffi::c_ulonglong as S64,
    0x21b502fe517 as ffi::c_ulonglong as S64,
    0x21d6a73a78f as ffi::c_ulonglong as S64,
    0x2210d144eee as ffi::c_ulonglong as S64,
    0x223eb7df52c as ffi::c_ulonglong as S64,
    0x226c9e1e713 as ffi::c_ulonglong as S64,
    0x229a84024bb as ffi::c_ulonglong as S64,
    0x22c23679b4e as ffi::c_ulonglong as S64,
    0x22f64eb83a8 as ffi::c_ulonglong as S64,
    0x2324338a51b as ffi::c_ulonglong as S64,
    0x235218012a9 as ffi::c_ulonglong as S64,
    0x237ffc1cc69 as ffi::c_ulonglong as S64,
    0x23a2c3b0ea4 as ffi::c_ulonglong as S64,
    0x23d13ee805b as ffi::c_ulonglong as S64,
    0x24035e9221f as ffi::c_ulonglong as S64,
    0x243788faf25 as ffi::c_ulonglong as S64,
    0x24656b4e735 as ffi::c_ulonglong as S64,
    0x247ed646bfe as ffi::c_ulonglong as S64,
    0x24c12ee3d98 as ffi::c_ulonglong as S64,
    0x24ef1025c1a as ffi::c_ulonglong as S64,
    0x251cf10c799 as ffi::c_ulonglong as S64,
    0x25492644d65 as ffi::c_ulonglong as S64,
    0x2578b1c85ee as ffi::c_ulonglong as S64,
    0x25a6919d8f0 as ffi::c_ulonglong as S64,
    0x25d13ee805b as ffi::c_ulonglong as S64,
    0x26025036716 as ffi::c_ulonglong as S64,
    0x26296453882 as ffi::c_ulonglong as S64,
    0x265e0d62b53 as ffi::c_ulonglong as S64,
    0x268beb701f3 as ffi::c_ulonglong as S64,
    0x26b9c92265e as ffi::c_ulonglong as S64,
    0x26d32f798a9 as ffi::c_ulonglong as S64,
    0x271583758eb as ffi::c_ulonglong as S64,
    0x2743601673b as ffi::c_ulonglong as S64,
    0x27713c5c3b0 as ffi::c_ulonglong as S64,
    0x279f1846e5f as ffi::c_ulonglong as S64,
    0x27ccf3d6761 as ffi::c_ulonglong as S64,
    0x27e6580aecb as ffi::c_ulonglong as S64,
    0x2828a9e44b3 as ffi::c_ulonglong as S64,
    0x28568462932 as ffi::c_ulonglong as S64,
    0x287bdbf5255 as ffi::c_ulonglong as S64,
    0x28b2384de4a as ffi::c_ulonglong as S64,
    0x28d13ee805b as ffi::c_ulonglong as S64,
    0x29035e9221f as ffi::c_ulonglong as S64,
    0x29296453882 as ffi::c_ulonglong as S64,
    0x29699bdfb61 as ffi::c_ulonglong as S64,
    0x29902a37aab as ffi::c_ulonglong as S64,
    0x29c54b864c9 as ffi::c_ulonglong as S64,
    0x29deabd1083 as ffi::c_ulonglong as S64,
    0x2a20f9c0bb5 as ffi::c_ulonglong as S64,
    0x2a4c7605d61 as ffi::c_ulonglong as S64,
    0x2a7bdbf5255 as ffi::c_ulonglong as S64,
    0x2a96056dafc as ffi::c_ulonglong as S64,
    0x2ac3daf14ef as ffi::c_ulonglong as S64,
    0x2af1b019eca as ffi::c_ulonglong as S64,
    0x2b296453882 as ffi::c_ulonglong as S64,
    0x2b5d022d80f as ffi::c_ulonglong as S64,
    0x2b8fa471cb3 as ffi::c_ulonglong as S64,
    0x2ba9012e713 as ffi::c_ulonglong as S64,
    0x2bd6d4901cc as ffi::c_ulonglong as S64,
    0x2c04a796cf6 as ffi::c_ulonglong as S64,
    0x2c327a428a6 as ffi::c_ulonglong as S64,
    0x2c61a5e8f4c as ffi::c_ulonglong as S64,
    0x2c8e1e891f6 as ffi::c_ulonglong as S64,
    0x2cbbf023fc2 as ffi::c_ulonglong as S64,
    0x2ce9c163e6e as ffi::c_ulonglong as S64,
    0x2d179248e13 as ffi::c_ulonglong as S64,
    0x2d4562d2ec6 as ffi::c_ulonglong as S64,
    0x2d73330209d as ffi::c_ulonglong as S64,
    0x2da102d63b0 as ffi::c_ulonglong as S64,
    0x2dced24f814 as ffi::c_ulonglong as S64,
];
pub unsafe fn crush_find_rule(
    mut map: *const CrushMap,
    mut ruleset: ffi::c_int,
    mut type_0: ffi::c_int,
    mut size: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut i: U32 = 0;
        i = 0 as ffi::c_int as U32;
        while i < (*map).max_rules {
            if !(*((*map).rules).offset(i as isize)).is_null()
                && (**((*map).rules).offset(i as isize)).mask.ruleset as ffi::c_int == ruleset
                && (**((*map).rules).offset(i as isize)).mask.type_0 as ffi::c_int == type_0
                && (**((*map).rules).offset(i as isize)).mask.min_size as ffi::c_int <= size
                && (**((*map).rules).offset(i as isize)).mask.max_size as ffi::c_int >= size
            {
                return i as ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        -(1 as ffi::c_int)
    }
}
unsafe fn bucket_perm_choose(
    mut bucket: *const CrushBucket,
    mut work: *mut CrushWorkBucket,
    mut x: ffi::c_int,
    mut r: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut pr: ffi::c_uint = r as U32 % (*bucket).size;
        let mut i: ffi::c_uint = 0;
        let mut s: ffi::c_uint = 0;
        if (*work).perm_x != x as U32 || (*work).perm_n == 0 as ffi::c_int as U32 {
            (*work).perm_x = x as U32;
            if pr == 0 as ffi::c_int as ffi::c_uint {
                s = crush_hash32_3(
                    (*bucket).hash as ffi::c_int,
                    x as U32,
                    (*bucket).id as U32,
                    0 as ffi::c_int as U32,
                ) % (*bucket).size;
                *((*work).perm).offset(0 as ffi::c_int as isize) = s;
                (*work).perm_n = 0xffff as ffi::c_int as U32;
                current_block = 3275366147856559585;
            } else {
                i = 0 as ffi::c_int as ffi::c_uint;
                while i < (*bucket).size {
                    *((*work).perm).offset(i as isize) = i;
                    i = i.wrapping_add(1);
                }
                (*work).perm_n = 0 as ffi::c_int as U32;
                current_block = 13056961889198038528;
            }
        } else {
            if (*work).perm_n == 0xffff as ffi::c_int as U32 {
                i = 1 as ffi::c_int as ffi::c_uint;
                while i < (*bucket).size {
                    *((*work).perm).offset(i as isize) = i;
                    i = i.wrapping_add(1);
                }
                *((*work).perm)
                    .offset(*((*work).perm).offset(0 as ffi::c_int as isize) as isize) =
                    0 as ffi::c_int as U32;
                (*work).perm_n = 1 as ffi::c_int as U32;
            }
            current_block = 13056961889198038528;
        }
        if current_block == 13056961889198038528 {
            i = 0 as ffi::c_int as ffi::c_uint;
            while i < (*work).perm_n {
                i = i.wrapping_add(1);
            }
            while (*work).perm_n <= pr {
                let mut p: ffi::c_uint = (*work).perm_n;
                if p < ((*bucket).size).wrapping_sub(1 as ffi::c_int as U32) {
                    i = (crush_hash32_3(
                        (*bucket).hash as ffi::c_int,
                        x as U32,
                        (*bucket).id as U32,
                        p,
                    ))
                    .wrapping_rem(((*bucket).size).wrapping_sub(p));
                    if i != 0 {
                        let mut t: ffi::c_uint =
                            *((*work).perm).offset(p.wrapping_add(i) as isize);
                        *((*work).perm).offset(p.wrapping_add(i) as isize) =
                            *((*work).perm).offset(p as isize);
                        *((*work).perm).offset(p as isize) = t;
                    }
                }
                (*work).perm_n = ((*work).perm_n).wrapping_add(1);
            }
            i = 0 as ffi::c_int as ffi::c_uint;
            while i < (*bucket).size {
                i = i.wrapping_add(1);
            }
            s = *((*work).perm).offset(pr as isize);
        }
        *((*bucket).items).offset(s as isize)
    }
}
unsafe fn bucket_uniform_choose(
    mut bucket: *const CrushBucketUniform,
    mut work: *mut CrushWorkBucket,
    mut x: ffi::c_int,
    mut r: ffi::c_int,
) -> ffi::c_int {
    unsafe { bucket_perm_choose(&(*bucket).h, work, x, r) }
}
unsafe fn bucket_list_choose(
    mut bucket: *const CrushBucketList,
    mut x: ffi::c_int,
    mut r: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_int = 0;
        i = ((*bucket).h.size).wrapping_sub(1 as ffi::c_int as U32) as ffi::c_int;
        while i >= 0 as ffi::c_int {
            let mut w: U64 = crush_hash32_4(
                (*bucket).h.hash as ffi::c_int,
                x as U32,
                *((*bucket).h.items).offset(i as isize) as U32,
                r as U32,
                (*bucket).h.id as U32,
            ) as U64;
            w &= 0xffff as ffi::c_int as U64;
            w *= *((*bucket).sum_weights).offset(i as isize) as U64;
            w >>= 16 as ffi::c_int;
            if w < *((*bucket).item_weights).offset(i as isize) as U64 {
                return *((*bucket).h.items).offset(i as isize);
            }
            i -= 1;
        }
        *((*bucket).h.items).offset(0 as ffi::c_int as isize)
    }
}
fn height(mut n: ffi::c_int) -> ffi::c_int {
    let mut h = 0;
    while n & 1 == 0 {
        h += 1;
        n >>= 1;
    }
    h
}

fn left(x: ffi::c_int) -> ffi::c_int {
    let h = height(x);
    x - ((1 as ffi::c_int) << (h - 1 as ffi::c_int))
}

fn right(x: ffi::c_int) -> ffi::c_int {
    let h = height(x);
    x + ((1 as ffi::c_int) << (h - 1 as ffi::c_int))
}

fn terminal(x: ffi::c_int) -> ffi::c_int {
    x & 1 as ffi::c_int
}
unsafe fn bucket_tree_choose(
    mut bucket: *const CrushBucketTree,
    mut x: ffi::c_int,
    mut r: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut n: ffi::c_int = 0;
        let mut w: U32 = 0;
        let mut t: U64 = 0;
        n = (*bucket).num_nodes as ffi::c_int >> 1 as ffi::c_int;
        while terminal(n) == 0 {
            let mut l: ffi::c_int = 0;
            w = *((*bucket).node_weights).offset(n as isize);
            t = crush_hash32_4(
                (*bucket).h.hash as ffi::c_int,
                x as U32,
                n as U32,
                r as U32,
                (*bucket).h.id as U32,
            ) as U64
                * w as U64;
            t >>= 32 as ffi::c_int;
            l = left(n);
            if t < *((*bucket).node_weights).offset(l as isize) as U64 {
                n = l;
            } else {
                n = right(n);
            }
        }
        *((*bucket).h.items).offset((n >> 1 as ffi::c_int) as isize)
    }
}
unsafe fn bucket_straw_choose(
    mut bucket: *const CrushBucketStraw,
    mut x: ffi::c_int,
    mut r: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut i: U32 = 0;
        let mut high: ffi::c_int = 0 as ffi::c_int;
        let mut high_draw: U64 = 0 as ffi::c_int as U64;
        let mut draw: U64 = 0;
        i = 0 as ffi::c_int as U32;
        while i < (*bucket).h.size {
            draw = crush_hash32_3(
                (*bucket).h.hash as ffi::c_int,
                x as U32,
                *((*bucket).h.items).offset(i as isize) as U32,
                r as U32,
            ) as U64;
            draw &= 0xffff as ffi::c_int as U64;
            draw *= *((*bucket).straws).offset(i as isize) as U64;
            if i == 0 as ffi::c_int as U32 || draw > high_draw {
                high = i as ffi::c_int;
                high_draw = draw;
            }
            i = i.wrapping_add(1);
        }
        *((*bucket).h.items).offset(high as isize)
    }
}
unsafe fn crush_ln(mut xin: ffi::c_uint) -> U64 {
    unsafe {
        let mut x: ffi::c_uint = xin;
        let mut iexpon: ffi::c_int = 0;
        let mut index1: ffi::c_int = 0;
        let mut index2: ffi::c_int = 0;
        let mut RH: U64 = 0;
        let mut LH: U64 = 0;
        let mut LL: U64 = 0;
        let mut xl64: U64 = 0;
        let mut result: U64 = 0;
        x = x.wrapping_add(1);
        iexpon = 15 as ffi::c_int;
        if x & 0x18000 as ffi::c_int as ffi::c_uint == 0 {
            let mut bits: ffi::c_int = (x & 0x1ffff as ffi::c_int as ffi::c_uint).leading_zeros()
                as i32
                - 16 as ffi::c_int;
            x <<= bits;
            iexpon = 15 as ffi::c_int - bits;
        }
        index1 = ((x >> 8 as ffi::c_int) << 1 as ffi::c_int) as ffi::c_int;
        RH = __RH_LH_tbl[(index1 - 256 as ffi::c_int) as usize] as U64;
        LH = __RH_LH_tbl[(index1 + 1 as ffi::c_int - 256 as ffi::c_int) as usize] as U64;
        xl64 = x as S64 as U64 * RH;
        xl64 >>= 48 as ffi::c_int;
        result = iexpon as U64;
        result <<= 12 as ffi::c_int + 32 as ffi::c_int;
        index2 = (xl64 & 0xff as ffi::c_int as U64) as ffi::c_int;
        LL = __LL_tbl[index2 as usize] as U64;
        LH = LH.wrapping_add(LL);
        LH >>= 48 as ffi::c_int - 12 as ffi::c_int - 32 as ffi::c_int;
        result = result.wrapping_add(LH);
        result
    }
}
#[inline]
unsafe fn get_choose_arg_weights(
    mut bucket: *const CrushBucketStraw2,
    mut arg: *const CrushChooseArg,
    mut position: ffi::c_int,
) -> *mut U32 {
    unsafe {
        if arg.is_null()
            || ((*arg).weight_set).is_null()
            || (*arg).weight_set_size == 0 as ffi::c_int as U32
        {
            return (*bucket).item_weights;
        }
        if position as U32 >= (*arg).weight_set_size {
            position =
                ((*arg).weight_set_size).wrapping_sub(1 as ffi::c_int as U32) as ffi::c_int;
        }
        (*((*arg).weight_set).offset(position as isize)).weights
    }
}
#[inline]
unsafe fn get_choose_arg_ids(
    mut bucket: *const CrushBucketStraw2,
    mut arg: *const CrushChooseArg,
) -> *mut ffi::c_int {
    unsafe {
        if arg.is_null() || ((*arg).ids).is_null() {
            return (*bucket).h.items;
        }
        (*arg).ids
    }
}
unsafe fn bucket_straw2_choose(
    mut bucket: *const CrushBucketStraw2,
    mut x: ffi::c_int,
    mut r: ffi::c_int,
    mut arg: *const CrushChooseArg,
    mut position: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        let mut i: ffi::c_uint = 0;
        let mut high: ffi::c_uint = 0 as ffi::c_int as ffi::c_uint;
        let mut u: ffi::c_uint = 0;
        let mut ln: S64 = 0;
        let mut draw: S64 = 0;
        let mut high_draw: S64 = 0 as ffi::c_int as S64;
        let mut weights: *mut U32 = get_choose_arg_weights(bucket, arg, position);
        let mut ids: *mut ffi::c_int = get_choose_arg_ids(bucket, arg);
        i = 0 as ffi::c_int as ffi::c_uint;
        while i < (*bucket).h.size {
            if *weights.offset(i as isize) != 0 {
                u = crush_hash32_3(
                    (*bucket).h.hash as ffi::c_int,
                    x as U32,
                    *ids.offset(i as isize) as U32,
                    r as U32,
                );
                u &= 0xffff as ffi::c_int as ffi::c_uint;
                ln = (crush_ln(u)).wrapping_sub(0x1000000000000 as ffi::c_longlong as U64) as S64;
                draw = ln / *weights.offset(i as isize) as S64;
            } else {
                draw = -((!(0 as ffi::c_ulonglong) >> 1 as ffi::c_int) as S64)
                    - 1 as ffi::c_int as S64;
            }
            if i == 0 as ffi::c_int as ffi::c_uint || draw > high_draw {
                high = i;
                high_draw = draw;
            }
            i = i.wrapping_add(1);
        }
        *((*bucket).h.items).offset(high as isize)
    }
}
unsafe fn crush_bucket_choose(
    mut in_0: *const CrushBucket,
    mut work: *mut CrushWorkBucket,
    mut x: ffi::c_int,
    mut r: ffi::c_int,
    mut arg: *const CrushChooseArg,
    mut position: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        if (*in_0).size != 0 as ffi::c_int as U32 {
        } else {
            panic!("Assertion failed: in->size({}) != 0", (*in_0).size);
        }

        match (*in_0).alg as ffi::c_int {
            1 => bucket_uniform_choose(in_0 as *const CrushBucketUniform, work, x, r),
            2 => bucket_list_choose(in_0 as *const CrushBucketList, x, r),
            3 => bucket_tree_choose(in_0 as *const CrushBucketTree, x, r),
            4 => bucket_straw_choose(in_0 as *const CrushBucketStraw, x, r),
            5 => bucket_straw2_choose(in_0 as *const CrushBucketStraw2, x, r, arg, position),
            _ => *((*in_0).items).offset(0 as ffi::c_int as isize),
        }
    }
}
unsafe fn is_out(
    mut _map: *const CrushMap,
    mut weight: *const U32,
    mut weight_max: ffi::c_int,
    mut item: ffi::c_int,
    mut x: ffi::c_int,
) -> ffi::c_int {
    unsafe {
        if item >= weight_max {
            return 1 as ffi::c_int;
        }
        if *weight.offset(item as isize) >= 0x10000 as ffi::c_int as U32 {
            return 0 as ffi::c_int;
        }
        if *weight.offset(item as isize) == 0 as ffi::c_int as U32 {
            return 1 as ffi::c_int;
        }
        if (crush_hash32_2(0 as ffi::c_int, x as U32, item as U32) & 0xffff as ffi::c_int as U32)
            < *weight.offset(item as isize)
        {
            return 0 as ffi::c_int;
        }
        1 as ffi::c_int
    }
}
unsafe fn crush_choose_firstn(
    mut map: *const CrushMap,
    mut work: *mut CrushWork,
    mut bucket: *const CrushBucket,
    mut weight: *const U32,
    mut weight_max: ffi::c_int,
    mut x: ffi::c_int,
    mut numrep: ffi::c_int,
    mut type_0: ffi::c_int,
    mut out: *mut ffi::c_int,
    mut outpos: ffi::c_int,
    mut out_size: ffi::c_int,
    mut tries: ffi::c_uint,
    mut recurse_tries: ffi::c_uint,
    mut local_retries: ffi::c_uint,
    mut local_fallback_retries: ffi::c_uint,
    mut recurse_to_leaf: ffi::c_int,
    mut vary_r: ffi::c_uint,
    mut stable: ffi::c_uint,
    mut out2: *mut ffi::c_int,
    mut parent_r: ffi::c_int,
    mut choose_args: *const CrushChooseArg,
) -> ffi::c_int {
    unsafe {
        let mut rep: ffi::c_int = 0;
        let mut ftotal: ffi::c_uint = 0;
        let mut flocal: ffi::c_uint = 0;
        let mut retry_descent: ffi::c_int = 0;
        let mut retry_bucket: ffi::c_int = 0;
        let mut skip_rep: ffi::c_int = 0;
        let mut in_0: *const CrushBucket = bucket;
        let mut r: ffi::c_int = 0;
        let mut i: ffi::c_int = 0;
        let mut item: ffi::c_int = 0 as ffi::c_int;
        let mut itemtype: ffi::c_int = 0;
        let mut collide: ffi::c_int = 0;
        let mut reject: ffi::c_int = 0;
        let mut count: ffi::c_int = out_size;
        rep = if stable != 0 {
            0 as ffi::c_int
        } else {
            outpos
        };
        while rep < numrep && count > 0 as ffi::c_int {
            ftotal = 0 as ffi::c_int as ffi::c_uint;
            skip_rep = 0 as ffi::c_int;
            loop {
                retry_descent = 0 as ffi::c_int;
                in_0 = bucket;
                flocal = 0 as ffi::c_int as ffi::c_uint;
                let mut current_block_55: u64;
                loop {
                    collide = 0 as ffi::c_int;
                    retry_bucket = 0 as ffi::c_int;
                    r = rep + parent_r;
                    r = (r as ffi::c_uint).wrapping_add(ftotal) as ffi::c_int as ffi::c_int;
                    if (*in_0).size == 0 as ffi::c_int as U32 {
                        reject = 1 as ffi::c_int;
                        current_block_55 = 5532067231413442433;
                    } else {
                        if local_fallback_retries > 0 as ffi::c_int as ffi::c_uint
                            && flocal >= (*in_0).size >> 1 as ffi::c_int
                            && flocal > local_fallback_retries
                        {
                            item = bucket_perm_choose(
                                in_0,
                                *((*work).work).offset((-(1 as ffi::c_int) - (*in_0).id) as isize),
                                x,
                                r,
                            );
                        } else {
                            item = crush_bucket_choose(
                                in_0,
                                *((*work).work).offset((-(1 as ffi::c_int) - (*in_0).id) as isize),
                                x,
                                r,
                                if !choose_args.is_null() {
                                    &*choose_args
                                        .offset((-(1 as ffi::c_int) - (*in_0).id) as isize)
                                } else {
                                    std::ptr::null::<CrushChooseArg>()
                                },
                                outpos,
                            );
                        }
                        if item >= (*map).max_devices {
                            skip_rep = 1 as ffi::c_int;
                            break;
                        } else {
                            if item < 0 as ffi::c_int {
                                itemtype = (**((*map).buckets)
                                    .offset((-(1 as ffi::c_int) - item) as isize))
                                .type_0 as ffi::c_int;
                            } else {
                                itemtype = 0 as ffi::c_int;
                            }
                            if itemtype != type_0 {
                                if item >= 0 as ffi::c_int
                                    || -(1 as ffi::c_int) - item >= (*map).max_buckets
                                {
                                    skip_rep = 1 as ffi::c_int;
                                    break;
                                } else {
                                    in_0 = *((*map).buckets)
                                        .offset((-(1 as ffi::c_int) - item) as isize);
                                    retry_bucket = 1 as ffi::c_int;
                                }
                                current_block_55 = 13109137661213826276;
                            } else {
                                i = 0 as ffi::c_int;
                                while i < outpos {
                                    if *out.offset(i as isize) == item {
                                        collide = 1 as ffi::c_int;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                                reject = 0 as ffi::c_int;
                                if collide == 0 && recurse_to_leaf != 0 {
                                    if item < 0 as ffi::c_int {
                                        let mut sub_r: ffi::c_int = 0;
                                        if vary_r != 0 {
                                            sub_r = r
                                                >> vary_r
                                                    .wrapping_sub(1 as ffi::c_int as ffi::c_uint);
                                        } else {
                                            sub_r = 0 as ffi::c_int;
                                        }
                                        if crush_choose_firstn(
                                            map,
                                            work,
                                            *((*map).buckets)
                                                .offset((-(1 as ffi::c_int) - item) as isize),
                                            weight,
                                            weight_max,
                                            x,
                                            if stable != 0 {
                                                1 as ffi::c_int
                                            } else {
                                                outpos + 1 as ffi::c_int
                                            },
                                            0 as ffi::c_int,
                                            out2,
                                            outpos,
                                            count,
                                            recurse_tries,
                                            0 as ffi::c_int as ffi::c_uint,
                                            local_retries,
                                            local_fallback_retries,
                                            0 as ffi::c_int,
                                            vary_r,
                                            stable,
                                            std::ptr::null_mut::<ffi::c_int>(),
                                            sub_r,
                                            choose_args,
                                        ) <= outpos
                                        {
                                            reject = 1 as ffi::c_int;
                                        }
                                    } else {
                                        *out2.offset(outpos as isize) = item;
                                    }
                                }
                                if reject == 0 && collide == 0 && itemtype == 0 as ffi::c_int {
                                    reject = is_out(map, weight, weight_max, item, x);
                                }
                                current_block_55 = 5532067231413442433;
                            }
                        }
                    }
                    if current_block_55 == 5532067231413442433 && (reject != 0 || collide != 0) {
                        ftotal = ftotal.wrapping_add(1);
                        flocal = flocal.wrapping_add(1);

                        #[allow(clippy::if_same_then_else)]
                        // TODO(sevki): remove this later.
                        if collide != 0 && flocal <= local_retries {
                            retry_bucket = 1 as ffi::c_int;
                        } else if local_fallback_retries > 0 as ffi::c_int as ffi::c_uint
                            && flocal <= ((*in_0).size).wrapping_add(local_fallback_retries)
                        {
                            retry_bucket = 1 as ffi::c_int;
                        } else if ftotal < tries {
                            retry_descent = 1 as ffi::c_int;
                        } else {
                            skip_rep = 1 as ffi::c_int;
                        }
                    }
                    if retry_bucket == 0 {
                        break;
                    }
                }
                if retry_descent == 0 {
                    break;
                }
            }
            if skip_rep == 0 {
                *out.offset(outpos as isize) = item;
                outpos += 1;
                count -= 1;
                if !((*map).choose_tries).is_null() && ftotal <= (*map).choose_total_tries {
                    let fresh0 = &mut (*((*map).choose_tries).offset(ftotal as isize));
                    *fresh0 = (*fresh0).wrapping_add(1);
                }
            }
            rep += 1;
        }
        outpos
    }
}
unsafe fn crush_choose_indep(
    mut map: *const CrushMap,
    mut work: *mut CrushWork,
    mut bucket: *const CrushBucket,
    mut weight: *const U32,
    mut weight_max: ffi::c_int,
    mut x: ffi::c_int,
    mut left_0: ffi::c_int,
    mut numrep: ffi::c_int,
    mut type_0: ffi::c_int,
    mut out: *mut ffi::c_int,
    mut outpos: ffi::c_int,
    mut tries: ffi::c_uint,
    mut recurse_tries: ffi::c_uint,
    mut recurse_to_leaf: ffi::c_int,
    mut out2: *mut ffi::c_int,
    mut parent_r: ffi::c_int,
    mut choose_args: *const CrushChooseArg,
) {
    unsafe {
        let mut in_0: *const CrushBucket = bucket;
        let mut endpos: ffi::c_int = outpos + left_0;
        let mut rep: ffi::c_int = 0;
        let mut ftotal: ffi::c_uint = 0;
        let mut r: ffi::c_int = 0;
        let mut i: ffi::c_int = 0;
        let mut item: ffi::c_int = 0 as ffi::c_int;
        let mut itemtype: ffi::c_int = 0;
        let mut collide: ffi::c_int = 0;
        rep = outpos;
        while rep < endpos {
            *out.offset(rep as isize) = 0x7ffffffe as ffi::c_int;
            if !out2.is_null() {
                *out2.offset(rep as isize) = 0x7ffffffe as ffi::c_int;
            }
            rep += 1;
        }
        ftotal = 0 as ffi::c_int as ffi::c_uint;
        while left_0 > 0 as ffi::c_int && ftotal < tries {
            rep = outpos;
            while rep < endpos {
                if *out.offset(rep as isize) == 0x7ffffffe as ffi::c_int {
                    in_0 = bucket;
                    loop {
                        r = rep + parent_r;
                        if (*in_0).alg as ffi::c_int == CRUSH_BUCKET_UNIFORM as ffi::c_int
                            && (*in_0).size % numrep as U32 == 0 as ffi::c_int as U32
                        {
                            r = (r as ffi::c_uint).wrapping_add(
                                ((numrep + 1 as ffi::c_int) as ffi::c_uint).wrapping_mul(ftotal),
                            ) as ffi::c_int as ffi::c_int;
                        } else {
                            r = (r as ffi::c_uint)
                                .wrapping_add((numrep as ffi::c_uint).wrapping_mul(ftotal))
                                as ffi::c_int as ffi::c_int;
                        }
                        if (*in_0).size == 0 as ffi::c_int as U32 {
                            break;
                        }
                        item = crush_bucket_choose(
                            in_0,
                            *((*work).work).offset((-(1 as ffi::c_int) - (*in_0).id) as isize),
                            x,
                            r,
                            if !choose_args.is_null() {
                                &*choose_args.offset((-(1 as ffi::c_int) - (*in_0).id) as isize)
                            } else {
                                std::ptr::null::<CrushChooseArg>()
                            },
                            outpos,
                        );
                        if item >= (*map).max_devices {
                            *out.offset(rep as isize) = 0x7fffffff as ffi::c_int;
                            if !out2.is_null() {
                                *out2.offset(rep as isize) = 0x7fffffff as ffi::c_int;
                            }
                            left_0 -= 1;
                            break;
                        } else {
                            if item < 0 as ffi::c_int {
                                itemtype = (**((*map).buckets)
                                    .offset((-(1 as ffi::c_int) - item) as isize))
                                .type_0 as ffi::c_int;
                            } else {
                                itemtype = 0 as ffi::c_int;
                            }
                            if itemtype != type_0 {
                                if item >= 0 as ffi::c_int
                                    || -(1 as ffi::c_int) - item >= (*map).max_buckets
                                {
                                    *out.offset(rep as isize) = 0x7fffffff as ffi::c_int;
                                    if !out2.is_null() {
                                        *out2.offset(rep as isize) = 0x7fffffff as ffi::c_int;
                                    }
                                    left_0 -= 1;
                                    break;
                                } else {
                                    in_0 = *((*map).buckets)
                                        .offset((-(1 as ffi::c_int) - item) as isize);
                                }
                            } else {
                                collide = 0 as ffi::c_int;
                                i = outpos;
                                while i < endpos {
                                    if *out.offset(i as isize) == item {
                                        collide = 1 as ffi::c_int;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                                if collide != 0 {
                                    break;
                                }
                                if recurse_to_leaf != 0 {
                                    if item < 0 as ffi::c_int {
                                        crush_choose_indep(
                                            map,
                                            work,
                                            *((*map).buckets)
                                                .offset((-(1 as ffi::c_int) - item) as isize),
                                            weight,
                                            weight_max,
                                            x,
                                            1 as ffi::c_int,
                                            numrep,
                                            0 as ffi::c_int,
                                            out2,
                                            rep,
                                            recurse_tries,
                                            0 as ffi::c_int as ffi::c_uint,
                                            0 as ffi::c_int,
                                            std::ptr::null_mut::<ffi::c_int>(),
                                            r,
                                            choose_args,
                                        );
                                        if *out2.offset(rep as isize) == 0x7fffffff as ffi::c_int {
                                            break;
                                        }
                                    } else {
                                        *out2.offset(rep as isize) = item;
                                    }
                                }
                                if itemtype == 0 as ffi::c_int
                                    && is_out(map, weight, weight_max, item, x) != 0
                                {
                                    break;
                                }
                                *out.offset(rep as isize) = item;
                                left_0 -= 1;
                                break;
                            }
                        }
                    }
                }
                rep += 1;
            }
            ftotal = ftotal.wrapping_add(1);
        }
        rep = outpos;
        while rep < endpos {
            if *out.offset(rep as isize) == 0x7ffffffe as ffi::c_int {
                *out.offset(rep as isize) = 0x7fffffff as ffi::c_int;
            }
            if !out2.is_null() && *out2.offset(rep as isize) == 0x7ffffffe as ffi::c_int {
                *out2.offset(rep as isize) = 0x7fffffff as ffi::c_int;
            }
            rep += 1;
        }
        if !((*map).choose_tries).is_null() && ftotal <= (*map).choose_total_tries {
            let fresh1 = &mut (*((*map).choose_tries).offset(ftotal as isize));
            *fresh1 = (*fresh1).wrapping_add(1);
        }
    }
}
pub unsafe fn crush_init_workspace(m: *const CrushMap, v: *mut ffi::c_void) {
    unsafe {
        let w: *mut CrushWork = v as *mut CrushWork;
        let mut point: *mut ffi::c_char = v as *mut ffi::c_char;
        
        point = point.offset(::core::mem::size_of::<CrushWork>() as ffi::c_ulong as isize);
        (*w).work = point as *mut *mut CrushWorkBucket;
        point = point.offset(
            ((*m).max_buckets as ffi::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut CrushWorkBucket>() as ffi::c_ulong)
                as isize,
        );
        
        for b in 0..(*m).max_buckets {
            if !(*((*m).buckets).offset(b as isize)).is_null() {
                let fresh2 = &mut (*((*w).work).offset(b as isize));
                *fresh2 = point as *mut CrushWorkBucket;
                let _ = ((*m).buckets).offset(b as isize);
                {}
                point = point
                    .offset(::core::mem::size_of::<CrushWorkBucket>() as ffi::c_ulong as isize);
                (**((*w).work).offset(b as isize)).perm_x = 0;
                (**((*w).work).offset(b as isize)).perm_n = 0;
                let fresh3 = &mut (**((*w).work).offset(b as isize)).perm;
                *fresh3 = point as *mut U32;
                point = point.offset(
                    ((**((*m).buckets).offset(b as isize)).size as ffi::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<U32>() as ffi::c_ulong)
                        as isize,
                );
            }
        }
        
        if point.offset_from(w as *mut ffi::c_char) as ffi::c_long as SizeT == (*m).working_size {
        } else {
            panic!("Assertion failed: !((char *)point - (char *)w != m->working_size)");
        }
    }
}
pub unsafe fn crush_do_rule(
    mut map: *const CrushMap,
    mut ruleno: ffi::c_int,
    mut x: ffi::c_int,
    mut result: *mut ffi::c_int,
    mut result_max: ffi::c_int,
    mut weight: *const U32,
    mut weight_max: ffi::c_int,
    mut cwin: *mut ffi::c_void,
    mut choose_args: *const CrushChooseArg,
) -> ffi::c_int {
    unsafe {
        let mut result_len: ffi::c_int = 0;
        let mut cw: *mut CrushWork = cwin as *mut CrushWork;
        let mut a: *mut ffi::c_int =
            (cw as *mut ffi::c_char).offset((*map).working_size as isize) as *mut ffi::c_int;
        let mut b: *mut ffi::c_int = a.offset(result_max as isize);
        let mut c: *mut ffi::c_int = b.offset(result_max as isize);
        let mut w: *mut ffi::c_int = a;
        let mut o: *mut ffi::c_int = b;
        let mut recurse_to_leaf: ffi::c_int = 0;
        let mut wsize: ffi::c_int = 0 as ffi::c_int;
        let mut osize: ffi::c_int = 0;
        let mut tmp: *mut ffi::c_int = std::ptr::null_mut::<ffi::c_int>();
        let mut rule: *const CrushRule = std::ptr::null::<CrushRule>();
        let mut step: U32 = 0;
        let mut i: ffi::c_int = 0;
        let mut j: ffi::c_int = 0;
        let mut numrep: ffi::c_int = 0;
        let mut out_size: ffi::c_int = 0;
        let mut choose_tries: ffi::c_int =
            ((*map).choose_total_tries).wrapping_add(1 as ffi::c_int as U32) as ffi::c_int;
        let mut choose_leaf_tries: ffi::c_int = 0 as ffi::c_int;
        let mut choose_local_retries: ffi::c_int = (*map).choose_local_tries as ffi::c_int;
        let mut choose_local_fallback_retries: ffi::c_int =
            (*map).choose_local_fallback_tries as ffi::c_int;
        let mut vary_r: ffi::c_int = (*map).chooseleaf_vary_r as ffi::c_int;
        let mut stable: ffi::c_int = (*map).chooseleaf_stable as ffi::c_int;
        if ruleno as U32 >= (*map).max_rules {
            return 0 as ffi::c_int;
        }
        rule = *((*map).rules).offset(ruleno as isize);
        result_len = 0 as ffi::c_int;
        step = 0 as ffi::c_int as U32;
        while step < (*rule).len {
            let mut firstn: ffi::c_int = 0 as ffi::c_int;
            let mut curstep: *const CrushRuleStep =
                &*((*rule).steps).as_ptr().offset(step as isize) as *const CrushRuleStep;
            let mut current_block_59: u64;
            match (*curstep).op {
                1 => {
                    if (*curstep).arg1 >= 0 as ffi::c_int && (*curstep).arg1 < (*map).max_devices
                        || -(1 as ffi::c_int) - (*curstep).arg1 >= 0 as ffi::c_int
                            && -(1 as ffi::c_int) - (*curstep).arg1 < (*map).max_buckets
                            && !(*((*map).buckets)
                                .offset((-(1 as ffi::c_int) - (*curstep).arg1) as isize))
                            .is_null()
                    {
                        *w.offset(0 as ffi::c_int as isize) = (*curstep).arg1;
                        wsize = 1 as ffi::c_int;
                    }
                    current_block_59 = 15462640364611497761;
                }
                8 => {
                    if (*curstep).arg1 > 0 as ffi::c_int {
                        choose_tries = (*curstep).arg1;
                    }
                    current_block_59 = 15462640364611497761;
                }
                9 => {
                    if (*curstep).arg1 > 0 as ffi::c_int {
                        choose_leaf_tries = (*curstep).arg1;
                    }
                    current_block_59 = 15462640364611497761;
                }
                10 => {
                    if (*curstep).arg1 >= 0 as ffi::c_int {
                        choose_local_retries = (*curstep).arg1;
                    }
                    current_block_59 = 15462640364611497761;
                }
                11 => {
                    if (*curstep).arg1 >= 0 as ffi::c_int {
                        choose_local_fallback_retries = (*curstep).arg1;
                    }
                    current_block_59 = 15462640364611497761;
                }
                12 => {
                    if (*curstep).arg1 >= 0 as ffi::c_int {
                        vary_r = (*curstep).arg1;
                    }
                    current_block_59 = 15462640364611497761;
                }
                13 => {
                    if (*curstep).arg1 >= 0 as ffi::c_int {
                        stable = (*curstep).arg1;
                    }
                    current_block_59 = 15462640364611497761;
                }
                6 | 2 => {
                    firstn = 1 as ffi::c_int;
                    current_block_59 = 7054583439108689069;
                }
                7 | 3 => {
                    current_block_59 = 7054583439108689069;
                }
                4 => {
                    i = 0 as ffi::c_int;
                    while i < wsize && result_len < result_max {
                        *result.offset(result_len as isize) = *w.offset(i as isize);
                        result_len += 1;
                        i += 1;
                    }
                    wsize = 0 as ffi::c_int;
                    current_block_59 = 15462640364611497761;
                }
                _ => {
                    current_block_59 = 15462640364611497761;
                }
            }
            if current_block_59 == 7054583439108689069 && wsize != 0 as ffi::c_int {
                recurse_to_leaf = ((*curstep).op
                    == CRUSH_RULE_CHOOSELEAF_FIRSTN as ffi::c_int as U32
                    || (*curstep).op == CRUSH_RULE_CHOOSELEAF_INDEP as ffi::c_int as U32)
                    as ffi::c_int;
                osize = 0 as ffi::c_int;
                let mut current_block_45: u64;
                i = 0 as ffi::c_int;
                while i < wsize {
                    let mut bno: ffi::c_int = 0;
                    numrep = (*curstep).arg1;
                    if numrep <= 0 as ffi::c_int {
                        numrep += result_max;
                        if numrep <= 0 as ffi::c_int {
                            current_block_45 = 3934796541983872331;
                        } else {
                            current_block_45 = 6717214610478484138;
                        }
                    } else {
                        current_block_45 = 6717214610478484138;
                    }
                    if current_block_45 == 6717214610478484138 {
                        j = 0 as ffi::c_int;
                        bno = -(1 as ffi::c_int) - *w.offset(i as isize);
                        if !(bno < 0 as ffi::c_int || bno >= (*map).max_buckets) {
                            if firstn != 0 {
                                let mut recurse_tries: ffi::c_int = 0;
                                if choose_leaf_tries != 0 {
                                    recurse_tries = choose_leaf_tries;
                                } else if (*map).chooseleaf_descend_once != 0 {
                                    recurse_tries = 1 as ffi::c_int;
                                } else {
                                    recurse_tries = choose_tries;
                                }
                                osize += crush_choose_firstn(
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
                                    choose_tries as ffi::c_uint,
                                    recurse_tries as ffi::c_uint,
                                    choose_local_retries as ffi::c_uint,
                                    choose_local_fallback_retries as ffi::c_uint,
                                    recurse_to_leaf,
                                    vary_r as ffi::c_uint,
                                    stable as ffi::c_uint,
                                    c.offset(osize as isize),
                                    0 as ffi::c_int,
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
                                    choose_tries as ffi::c_uint,
                                    (if choose_leaf_tries != 0 {
                                        choose_leaf_tries
                                    } else {
                                        1 as ffi::c_int
                                    }) as ffi::c_uint,
                                    recurse_to_leaf,
                                    c.offset(osize as isize),
                                    0 as ffi::c_int,
                                    choose_args,
                                );
                                osize += out_size;
                            }
                        }
                    }
                    i += 1;
                }
                if recurse_to_leaf != 0 {
                    memcpy(
                        o as *mut ffi::c_void,
                        c as *const ffi::c_void,
                        (osize as ffi::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<ffi::c_int>() as ffi::c_ulong),
                    );
                }
                tmp = o;
                o = w;
                w = tmp;
                wsize = osize;
            }
            step = step.wrapping_add(1);
        }
        result_len
    }
}
