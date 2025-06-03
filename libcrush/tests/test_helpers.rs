use googletest::prelude::*;
use libcrush::*;

#[googletest::test]
fn test_crush_find_roots() {
    let mut m = CrushMap::new();

    // Empty map should have no roots
    let roots = m.find_roots().unwrap();
    assert!(roots.is_empty());

    // Add first bucket
    let first = m
        .make_bucket(BucketAlgorithm::Straw2, 0, 1, &[], &[])
        .unwrap();
    let first_bucketno = m.add_bucket(0, first).unwrap();

    // Add second bucket
    let second = m
        .make_bucket(BucketAlgorithm::Straw2, 0, 1, &[], &[])
        .unwrap();
    let second_bucketno = m.add_bucket(0, second).unwrap();

    // Should have two roots
    let roots = m.find_roots().unwrap();
    assert_that!(roots[0], eq(first_bucketno));
    assert_that!(roots[1], eq(second_bucketno));
    assert_that!(roots.len(), eq(2));

    // Add second bucket to first bucket (need to use raw API for bucket manipulation after adding)
    // This is a limitation of the current safe API - buckets can't be modified after being added to the map
    unsafe {
        let first_bucket = *(*m.ptr).buckets.offset(-1 - first_bucketno as isize);
        crush_sys::crush_bucket_add_item(m.ptr, first_bucket, second_bucketno, 0x1000);
    }

    // Should have only one root now
    let roots = m.find_roots().unwrap();
    assert_that!(roots[0], eq(first_bucketno));
    assert_that!(roots.len(), eq(1));

    // Add an invalid item to trigger error
    unsafe {
        let first_bucket = *(*m.ptr).buckets.offset(-1 - first_bucketno as isize);
        crush_sys::crush_bucket_add_item(m.ptr, first_bucket, -200, 0x1000);
    }

    // Should return error
    let result = m.find_roots();
    assert!(result.is_err());
    assert_that!(result.unwrap_err(), eq(-libc::EINVAL));
}
