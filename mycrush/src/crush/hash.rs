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
use ::libc;
unsafe extern "C" fn crush_hash32_rjenkins1(mut a: __u32) -> __u32 {
    let mut hash: __u32 = 1315423911 as libc::c_int as __u32 ^ a;
    let mut b: __u32 = a;
    let mut x: __u32 = 231232 as libc::c_int as __u32;
    let mut y: __u32 = 1232 as libc::c_int as __u32;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 13 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 8 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13 as libc::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 12 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 16 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5 as libc::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 3 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 10 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15 as libc::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 8 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 13 as libc::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 16 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 5 as libc::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 10 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 15 as libc::c_int;
    hash
}
unsafe extern "C" fn crush_hash32_rjenkins1_2(mut a: __u32, mut b: __u32) -> __u32 {
    let mut hash: __u32 = 1315423911 as libc::c_int as __u32 ^ a ^ b;
    let mut x: __u32 = 231232 as libc::c_int as __u32;
    let mut y: __u32 = 1232 as libc::c_int as __u32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 13 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 8 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 13 as libc::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 12 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 16 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 5 as libc::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 3 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 10 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 15 as libc::c_int;
    x = x.wrapping_sub(a);
    x = x.wrapping_sub(hash);
    x ^= hash >> 13 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(x);
    a ^= x << 8 as libc::c_int;
    hash = hash.wrapping_sub(x);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 13 as libc::c_int;
    x = x.wrapping_sub(a);
    x = x.wrapping_sub(hash);
    x ^= hash >> 12 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(x);
    a ^= x << 16 as libc::c_int;
    hash = hash.wrapping_sub(x);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 5 as libc::c_int;
    x = x.wrapping_sub(a);
    x = x.wrapping_sub(hash);
    x ^= hash >> 3 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(x);
    a ^= x << 10 as libc::c_int;
    hash = hash.wrapping_sub(x);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 15 as libc::c_int;
    b = b.wrapping_sub(y);
    b = b.wrapping_sub(hash);
    b ^= hash >> 13 as libc::c_int;
    y = y.wrapping_sub(hash);
    y = y.wrapping_sub(b);
    y ^= b << 8 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(y);
    hash ^= y >> 13 as libc::c_int;
    b = b.wrapping_sub(y);
    b = b.wrapping_sub(hash);
    b ^= hash >> 12 as libc::c_int;
    y = y.wrapping_sub(hash);
    y = y.wrapping_sub(b);
    y ^= b << 16 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(y);
    hash ^= y >> 5 as libc::c_int;
    b = b.wrapping_sub(y);
    b = b.wrapping_sub(hash);
    b ^= hash >> 3 as libc::c_int;
    y = y.wrapping_sub(hash);
    y = y.wrapping_sub(b);
    y ^= b << 10 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(y);
    hash ^= y >> 15 as libc::c_int;
    hash
}
unsafe extern "C" fn crush_hash32_rjenkins1_3(mut a: __u32, mut b: __u32, mut c: __u32) -> __u32 {
    let mut hash: __u32 = 1315423911 as libc::c_int as __u32 ^ a ^ b ^ c;
    let mut x: __u32 = 231232 as libc::c_int as __u32;
    let mut y: __u32 = 1232 as libc::c_int as __u32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 13 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 8 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 13 as libc::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 12 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 16 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 5 as libc::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 3 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 10 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 15 as libc::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 13 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 8 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13 as libc::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 12 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 16 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5 as libc::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 3 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 10 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15 as libc::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 8 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 13 as libc::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 16 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 5 as libc::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 10 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 15 as libc::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 13 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 8 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13 as libc::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 12 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 16 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5 as libc::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 3 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 10 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15 as libc::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13 as libc::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 8 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 13 as libc::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12 as libc::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 16 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 5 as libc::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3 as libc::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 10 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 15 as libc::c_int;
    hash
}
unsafe extern "C" fn crush_hash32_rjenkins1_4(
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
    mut d: __u32,
) -> __u32 {
    let mut hash: __u32 = 1315423911 as libc::c_int as __u32 ^ a ^ b ^ c ^ d;
    let mut x: __u32 = 231232 as libc::c_int as __u32;
    let mut y: __u32 = 1232 as libc::c_int as __u32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 13 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 8 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 13 as libc::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 12 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 16 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 5 as libc::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 3 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 10 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 15 as libc::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 13 as libc::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 8 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 13 as libc::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 12 as libc::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 16 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 5 as libc::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 3 as libc::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 10 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 15 as libc::c_int;
    a = a.wrapping_sub(x);
    a = a.wrapping_sub(hash);
    a ^= hash >> 13 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(a);
    x ^= a << 8 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13 as libc::c_int;
    a = a.wrapping_sub(x);
    a = a.wrapping_sub(hash);
    a ^= hash >> 12 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(a);
    x ^= a << 16 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5 as libc::c_int;
    a = a.wrapping_sub(x);
    a = a.wrapping_sub(hash);
    a ^= hash >> 3 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(a);
    x ^= a << 10 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15 as libc::c_int;
    y = y.wrapping_sub(b);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(y);
    b ^= y << 8 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 13 as libc::c_int;
    y = y.wrapping_sub(b);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(y);
    b ^= y << 16 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 5 as libc::c_int;
    y = y.wrapping_sub(b);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(y);
    b ^= y << 10 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 15 as libc::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 13 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 8 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13 as libc::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 12 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 16 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5 as libc::c_int;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 3 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 10 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15 as libc::c_int;
    y = y.wrapping_sub(d);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13 as libc::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(y);
    d ^= y << 8 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 13 as libc::c_int;
    y = y.wrapping_sub(d);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12 as libc::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(y);
    d ^= y << 16 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 5 as libc::c_int;
    y = y.wrapping_sub(d);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3 as libc::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(y);
    d ^= y << 10 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 15 as libc::c_int;
    hash
}
unsafe extern "C" fn crush_hash32_rjenkins1_5(
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
    mut d: __u32,
    mut e: __u32,
) -> __u32 {
    let mut hash: __u32 = 1315423911 as libc::c_int as __u32 ^ a ^ b ^ c ^ d ^ e;
    let mut x: __u32 = 231232 as libc::c_int as __u32;
    let mut y: __u32 = 1232 as libc::c_int as __u32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 13 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 8 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 13 as libc::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 12 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 16 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 5 as libc::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 3 as libc::c_int;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 10 as libc::c_int;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 15 as libc::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 13 as libc::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 8 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 13 as libc::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 12 as libc::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 16 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 5 as libc::c_int;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 3 as libc::c_int;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 10 as libc::c_int;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 15 as libc::c_int;
    e = e.wrapping_sub(x);
    e = e.wrapping_sub(hash);
    e ^= hash >> 13 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(e);
    x ^= e << 8 as libc::c_int;
    hash = hash.wrapping_sub(e);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13 as libc::c_int;
    e = e.wrapping_sub(x);
    e = e.wrapping_sub(hash);
    e ^= hash >> 12 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(e);
    x ^= e << 16 as libc::c_int;
    hash = hash.wrapping_sub(e);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5 as libc::c_int;
    e = e.wrapping_sub(x);
    e = e.wrapping_sub(hash);
    e ^= hash >> 3 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(e);
    x ^= e << 10 as libc::c_int;
    hash = hash.wrapping_sub(e);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15 as libc::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 8 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 13 as libc::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 16 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 5 as libc::c_int;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3 as libc::c_int;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 10 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 15 as libc::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 13 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 8 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13 as libc::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 12 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 16 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5 as libc::c_int;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 3 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 10 as libc::c_int;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15 as libc::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13 as libc::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 8 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 13 as libc::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12 as libc::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 16 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 5 as libc::c_int;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3 as libc::c_int;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 10 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 15 as libc::c_int;
    d = d.wrapping_sub(x);
    d = d.wrapping_sub(hash);
    d ^= hash >> 13 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(d);
    x ^= d << 8 as libc::c_int;
    hash = hash.wrapping_sub(d);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13 as libc::c_int;
    d = d.wrapping_sub(x);
    d = d.wrapping_sub(hash);
    d ^= hash >> 12 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(d);
    x ^= d << 16 as libc::c_int;
    hash = hash.wrapping_sub(d);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5 as libc::c_int;
    d = d.wrapping_sub(x);
    d = d.wrapping_sub(hash);
    d ^= hash >> 3 as libc::c_int;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(d);
    x ^= d << 10 as libc::c_int;
    hash = hash.wrapping_sub(d);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15 as libc::c_int;
    y = y.wrapping_sub(e);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13 as libc::c_int;
    e = e.wrapping_sub(hash);
    e = e.wrapping_sub(y);
    e ^= y << 8 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(e);
    hash ^= e >> 13 as libc::c_int;
    y = y.wrapping_sub(e);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12 as libc::c_int;
    e = e.wrapping_sub(hash);
    e = e.wrapping_sub(y);
    e ^= y << 16 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(e);
    hash ^= e >> 5 as libc::c_int;
    y = y.wrapping_sub(e);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3 as libc::c_int;
    e = e.wrapping_sub(hash);
    e = e.wrapping_sub(y);
    e ^= y << 10 as libc::c_int;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(e);
    hash ^= e >> 15 as libc::c_int;
    hash
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash32(mut type_0: libc::c_int, mut a: __u32) -> __u32 {
    match type_0 {
        0 => crush_hash32_rjenkins1(a),
        _ => 0 as libc::c_int as __u32,
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_2(
    mut type_0: libc::c_int,
    mut a: __u32,
    mut b: __u32,
) -> __u32 {
    match type_0 {
        0 => crush_hash32_rjenkins1_2(a, b),
        _ => 0 as libc::c_int as __u32,
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_3(
    mut type_0: libc::c_int,
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
) -> __u32 {
    match type_0 {
        0 => crush_hash32_rjenkins1_3(a, b, c),
        _ => 0 as libc::c_int as __u32,
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_4(
    mut type_0: libc::c_int,
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
    mut d: __u32,
) -> __u32 {
    match type_0 {
        0 => crush_hash32_rjenkins1_4(a, b, c, d),
        _ => 0 as libc::c_int as __u32,
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_5(
    mut type_0: libc::c_int,
    mut a: __u32,
    mut b: __u32,
    mut c: __u32,
    mut d: __u32,
    mut e: __u32,
) -> __u32 {
    match type_0 {
        0 => crush_hash32_rjenkins1_5(a, b, c, d, e),
        _ => 0 as libc::c_int as __u32,
    }
}
#[no_mangle]
pub unsafe extern "C" fn crush_hash_name(mut type_0: libc::c_int) -> *const libc::c_char {
    match type_0 {
        0 => b"rjenkins1\0" as *const u8 as *const libc::c_char,
        _ => b"unknown\0" as *const u8 as *const libc::c_char,
    }
}
