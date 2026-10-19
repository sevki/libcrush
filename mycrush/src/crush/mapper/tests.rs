use super::*;

/// Original c2rust-shaped implementation, kept as a test oracle. Its `unsafe`
/// block dates from when the tables were `static mut`.
#[allow(unused_unsafe)]
unsafe fn crush_ln_unsafe_oracle(mut xin: ffi::c_uint) -> U64 {
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

/// straw2 only ever draws from `0..=0xffff`; check the whole domain.
#[test]
fn crush_ln_matches_oracle_over_the_whole_domain() {
    for x in 0..=0xffff {
        assert_eq!(
            crush_ln(x),
            unsafe { crush_ln_unsafe_oracle(x) },
            "x={x:#x}"
        );
    }
}

#[test]
fn crush_ln_stays_within_the_straw2_range() {
    // straw2 subtracts 2^48, so every draw is <= 0 and approaches 0 at the top.
    assert!((0..=0xffff).all(|x| crush_ln(x) <= 0x1000000000000));
    assert!(crush_ln(0xffff) > 0x1000000000000 - (1 << 30));
}
