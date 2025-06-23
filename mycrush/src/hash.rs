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

//! CRUSH hashing functions.
//!
//! This module re-implements the hashing functions found in the original
//! C libcrush library, specifically the Robert Jenkins' 1-at-a-time hash.

/// Represents the available hash algorithms.
/// Currently, only RJENKINS1 is supported, mirroring the C library's behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    /// Robert Jenkins' 1-at-a-time hash.
    Rjenkins1 = 0,
    // Placeholder for other potential hash types if ever needed.
    // Unknown = -1, // Or some other value not 0
}

impl HashAlgorithm {
    /// Gets the algorithm from an integer type.
    /// Returns `None` if the type is not recognized.
    pub fn from_i32(type_val: i32) -> Option<Self> {
        match type_val {
            0 => Some(HashAlgorithm::Rjenkins1),
            _ => None,
        }
    }

    /// Returns the name of the hash algorithm.
    pub fn name(&self) -> &'static str {
        match self {
            HashAlgorithm::Rjenkins1 => "rjenkins1",
        }
    }
}

// Private helper function for rjenkins1 with 1 input.
// Corrected rjenkins1_1 based on direct translation of the C code's single argument version
fn rjenkins1_direct_translation(val_a: u32) -> u32 {
    let mut hash: u32 = 1_315_423_911u32 ^ val_a;
    let mut b: u32 = val_a;
    let mut x: u32 = 231_232u32;
    let mut a_param: u32 = val_a;
    let mut y: u32 = 1_232u32;

    // Macro crush_hash_mix() applied once with b, x, hash
    b = b.wrapping_sub(x); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(13);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(b); x ^= b.wrapping_shl(8);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(13);
    b = b.wrapping_sub(x); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(12);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(b); x ^= b.wrapping_shl(16);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(5);
    b = b.wrapping_sub(x); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(3);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(b); x ^= b.wrapping_shl(10);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(15);

    // Second mix block from C, using y and a_param (which is original 'a')
    y = y.wrapping_sub(a_param); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(13);
    a_param = a_param.wrapping_sub(hash); a_param = a_param.wrapping_sub(y); a_param ^= y.wrapping_shl(8);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(a_param); hash ^= a_param.wrapping_shr(13);
    y = y.wrapping_sub(a_param); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(12);
    a_param = a_param.wrapping_sub(hash); a_param = a_param.wrapping_sub(y); a_param ^= y.wrapping_shl(16);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(a_param); hash ^= a_param.wrapping_shr(5);
    y = y.wrapping_sub(a_param); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(3);
    a_param = a_param.wrapping_sub(hash); a_param = a_param.wrapping_sub(y); a_param ^= y.wrapping_shl(10);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(a_param); hash ^= a_param.wrapping_shr(15);

    hash
}


// Private helper function for rjenkins1 with 2 inputs.
fn rjenkins1_2(mut a: u32, mut b: u32) -> u32 {
    let mut hash: u32 = 1_315_423_911u32 ^ a ^ b;
    let mut x: u32 = 231_232u32;
    let mut y: u32 = 1_232u32;

    // Mix 1 (a, b)
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(13);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(8);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(13);
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(12);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(16);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(5);
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(3);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(10);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(15);

    // Mix 2 (x, a)
    x = x.wrapping_sub(a); x = x.wrapping_sub(hash); x ^= hash.wrapping_shr(13);
    a = a.wrapping_sub(hash); a = a.wrapping_sub(x); a ^= x.wrapping_shl(8);
    hash = hash.wrapping_sub(x); hash = hash.wrapping_sub(a); hash ^= a.wrapping_shr(13);
    x = x.wrapping_sub(a); x = x.wrapping_sub(hash); x ^= hash.wrapping_shr(12);
    a = a.wrapping_sub(hash); a = a.wrapping_sub(x); a ^= x.wrapping_shl(16);
    hash = hash.wrapping_sub(x); hash = hash.wrapping_sub(a); hash ^= a.wrapping_shr(5);
    x = x.wrapping_sub(a); x = x.wrapping_sub(hash); x ^= hash.wrapping_shr(3);
    a = a.wrapping_sub(hash); a = a.wrapping_sub(x); a ^= x.wrapping_shl(10);
    hash = hash.wrapping_sub(x); hash = hash.wrapping_sub(a); hash ^= a.wrapping_shr(15);

    // Mix 3 (b, y)
    b = b.wrapping_sub(y); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(13);
    y = y.wrapping_sub(hash); y = y.wrapping_sub(b); y ^= b.wrapping_shl(8);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(y); hash ^= y.wrapping_shr(13);
    b = b.wrapping_sub(y); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(12);
    y = y.wrapping_sub(hash); y = y.wrapping_sub(b); y ^= b.wrapping_shl(16);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(y); hash ^= y.wrapping_shr(5);
    b = b.wrapping_sub(y); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(3);
    y = y.wrapping_sub(hash); y = y.wrapping_sub(b); y ^= b.wrapping_shl(10);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(y); hash ^= y.wrapping_shr(15);

    hash
}

// Private helper function for rjenkins1 with 3 inputs.
fn rjenkins1_3(mut a: u32, mut b: u32, mut c: u32) -> u32 {
    let mut hash: u32 = 1_315_423_911u32 ^ a ^ b ^ c;
    let mut x: u32 = 231_232u32;
    let mut y: u32 = 1_232u32;

    // Mix 1 (a,b)
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(13);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(8);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(13);
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(12);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(16);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(5);
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(3);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(10);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(15);

    // Mix 2 (c,x)
    c = c.wrapping_sub(x); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(13);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(c); x ^= c.wrapping_shl(8);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(13);
    c = c.wrapping_sub(x); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(12);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(c); x ^= c.wrapping_shl(16);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(5);
    c = c.wrapping_sub(x); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(3);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(c); x ^= c.wrapping_shl(10);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(15);

    // Mix 3 (y,a)
    y = y.wrapping_sub(a); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(13);
    a = a.wrapping_sub(hash); a = a.wrapping_sub(y); a ^= y.wrapping_shl(8);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(a); hash ^= a.wrapping_shr(13);
    y = y.wrapping_sub(a); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(12);
    a = a.wrapping_sub(hash); a = a.wrapping_sub(y); a ^= y.wrapping_shl(16);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(a); hash ^= a.wrapping_shr(5);
    y = y.wrapping_sub(a); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(3);
    a = a.wrapping_sub(hash); a = a.wrapping_sub(y); a ^= y.wrapping_shl(10);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(a); hash ^= a.wrapping_shr(15);

    // Mix 4 (b,x)
    b = b.wrapping_sub(x); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(13);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(b); x ^= b.wrapping_shl(8);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(13);
    b = b.wrapping_sub(x); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(12);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(b); x ^= b.wrapping_shl(16);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(5);
    b = b.wrapping_sub(x); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(3);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(b); x ^= b.wrapping_shl(10);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(15);

    // Mix 5 (y,c)
    y = y.wrapping_sub(c); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(13);
    c = c.wrapping_sub(hash); c = c.wrapping_sub(y); c ^= y.wrapping_shl(8);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(c); hash ^= c.wrapping_shr(13);
    y = y.wrapping_sub(c); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(12);
    c = c.wrapping_sub(hash); c = c.wrapping_sub(y); c ^= y.wrapping_shl(16);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(c); hash ^= c.wrapping_shr(5);
    y = y.wrapping_sub(c); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(3);
    c = c.wrapping_sub(hash); c = c.wrapping_sub(y); c ^= y.wrapping_shl(10);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(c); hash ^= c.wrapping_shr(15);

    hash
}

// Private helper function for rjenkins1 with 4 inputs.
fn rjenkins1_4(mut a: u32, mut b: u32, mut c: u32, mut d: u32) -> u32 {
    let mut hash: u32 = 1_315_423_911u32 ^ a ^ b ^ c ^ d;
    let mut x: u32 = 231_232u32;
    let mut y: u32 = 1_232u32;

    // Mix 1 (a,b)
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(13);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(8);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(13);
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(12);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(16);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(5);
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(3);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(10);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(15);

    // Mix 2 (c,d)
    c = c.wrapping_sub(d); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(13);
    d = d.wrapping_sub(hash); d = d.wrapping_sub(c); d ^= c.wrapping_shl(8);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(d); hash ^= d.wrapping_shr(13);
    c = c.wrapping_sub(d); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(12);
    d = d.wrapping_sub(hash); d = d.wrapping_sub(c); d ^= c.wrapping_shl(16);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(d); hash ^= d.wrapping_shr(5);
    c = c.wrapping_sub(d); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(3);
    d = d.wrapping_sub(hash); d = d.wrapping_sub(c); d ^= c.wrapping_shl(10);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(d); hash ^= d.wrapping_shr(15);

    // Mix 3 (a,x)
    a = a.wrapping_sub(x); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(13);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(a); x ^= a.wrapping_shl(8);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(13);
    a = a.wrapping_sub(x); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(12);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(a); x ^= a.wrapping_shl(16);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(5);
    a = a.wrapping_sub(x); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(3);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(a); x ^= a.wrapping_shl(10);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(15);

    // Mix 4 (y,b)
    y = y.wrapping_sub(b); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(13);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(y); b ^= y.wrapping_shl(8);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(13);
    y = y.wrapping_sub(b); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(12);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(y); b ^= y.wrapping_shl(16);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(5);
    y = y.wrapping_sub(b); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(3);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(y); b ^= y.wrapping_shl(10);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(15);

    // Mix 5 (c,x)
    c = c.wrapping_sub(x); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(13);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(c); x ^= c.wrapping_shl(8);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(13);
    c = c.wrapping_sub(x); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(12);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(c); x ^= c.wrapping_shl(16);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(5);
    c = c.wrapping_sub(x); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(3);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(c); x ^= c.wrapping_shl(10);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(15);

    // Mix 6 (y,d)
    y = y.wrapping_sub(d); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(13);
    d = d.wrapping_sub(hash); d = d.wrapping_sub(y); d ^= y.wrapping_shl(8);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(d); hash ^= d.wrapping_shr(13);
    y = y.wrapping_sub(d); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(12);
    d = d.wrapping_sub(hash); d = d.wrapping_sub(y); d ^= y.wrapping_shl(16);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(d); hash ^= d.wrapping_shr(5);
    y = y.wrapping_sub(d); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(3);
    d = d.wrapping_sub(hash); d = d.wrapping_sub(y); d ^= y.wrapping_shl(10);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(d); hash ^= d.wrapping_shr(15);

    hash
}

// Private helper function for rjenkins1 with 5 inputs.
fn rjenkins1_5(mut a: u32, mut b: u32, mut c: u32, mut d: u32, mut e: u32) -> u32 {
    let mut hash: u32 = 1_315_423_911u32 ^ a ^ b ^ c ^ d ^ e;
    let mut x: u32 = 231_232u32;
    let mut y: u32 = 1_232u32;

    // Series of mixing operations, directly translated
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(13);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(8);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(13);
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(12);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(16);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(5);
    a = a.wrapping_sub(b); a = a.wrapping_sub(hash); a ^= hash.wrapping_shr(3);
    b = b.wrapping_sub(hash); b = b.wrapping_sub(a); b ^= a.wrapping_shl(10);
    hash = hash.wrapping_sub(a); hash = hash.wrapping_sub(b); hash ^= b.wrapping_shr(15);

    c = c.wrapping_sub(d); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(13);
    d = d.wrapping_sub(hash); d = d.wrapping_sub(c); d ^= c.wrapping_shl(8);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(d); hash ^= d.wrapping_shr(13);
    c = c.wrapping_sub(d); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(12);
    d = d.wrapping_sub(hash); d = d.wrapping_sub(c); d ^= c.wrapping_shl(16);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(d); hash ^= d.wrapping_shr(5);
    c = c.wrapping_sub(d); c = c.wrapping_sub(hash); c ^= hash.wrapping_shr(3);
    d = d.wrapping_sub(hash); d = d.wrapping_sub(c); d ^= c.wrapping_shl(10);
    hash = hash.wrapping_sub(c); hash = hash.wrapping_sub(d); hash ^= d.wrapping_shr(15);

    e = e.wrapping_sub(x); e = e.wrapping_sub(hash); e ^= hash.wrapping_shr(13);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(e); x ^= e.wrapping_shl(8);
    hash = hash.wrapping_sub(e); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(13);
    e = e.wrapping_sub(x); e = e.wrapping_sub(hash); e ^= hash.wrapping_shr(12);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(e); x ^= e.wrapping_shl(16);
    hash = hash.wrapping_sub(e); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(5);
    e = e.wrapping_sub(x); e = e.wrapping_sub(hash); e ^= hash.wrapping_shr(3);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(e); x ^= e.wrapping_shl(10);
    hash = hash.wrapping_sub(e); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(15);

    y = y.wrapping_sub(a); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(13);
    a = a.wrapping_sub(hash); a = a.wrapping_sub(y); a ^= y.wrapping_shl(8);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(a); hash ^= a.wrapping_shr(13);
    y = y.wrapping_sub(a); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(12);
    a = a.wrapping_sub(hash); a = a.wrapping_sub(y); a ^= y.wrapping_shl(16);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(a); hash ^= a.wrapping_shr(5);
    y = y.wrapping_sub(a); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(3);
    a = a.wrapping_sub(hash); a = a.wrapping_sub(y); a ^= y.wrapping_shl(10);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(a); hash ^= a.wrapping_shr(15);

    b = b.wrapping_sub(x); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(13);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(b); x ^= b.wrapping_shl(8);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(13);
    b = b.wrapping_sub(x); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(12);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(b); x ^= b.wrapping_shl(16);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(5);
    b = b.wrapping_sub(x); b = b.wrapping_sub(hash); b ^= hash.wrapping_shr(3);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(b); x ^= b.wrapping_shl(10);
    hash = hash.wrapping_sub(b); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(15);

    y = y.wrapping_sub(c); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(13);
    c = c.wrapping_sub(hash); c = c.wrapping_sub(y); c ^= y.wrapping_shl(8);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(c); hash ^= c.wrapping_shr(13);
    y = y.wrapping_sub(c); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(12);
    c = c.wrapping_sub(hash); c = c.wrapping_sub(y); c ^= y.wrapping_shl(16);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(c); hash ^= c.wrapping_shr(5);
    y = y.wrapping_sub(c); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(3);
    c = c.wrapping_sub(hash); c = c.wrapping_sub(y); c ^= y.wrapping_shl(10);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(c); hash ^= c.wrapping_shr(15);

    d = d.wrapping_sub(x); d = d.wrapping_sub(hash); d ^= hash.wrapping_shr(13);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(d); x ^= d.wrapping_shl(8);
    hash = hash.wrapping_sub(d); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(13);
    d = d.wrapping_sub(x); d = d.wrapping_sub(hash); d ^= hash.wrapping_shr(12);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(d); x ^= d.wrapping_shl(16);
    hash = hash.wrapping_sub(d); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(5);
    d = d.wrapping_sub(x); d = d.wrapping_sub(hash); d ^= hash.wrapping_shr(3);
    x = x.wrapping_sub(hash); x = x.wrapping_sub(d); x ^= d.wrapping_shl(10);
    hash = hash.wrapping_sub(d); hash = hash.wrapping_sub(x); hash ^= x.wrapping_shr(15);

    y = y.wrapping_sub(e); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(13);
    e = e.wrapping_sub(hash); e = e.wrapping_sub(y); e ^= y.wrapping_shl(8);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(e); hash ^= e.wrapping_shr(13);
    y = y.wrapping_sub(e); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(12);
    e = e.wrapping_sub(hash); e = e.wrapping_sub(y); e ^= y.wrapping_shl(16);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(e); hash ^= e.wrapping_shr(5);
    y = y.wrapping_sub(e); y = y.wrapping_sub(hash); y ^= hash.wrapping_shr(3);
    e = e.wrapping_sub(hash); e = e.wrapping_sub(y); e ^= y.wrapping_shl(10);
    hash = hash.wrapping_sub(y); hash = hash.wrapping_sub(e); hash ^= e.wrapping_shr(15);

    hash
}

/// Computes a hash value using the specified algorithm and one input.
/// This is the specific rjenkins1 version.
pub fn hash32_rjenkins1(a: u32) -> u32 {
    rjenkins1_direct_translation(a)
}

/// Computes a hash value using the specified algorithm and one input.
pub fn hash32(alg: HashAlgorithm, a: u32) -> u32 {
    match alg {
        HashAlgorithm::Rjenkins1 => rjenkins1_direct_translation(a),
    }
}

/// Computes a hash value using the specified algorithm and two inputs.
pub fn hash32_2(alg: HashAlgorithm, a: u32, b: u32) -> u32 {
    match alg {
        HashAlgorithm::Rjenkins1 => rjenkins1_2(a, b),
    }
}

/// Computes a hash value using the specified algorithm and three inputs.
pub fn hash32_3(alg: HashAlgorithm, a: u32, b: u32, c: u32) -> u32 {
    match alg {
        HashAlgorithm::Rjenkins1 => rjenkins1_3(a, b, c),
    }
}

/// Computes a hash value using the specified algorithm and four inputs.
pub fn hash32_4(alg: HashAlgorithm, a: u32, b: u32, c: u32, d: u32) -> u32 {
    match alg {
        HashAlgorithm::Rjenkins1 => rjenkins1_4(a, b, c, d),
    }
}

/// Computes a hash value using the specified algorithm and five inputs.
pub fn hash32_5(alg: HashAlgorithm, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    match alg {
        HashAlgorithm::Rjenkins1 => rjenkins1_5(a, b, c, d, e),
    }
}

pub fn crush_hash_name_from_i32(type_val: i32) -> &'static str {
    HashAlgorithm::from_i32(type_val)
        .map_or("unknown", |alg| alg.name())
}
