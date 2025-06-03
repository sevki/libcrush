const CRUSH_HASH_SEED: u32 = 1315423911;

// Helper macro translated to an inline function.
// In Rust, macros are possible, but for this, an inline function is cleaner.
// The C macro modifies its arguments in place. In Rust, we'll pass them as mutable.
#[inline]
fn crush_hash_mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = a.wrapping_sub(*b); *a = a.wrapping_sub(*c); *a ^= c.wrapping_shr(13);
    *b = b.wrapping_sub(*c); *b = b.wrapping_sub(*a); *b ^= a.wrapping_shl(8);
    *c = c.wrapping_sub(*a); *c = c.wrapping_sub(*b); *c ^= b.wrapping_shr(13);
    *a = a.wrapping_sub(*b); *a = a.wrapping_sub(*c); *a ^= c.wrapping_shr(12);
    *b = b.wrapping_sub(*c); *b = b.wrapping_sub(*a); *b ^= a.wrapping_shl(16);
    *c = c.wrapping_sub(*a); *c = c.wrapping_sub(*b); *c ^= b.wrapping_shr(5);
    *a = a.wrapping_sub(*b); *a = a.wrapping_sub(*c); *a ^= c.wrapping_shr(3);
    *b = b.wrapping_sub(*c); *b = b.wrapping_sub(*a); *b ^= a.wrapping_shl(10);
    *c = c.wrapping_sub(*a); *c = c.wrapping_sub(*b); *c ^= b.wrapping_shr(15);
}

/// Corresponds to `crush_hash32_rjenkins1_3` from `crush/hash.c`
/// This is the Jenkins hash function variant used by CRUSH for 3 u32 inputs.
pub fn crush_hash32_rjenkins1(input_a: u32, input_b: u32, input_c: u32) -> u32 {
    let mut hash = CRUSH_HASH_SEED ^ input_a ^ input_b ^ input_c;

    // Need mutable copies of inputs for the mix operations,
    // as the original C macro modifies its parameters.
    let mut a = input_a;
    let mut b = input_b;
    let mut c = input_c;

    let mut x: u32 = 231232;
    let mut y: u32 = 1232;

    // crush_hashmix(a, b, hash);
    crush_hash_mix(&mut a, &mut b, &mut hash);

    // crush_hashmix(c, x, hash);
    crush_hash_mix(&mut c, &mut x, &mut hash);

    // crush_hashmix(y, a, hash);
    // Re-initialize x because its value from previous mix is not used for 'y' parameter.
    // The C macro passes variables by value essentially, then they are modified.
    // In crush_hash32_rjenkins1_3, 'a' (the original input_a) is passed to the third mix.
    // 'y' is a local variable.
    // Let's trace the C macro carefully:
    // crush_hashmix(a, b, hash); -> a, b, hash are modified.
    // crush_hashmix(c, x, hash); -> c (input_c), x (local), hash are modified.
    // crush_hashmix(y, a, hash); -> y (local), a (which was input_a, now modified by first mix), hash are modified.
    // This means we need to use the *modified* 'a' from the first mix.
    crush_hash_mix(&mut y, &mut a, &mut hash);

    // crush_hashmix(b, x, hash);
    // 'b' is from first mix. 'x' needs to be re-initialized to its original value for this call if the C logic means "original x".
    // The C code reuses the variable 'x' which has been modified by the second call to crush_hashmix.
    // This is a key detail: the C macro modifies its parameters. So 'x' used in the 4th call is the 'x' modified by the 2nd call.
    // 'c' in the 5th call is the 'c' modified by the 2nd call.
    crush_hash_mix(&mut b, &mut x, &mut hash);

    // crush_hashmix(y, c, hash);
    // 'y' is from 3rd mix. 'c' is from 2nd mix.
    crush_hash_mix(&mut y, &mut c, &mut hash);

    hash
}

// Example of how other variants could be defined if needed:
// pub fn crush_hash32_rjenkins1_1(input_a: u32) -> u32 { ... }
// pub fn crush_hash32_rjenkins1_2(input_a: u32, input_b: u32) -> u32 { ... }
