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
unsafe extern "C" fn crush_hash32_rjenkins1(mut a: U32) -> U32 {
    let mut hash: U32 = 1315423911 as U32 ^ a;
    let mut b: U32 = a;
    let mut x: U32 = 231232 as U32;
    let mut y: U32 = 1232 as U32;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 13;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 8;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 12;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 16;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 3;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 10;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 8;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 13;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 16;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 5;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 10;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 15;
    hash
}
unsafe extern "C" fn crush_hash32_rjenkins1_2(mut a: U32, mut b: U32) -> U32 {
    let mut hash: U32 = 1315423911 as U32 ^ a ^ b;
    let mut x: U32 = 231232 as U32;
    let mut y: U32 = 1232 as U32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 13;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 8;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 13;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 12;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 16;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 5;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 3;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 10;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 15;
    x = x.wrapping_sub(a);
    x = x.wrapping_sub(hash);
    x ^= hash >> 13;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(x);
    a ^= x << 8;
    hash = hash.wrapping_sub(x);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 13;
    x = x.wrapping_sub(a);
    x = x.wrapping_sub(hash);
    x ^= hash >> 12;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(x);
    a ^= x << 16;
    hash = hash.wrapping_sub(x);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 5;
    x = x.wrapping_sub(a);
    x = x.wrapping_sub(hash);
    x ^= hash >> 3;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(x);
    a ^= x << 10;
    hash = hash.wrapping_sub(x);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 15;
    b = b.wrapping_sub(y);
    b = b.wrapping_sub(hash);
    b ^= hash >> 13;
    y = y.wrapping_sub(hash);
    y = y.wrapping_sub(b);
    y ^= b << 8;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(y);
    hash ^= y >> 13;
    b = b.wrapping_sub(y);
    b = b.wrapping_sub(hash);
    b ^= hash >> 12;
    y = y.wrapping_sub(hash);
    y = y.wrapping_sub(b);
    y ^= b << 16;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(y);
    hash ^= y >> 5;
    b = b.wrapping_sub(y);
    b = b.wrapping_sub(hash);
    b ^= hash >> 3;
    y = y.wrapping_sub(hash);
    y = y.wrapping_sub(b);
    y ^= b << 10;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(y);
    hash ^= y >> 15;
    hash
}
unsafe extern "C" fn crush_hash32_rjenkins1_3(mut a: U32, mut b: U32, mut c: U32) -> U32 {
    let mut hash: U32 = 1315423911 as U32 ^ a ^ b ^ c;
    let mut x: U32 = 231232 as U32;
    let mut y: U32 = 1232 as U32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 13;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 8;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 13;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 12;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 16;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 5;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 3;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 10;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 15;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 13;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 8;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 12;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 16;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 3;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 10;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 8;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 13;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 16;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 5;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 10;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 15;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 13;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 8;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 12;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 16;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 3;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 10;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 8;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 13;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 16;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 5;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 10;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 15;
    hash
}
unsafe extern "C" fn crush_hash32_rjenkins1_4(
    mut a: U32,
    mut b: U32,
    mut c: U32,
    mut d: U32,
) -> U32 {
    let mut hash: U32 = 1315423911 as U32 ^ a ^ b ^ c ^ d;
    let mut x: U32 = 231232 as U32;
    let mut y: U32 = 1232 as U32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 13;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 8;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 13;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 12;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 16;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 5;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 3;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 10;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 15;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 13;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 8;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 13;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 12;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 16;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 5;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 3;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 10;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 15;
    a = a.wrapping_sub(x);
    a = a.wrapping_sub(hash);
    a ^= hash >> 13;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(a);
    x ^= a << 8;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13;
    a = a.wrapping_sub(x);
    a = a.wrapping_sub(hash);
    a ^= hash >> 12;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(a);
    x ^= a << 16;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5;
    a = a.wrapping_sub(x);
    a = a.wrapping_sub(hash);
    a ^= hash >> 3;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(a);
    x ^= a << 10;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15;
    y = y.wrapping_sub(b);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(y);
    b ^= y << 8;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 13;
    y = y.wrapping_sub(b);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(y);
    b ^= y << 16;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 5;
    y = y.wrapping_sub(b);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(y);
    b ^= y << 10;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 15;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 13;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 8;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 12;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 16;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5;
    c = c.wrapping_sub(x);
    c = c.wrapping_sub(hash);
    c ^= hash >> 3;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(c);
    x ^= c << 10;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15;
    y = y.wrapping_sub(d);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(y);
    d ^= y << 8;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 13;
    y = y.wrapping_sub(d);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(y);
    d ^= y << 16;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 5;
    y = y.wrapping_sub(d);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(y);
    d ^= y << 10;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 15;
    hash
}
unsafe extern "C" fn crush_hash32_rjenkins1_5(
    mut a: U32,
    mut b: U32,
    mut c: U32,
    mut d: U32,
    mut e: U32,
) -> U32 {
    let mut hash: U32 = 1315423911 as U32 ^ a ^ b ^ c ^ d ^ e;
    let mut x: U32 = 231232 as U32;
    let mut y: U32 = 1232 as U32;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 13;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 8;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 13;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 12;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 16;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 5;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(hash);
    a ^= hash >> 3;
    b = b.wrapping_sub(hash);
    b = b.wrapping_sub(a);
    b ^= a << 10;
    hash = hash.wrapping_sub(a);
    hash = hash.wrapping_sub(b);
    hash ^= b >> 15;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 13;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 8;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 13;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 12;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 16;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 5;
    c = c.wrapping_sub(d);
    c = c.wrapping_sub(hash);
    c ^= hash >> 3;
    d = d.wrapping_sub(hash);
    d = d.wrapping_sub(c);
    d ^= c << 10;
    hash = hash.wrapping_sub(c);
    hash = hash.wrapping_sub(d);
    hash ^= d >> 15;
    e = e.wrapping_sub(x);
    e = e.wrapping_sub(hash);
    e ^= hash >> 13;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(e);
    x ^= e << 8;
    hash = hash.wrapping_sub(e);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13;
    e = e.wrapping_sub(x);
    e = e.wrapping_sub(hash);
    e ^= hash >> 12;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(e);
    x ^= e << 16;
    hash = hash.wrapping_sub(e);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5;
    e = e.wrapping_sub(x);
    e = e.wrapping_sub(hash);
    e ^= hash >> 3;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(e);
    x ^= e << 10;
    hash = hash.wrapping_sub(e);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 8;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 13;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 16;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 5;
    y = y.wrapping_sub(a);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3;
    a = a.wrapping_sub(hash);
    a = a.wrapping_sub(y);
    a ^= y << 10;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(a);
    hash ^= a >> 15;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 13;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 8;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 12;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 16;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5;
    b = b.wrapping_sub(x);
    b = b.wrapping_sub(hash);
    b ^= hash >> 3;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(b);
    x ^= b << 10;
    hash = hash.wrapping_sub(b);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 8;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 13;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 16;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 5;
    y = y.wrapping_sub(c);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3;
    c = c.wrapping_sub(hash);
    c = c.wrapping_sub(y);
    c ^= y << 10;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(c);
    hash ^= c >> 15;
    d = d.wrapping_sub(x);
    d = d.wrapping_sub(hash);
    d ^= hash >> 13;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(d);
    x ^= d << 8;
    hash = hash.wrapping_sub(d);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 13;
    d = d.wrapping_sub(x);
    d = d.wrapping_sub(hash);
    d ^= hash >> 12;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(d);
    x ^= d << 16;
    hash = hash.wrapping_sub(d);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 5;
    d = d.wrapping_sub(x);
    d = d.wrapping_sub(hash);
    d ^= hash >> 3;
    x = x.wrapping_sub(hash);
    x = x.wrapping_sub(d);
    x ^= d << 10;
    hash = hash.wrapping_sub(d);
    hash = hash.wrapping_sub(x);
    hash ^= x >> 15;
    y = y.wrapping_sub(e);
    y = y.wrapping_sub(hash);
    y ^= hash >> 13;
    e = e.wrapping_sub(hash);
    e = e.wrapping_sub(y);
    e ^= y << 8;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(e);
    hash ^= e >> 13;
    y = y.wrapping_sub(e);
    y = y.wrapping_sub(hash);
    y ^= hash >> 12;
    e = e.wrapping_sub(hash);
    e = e.wrapping_sub(y);
    e ^= y << 16;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(e);
    hash ^= e >> 5;
    y = y.wrapping_sub(e);
    y = y.wrapping_sub(hash);
    y ^= hash >> 3;
    e = e.wrapping_sub(hash);
    e = e.wrapping_sub(y);
    e ^= y << 10;
    hash = hash.wrapping_sub(y);
    hash = hash.wrapping_sub(e);
    hash ^= e >> 15;
    hash
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_hash32(type_0: libc::c_int, a: U32) -> U32 {
    match type_0 {
        0 => crush_hash32_rjenkins1(a),
        _ => 0,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_hash32_2(type_0: libc::c_int, a: U32, b: U32) -> U32 {
    match type_0 {
        0 => crush_hash32_rjenkins1_2(a, b),
        _ => 0,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_hash32_3(
    type_0: libc::c_int,
    a: U32,
    b: U32,
    c: U32,
) -> U32 {
    match type_0 {
        0 => crush_hash32_rjenkins1_3(a, b, c),
        _ => 0,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_hash32_4(
    type_0: libc::c_int,
    a: U32,
    b: U32,
    c: U32,
    d: U32,
) -> U32 {
    match type_0 {
        0 => crush_hash32_rjenkins1_4(a, b, c, d),
        _ => 0,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_hash32_5(
    type_0: libc::c_int,
    a: U32,
    b: U32,
    c: U32,
    d: U32,
    e: U32,
) -> U32 {
    match type_0 {
        0 => crush_hash32_rjenkins1_5(a, b, c, d, e),
        _ => 0,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crush_hash_name(type_0: libc::c_int) -> *const libc::c_char {
    match type_0 {
        0 => b"rjenkins1\0" as *const u8 as *const libc::c_char,
        _ => b"unknown\0" as *const u8 as *const libc::c_char,
    }
}
