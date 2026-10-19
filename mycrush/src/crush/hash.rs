//! Robert Jenkins' 32-bit integer hash, as used by CRUSH.
// The trailing mix of each variant leaves unread values in the mixed inputs,
// exactly as the C macro does.
#![allow(unused_assignments)]
// The full C hash API is kept for parity even where mycrush does not call it.
#![allow(dead_code)]

const SEED: u32 = 1_315_423_911;

/// Hash function selector (`CRUSH_HASH_*`).
const HASH_RJENKINS1: i32 = 0;

/// One round of Jenkins' reversible mix.
fn hashmix(mut a: u32, mut b: u32, mut c: u32) -> (u32, u32, u32) {
    a = a.wrapping_sub(b).wrapping_sub(c) ^ (c >> 13);
    b = b.wrapping_sub(c).wrapping_sub(a) ^ (a << 8);
    c = c.wrapping_sub(a).wrapping_sub(b) ^ (b >> 13);
    a = a.wrapping_sub(b).wrapping_sub(c) ^ (c >> 12);
    b = b.wrapping_sub(c).wrapping_sub(a) ^ (a << 16);
    c = c.wrapping_sub(a).wrapping_sub(b) ^ (b >> 5);
    a = a.wrapping_sub(b).wrapping_sub(c) ^ (c >> 3);
    b = b.wrapping_sub(c).wrapping_sub(a) ^ (a << 10);
    c = c.wrapping_sub(a).wrapping_sub(b) ^ (b >> 15);
    (a, b, c)
}

/// Mixes `$a` and `$b` into `$hash`, updating all three in place.
macro_rules! mix {
    ($a:ident, $b:ident, $hash:ident) => {
        ($a, $b, $hash) = hashmix($a, $b, $hash);
    };
}

fn rjenkins1(mut a: u32) -> u32 {
    let mut hash = SEED ^ a;
    let (mut b, mut x, mut y) = (a, 231_232, 1_232);
    mix!(b, x, hash);
    mix!(y, a, hash);
    hash
}

fn rjenkins1_2(mut a: u32, mut b: u32) -> u32 {
    let mut hash = SEED ^ a ^ b;
    let (mut x, mut y) = (231_232, 1_232);
    mix!(a, b, hash);
    mix!(x, a, hash);
    mix!(b, y, hash);
    hash
}

fn rjenkins1_3(mut a: u32, mut b: u32, mut c: u32) -> u32 {
    let mut hash = SEED ^ a ^ b ^ c;
    let (mut x, mut y) = (231_232, 1_232);
    mix!(a, b, hash);
    mix!(c, x, hash);
    mix!(y, a, hash);
    mix!(b, x, hash);
    mix!(y, c, hash);
    hash
}

fn rjenkins1_4(mut a: u32, mut b: u32, mut c: u32, mut d: u32) -> u32 {
    let mut hash = SEED ^ a ^ b ^ c ^ d;
    let (mut x, mut y) = (231_232, 1_232);
    mix!(a, b, hash);
    mix!(c, d, hash);
    mix!(a, x, hash);
    mix!(y, b, hash);
    mix!(c, x, hash);
    mix!(y, d, hash);
    hash
}

fn rjenkins1_5(mut a: u32, mut b: u32, mut c: u32, mut d: u32, mut e: u32) -> u32 {
    let mut hash = SEED ^ a ^ b ^ c ^ d ^ e;
    let (mut x, mut y) = (231_232, 1_232);
    mix!(a, b, hash);
    mix!(c, d, hash);
    mix!(e, x, hash);
    mix!(y, a, hash);
    mix!(b, x, hash);
    mix!(y, c, hash);
    mix!(d, x, hash);
    mix!(y, e, hash);
    hash
}

/// Robert Jenkins' hash (`CRUSH_HASH_RJENKINS1`) as a [`digest::Digest`].
///
/// CRUSH hashes between one and five 32-bit words, and the word count selects
/// the mixing schedule, so the input must be exactly 4, 8, 12, 16 or 20 bytes
/// (native-endian `u32`s, as `u32::to_ne_bytes` produces). Any other length
/// hashes to 0, like an unknown hash type in the C implementation. The 4-byte
/// output is the hash in little-endian order; see [`RJenkins1::to_u32`].
#[derive(Debug, Clone, Default)]
pub struct RJenkins1 {
    buf: [u8; Self::MAX_BYTES],
    len: usize,
}

impl RJenkins1 {
    const MAX_BYTES: usize = 20;
    pub const NAME: &'static str = "rjenkins1";

    /// Converts a digest output back into the `u32` hash value.
    pub fn to_u32(out: &digest::Output<Self>) -> u32 {
        u32::from_le_bytes((*out).into())
    }

    fn word(&self, i: usize) -> u32 {
        let w = &self.buf[i * 4..i * 4 + 4];
        u32::from_ne_bytes([w[0], w[1], w[2], w[3]])
    }
}

impl digest::HashMarker for RJenkins1 {}

impl digest::OutputSizeUser for RJenkins1 {
    type OutputSize = digest::consts::U4;
}

impl digest::Update for RJenkins1 {
    fn update(&mut self, data: &[u8]) {
        // Bytes past the buffer are dropped but still counted, so the
        // oversized input is rejected at finalization.
        let room = Self::MAX_BYTES - self.len.min(Self::MAX_BYTES);
        let take = data.len().min(room);
        self.buf[self.len..self.len + take].copy_from_slice(&data[..take]);
        self.len = self.len.saturating_add(data.len());
    }
}

impl digest::Reset for RJenkins1 {
    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl digest::FixedOutput for RJenkins1 {
    fn finalize_into(self, out: &mut digest::Output<Self>) {
        let w = |i| self.word(i);
        let hash = match self.len {
            4 => rjenkins1(w(0)),
            8 => rjenkins1_2(w(0), w(1)),
            12 => rjenkins1_3(w(0), w(1), w(2)),
            16 => rjenkins1_4(w(0), w(1), w(2), w(3)),
            20 => rjenkins1_5(w(0), w(1), w(2), w(3), w(4)),
            _ => 0,
        };
        *out = hash.to_le_bytes().into();
    }
}

// Runtime-dispatched entry points used by the mapper until it is made generic
// over `digest::Digest`. Unknown hash types yield 0, as in the C implementation.

pub fn crush_hash32(ty: i32, a: u32) -> u32 {
    if ty == HASH_RJENKINS1 {
        rjenkins1(a)
    } else {
        0
    }
}

pub fn crush_hash32_2(ty: i32, a: u32, b: u32) -> u32 {
    if ty == HASH_RJENKINS1 {
        rjenkins1_2(a, b)
    } else {
        0
    }
}

pub fn crush_hash32_3(ty: i32, a: u32, b: u32, c: u32) -> u32 {
    if ty == HASH_RJENKINS1 {
        rjenkins1_3(a, b, c)
    } else {
        0
    }
}

pub fn crush_hash32_4(ty: i32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    if ty == HASH_RJENKINS1 {
        rjenkins1_4(a, b, c, d)
    } else {
        0
    }
}

pub fn crush_hash32_5(ty: i32, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    if ty == HASH_RJENKINS1 {
        rjenkins1_5(a, b, c, d, e)
    } else {
        0
    }
}

pub fn crush_hash_name(ty: i32) -> &'static str {
    if ty == HASH_RJENKINS1 {
        RJenkins1::NAME
    } else {
        "unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic spread of inputs, including edge values.
    fn samples() -> Vec<u32> {
        let mut v = vec![0, 1, 2, 0xff, 0xffff, 0x7fff_ffff, 0x8000_0000, u32::MAX];
        let mut x = 0x1234_5678u32;
        for _ in 0..200 {
            x = x.wrapping_mul(1664525).wrapping_add(1013904223);
            v.push(x);
        }
        v
    }

    #[test]
    fn matches_c_oracle() {
        let s = samples();
        for (i, &a) in s.iter().enumerate() {
            let b = s[(i * 7 + 1) % s.len()];
            let c = s[(i * 13 + 2) % s.len()];
            let d = s[(i * 17 + 3) % s.len()];
            let e = s[(i * 19 + 4) % s.len()];
            for ty in [0, 1, -1] {
                unsafe {
                    assert_eq!(crush_hash32(ty, a), crush_sys::crush_hash32(ty, a));
                    assert_eq!(
                        crush_hash32_2(ty, a, b),
                        crush_sys::crush_hash32_2(ty, a, b)
                    );
                    assert_eq!(
                        crush_hash32_3(ty, a, b, c),
                        crush_sys::crush_hash32_3(ty, a, b, c)
                    );
                    assert_eq!(
                        crush_hash32_4(ty, a, b, c, d),
                        crush_sys::crush_hash32_4(ty, a, b, c, d)
                    );
                    assert_eq!(
                        crush_hash32_5(ty, a, b, c, d, e),
                        crush_sys::crush_hash32_5(ty, a, b, c, d, e)
                    );
                }
            }
        }
    }

    fn digest_of(words: &[u32]) -> u32 {
        use digest::Digest;
        let mut h = RJenkins1::new();
        for w in words {
            h.update(w.to_ne_bytes());
        }
        RJenkins1::to_u32(&h.finalize())
    }

    #[test]
    fn digest_matches_c_oracle() {
        for (i, &a) in samples().iter().enumerate() {
            let (b, c, d, e) = (a.rotate_left(3), !a, a ^ 0xdead_beef, i as u32);
            unsafe {
                assert_eq!(digest_of(&[a]), crush_sys::crush_hash32(0, a));
                assert_eq!(digest_of(&[a, b]), crush_sys::crush_hash32_2(0, a, b));
                assert_eq!(digest_of(&[a, b, c]), crush_sys::crush_hash32_3(0, a, b, c));
                assert_eq!(
                    digest_of(&[a, b, c, d]),
                    crush_sys::crush_hash32_4(0, a, b, c, d)
                );
                assert_eq!(
                    digest_of(&[a, b, c, d, e]),
                    crush_sys::crush_hash32_5(0, a, b, c, d, e)
                );
            }
        }
    }

    #[test]
    fn digest_rejects_unsupported_lengths() {
        use digest::Digest;
        assert_eq!(RJenkins1::to_u32(&RJenkins1::digest([])), 0);
        assert_eq!(RJenkins1::to_u32(&RJenkins1::digest([1, 2, 3])), 0);
        assert_eq!(RJenkins1::to_u32(&RJenkins1::digest([7; 21])), 0);
    }

    #[test]
    fn hash_names() {
        assert_eq!(crush_hash_name(0), "rjenkins1");
        assert_eq!(crush_hash_name(1), "unknown");
    }
}
