#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __u32 = std::ffi::c_uint;
unsafe extern "C" fn crush_hash32_rjenkins1(mut a: __u32) -> __u32 {
    let mut hash: __u32 = 1315423911 as std::ffi::c_int as __u32 ^ a;
    let mut b: __u32 = a;
    let mut x: __u32 = 231232 as std::ffi::c_int as __u32;
    let mut y: __u32 = 1232 as std::ffi::c_int as __u32;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 13 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x = x ^ b << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 13 as std::ffi::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 12 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x = x ^ b << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 5 as std::ffi::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 3 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x = x ^ b << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 15 as std::ffi::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 13 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a = a ^ y << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 13 as std::ffi::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 12 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a = a ^ y << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 5 as std::ffi::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 3 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a = a ^ y << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 15 as std::ffi::c_int;
    return hash;
}
unsafe extern "C" fn crush_hash32_rjenkins1_2(mut a: __u32, mut b: __u32) -> __u32 {
    let mut hash: __u32 = 1315423911 as std::ffi::c_int as __u32 ^ a ^ b;
    let mut x: __u32 = 231232 as std::ffi::c_int as __u32;
    let mut y: __u32 = 1232 as std::ffi::c_int as __u32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 13 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 13 as std::ffi::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 12 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 5 as std::ffi::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 3 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 15 as std::ffi::c_int;
    x = x.wrapping_sub(a);
    x = x.wrapping_sub(hash);
    x = x ^ hash >> 13 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(x);
    a = a ^ x << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(x);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 13 as std::ffi::c_int;
    x = x.wrapping_sub(a);
    x = x.wrapping_sub(hash);
    x = x ^ hash >> 12 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(x);
    a = a ^ x << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(x);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 5 as std::ffi::c_int;
    x = x.wrapping_sub(a);
    x = x.wrapping_sub(hash);
    x = x ^ hash >> 3 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(x);
    a = a ^ x << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(x);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 15 as std::ffi::c_int;
    b = b.wrapping_sub(y);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 13 as std::ffi::c_int;
    y = y.wrapping_sub(hash);
    y = y.wrapping_sub(b);
    y = y ^ b << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(y);
    hash = hash ^ y >> 13 as std::ffi::c_int;
    b = b.wrapping_sub(y);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 12 as std::ffi::c_int;
    y = y.wrapping_sub(hash);
    y = y.wrapping_sub(b);
    y = y ^ b << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(y);
    hash = hash ^ y >> 5 as std::ffi::c_int;
    b = b.wrapping_sub(y);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 3 as std::ffi::c_int;
    y = y.wrapping_sub(hash);
    y = y.wrapping_sub(b);
    y = y ^ b << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(y);
    hash = hash ^ y >> 15 as std::ffi::c_int;
    return hash;
}
unsafe extern "C" fn crush_hash32_rjenkins1_3(
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
) -> __u32 {
    let mut hash: __u32 = 1315423911 as std::ffi::c_int as __u32 ^ a ^ b ^ c;
    let mut x: __u32 = 231232 as std::ffi::c_int as __u32;
    let mut y: __u32 = 1232 as std::ffi::c_int as __u32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 13 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 13 as std::ffi::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 12 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 5 as std::ffi::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 3 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 15 as std::ffi::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 13 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x = x ^ c << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 13 as std::ffi::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 12 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x = x ^ c << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 5 as std::ffi::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 3 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x = x ^ c << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 15 as std::ffi::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 13 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a = a ^ y << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 13 as std::ffi::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 12 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a = a ^ y << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 5 as std::ffi::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 3 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a = a ^ y << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 15 as std::ffi::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 13 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x = x ^ b << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 13 as std::ffi::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 12 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x = x ^ b << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 5 as std::ffi::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 3 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x = x ^ b << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 15 as std::ffi::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 13 as std::ffi::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c = c ^ y << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash = hash ^ c >> 13 as std::ffi::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 12 as std::ffi::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c = c ^ y << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash = hash ^ c >> 5 as std::ffi::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 3 as std::ffi::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c = c ^ y << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash = hash ^ c >> 15 as std::ffi::c_int;
    return hash;
}
unsafe extern "C" fn crush_hash32_rjenkins1_4(
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
    mut d: __u32,
) -> __u32 {
    let mut hash: __u32 = 1315423911 as std::ffi::c_int as __u32 ^ a ^ b ^ c ^ d;
    let mut x: __u32 = 231232 as std::ffi::c_int as __u32;
    let mut y: __u32 = 1232 as std::ffi::c_int as __u32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 13 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 13 as std::ffi::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 12 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 5 as std::ffi::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 3 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 15 as std::ffi::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 13 as std::ffi::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d = d ^ c << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash = hash ^ d >> 13 as std::ffi::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 12 as std::ffi::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d = d ^ c << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash = hash ^ d >> 5 as std::ffi::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 3 as std::ffi::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d = d ^ c << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash = hash ^ d >> 15 as std::ffi::c_int;
    a = a.wrapping_sub(x);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 13 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(a);
    x = x ^ a << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 13 as std::ffi::c_int;
    a = a.wrapping_sub(x);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 12 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(a);
    x = x ^ a << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 5 as std::ffi::c_int;
    a = a.wrapping_sub(x);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 3 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(a);
    x = x ^ a << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 15 as std::ffi::c_int;
    y = y.wrapping_sub(b);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 13 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(y);
    b = b ^ y << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 13 as std::ffi::c_int;
    y = y.wrapping_sub(b);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 12 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(y);
    b = b ^ y << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 5 as std::ffi::c_int;
    y = y.wrapping_sub(b);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 3 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(y);
    b = b ^ y << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 15 as std::ffi::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 13 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x = x ^ c << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 13 as std::ffi::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 12 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x = x ^ c << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 5 as std::ffi::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 3 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x = x ^ c << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 15 as std::ffi::c_int;
    y = y.wrapping_sub(d);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 13 as std::ffi::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(y);
    d = d ^ y << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(d);
    hash = hash ^ d >> 13 as std::ffi::c_int;
    y = y.wrapping_sub(d);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 12 as std::ffi::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(y);
    d = d ^ y << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(d);
    hash = hash ^ d >> 5 as std::ffi::c_int;
    y = y.wrapping_sub(d);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 3 as std::ffi::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(y);
    d = d ^ y << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(d);
    hash = hash ^ d >> 15 as std::ffi::c_int;
    return hash;
}
unsafe extern "C" fn crush_hash32_rjenkins1_5(
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
    mut d: __u32,
    mut e: __u32,
) -> __u32 {
    let mut hash: __u32 = 1315423911 as std::ffi::c_int as __u32 ^ a ^ b ^ c ^ d ^ e;
    let mut x: __u32 = 231232 as std::ffi::c_int as __u32;
    let mut y: __u32 = 1232 as std::ffi::c_int as __u32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 13 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 13 as std::ffi::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 12 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 5 as std::ffi::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a = a ^ hash >> 3 as std::ffi::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b = b ^ a << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash = hash ^ b >> 15 as std::ffi::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 13 as std::ffi::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d = d ^ c << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash = hash ^ d >> 13 as std::ffi::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 12 as std::ffi::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d = d ^ c << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash = hash ^ d >> 5 as std::ffi::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c = c ^ hash >> 3 as std::ffi::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d = d ^ c << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash = hash ^ d >> 15 as std::ffi::c_int;
    e = e.wrapping_sub(x);
    e = e.wrapping_sub(hash);
    e = e ^ hash >> 13 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(e);
    x = x ^ e << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(e);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 13 as std::ffi::c_int;
    e = e.wrapping_sub(x);
    e = e.wrapping_sub(hash);
    e = e ^ hash >> 12 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(e);
    x = x ^ e << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(e);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 5 as std::ffi::c_int;
    e = e.wrapping_sub(x);
    e = e.wrapping_sub(hash);
    e = e ^ hash >> 3 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(e);
    x = x ^ e << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(e);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 15 as std::ffi::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 13 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a = a ^ y << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 13 as std::ffi::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 12 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a = a ^ y << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 5 as std::ffi::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 3 as std::ffi::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a = a ^ y << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash = hash ^ a >> 15 as std::ffi::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 13 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x = x ^ b << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 13 as std::ffi::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 12 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x = x ^ b << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 5 as std::ffi::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b = b ^ hash >> 3 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x = x ^ b << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 15 as std::ffi::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 13 as std::ffi::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c = c ^ y << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash = hash ^ c >> 13 as std::ffi::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 12 as std::ffi::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c = c ^ y << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash = hash ^ c >> 5 as std::ffi::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 3 as std::ffi::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c = c ^ y << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash = hash ^ c >> 15 as std::ffi::c_int;
    d = d.wrapping_sub(x);
    d = d.wrapping_sub(hash);
    d = d ^ hash >> 13 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(d);
    x = x ^ d << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(d);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 13 as std::ffi::c_int;
    d = d.wrapping_sub(x);
    d = d.wrapping_sub(hash);
    d = d ^ hash >> 12 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(d);
    x = x ^ d << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(d);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 5 as std::ffi::c_int;
    d = d.wrapping_sub(x);
    d = d.wrapping_sub(hash);
    d = d ^ hash >> 3 as std::ffi::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(d);
    x = x ^ d << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(d);
    hash = hash.wrapping_sub(x);
    hash = hash ^ x >> 15 as std::ffi::c_int;
    y = y.wrapping_sub(e);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 13 as std::ffi::c_int;
    e = e.wrapping_sub(hash);
    e = e.wrapping_sub(y);
    e = e ^ y << 8 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(e);
    hash = hash ^ e >> 13 as std::ffi::c_int;
    y = y.wrapping_sub(e);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 12 as std::ffi::c_int;
    e = e.wrapping_sub(hash);
    e = e.wrapping_sub(y);
    e = e ^ y << 16 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(e);
    hash = hash ^ e >> 5 as std::ffi::c_int;
    y = y.wrapping_sub(e);
    y = y.wrapping_sub(hash);
    y = y ^ hash >> 3 as std::ffi::c_int;
    e = e.wrapping_sub(hash);
    e = e.wrapping_sub(y);
    e = e ^ y << 10 as std::ffi::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(e);
    hash = hash ^ e >> 15 as std::ffi::c_int;
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash32(
    mut type_0: std::ffi::c_int,
    mut a: __u32,
) -> __u32 {
    match type_0 {
        0 => return crush_hash32_rjenkins1(a),
        _ => return 0 as std::ffi::c_int as __u32,
    };
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_2(
    mut type_0: std::ffi::c_int,
    mut a: __u32,
    mut b: __u32,
) -> __u32 {
    match type_0 {
        0 => return crush_hash32_rjenkins1_2(a, b),
        _ => return 0 as std::ffi::c_int as __u32,
    };
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_3(
    mut type_0: std::ffi::c_int,
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
) -> __u32 {
    match type_0 {
        0 => return crush_hash32_rjenkins1_3(a, b, c),
        _ => return 0 as std::ffi::c_int as __u32,
    };
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_4(
    mut type_0: std::ffi::c_int,
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
    mut d: __u32,
) -> __u32 {
    match type_0 {
        0 => return crush_hash32_rjenkins1_4(a, b, c, d),
        _ => return 0 as std::ffi::c_int as __u32,
    };
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_5(
    mut type_0: std::ffi::c_int,
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
    mut d: __u32,
    mut e: __u32,
) -> __u32 {
    match type_0 {
        0 => return crush_hash32_rjenkins1_5(a, b, c, d, e),
        _ => return 0 as std::ffi::c_int as __u32,
    };
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash_name(
    mut type_0: std::ffi::c_int,
) -> *const std::ffi::c_char {
    match type_0 {
        0 => return b"rjenkins1\0" as *const u8 as *const std::ffi::c_char,
        _ => return b"unknown\0" as *const u8 as *const std::ffi::c_char,
    };
}
