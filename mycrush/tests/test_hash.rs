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

use mycrush::hash::{self, HashAlgorithm};

#[test]
fn test_rjenkins1_1_arg() {
    assert_eq!(
        hash::hash32_rjenkins1(123),
        3684466007,
        "hash32_rjenkins1(123)"
    );
    // For consistency with the public API, let's also test via the general dispatcher
    assert_eq!(
        hash::hash32(HashAlgorithm::Rjenkins1, 123),
        3684466007,
        "hash32(Rjenkins1, 123)"
    );
}

#[test]
fn test_rjenkins1_2_args() {
    assert_eq!(
        hash::hash32_2(HashAlgorithm::Rjenkins1, 123, 456),
        2468338131,
        "hash32_2(Rjenkins1, 123, 456)"
    );
    assert_eq!(
        hash::hash32_2(HashAlgorithm::Rjenkins1, 0, 0),
        430787817,
        "hash32_2(Rjenkins1, 0, 0)"
    );
    assert_eq!(
        hash::hash32_2(HashAlgorithm::Rjenkins1, 0xffffffff, 0xffffffff),
        2671514060,
        "hash32_2(Rjenkins1, u32::MAX, u32::MAX)"
    );
}

#[test]
fn test_rjenkins1_3_args() {
    assert_eq!(
        hash::hash32_3(HashAlgorithm::Rjenkins1, 123, 456, 789),
        2675329259,
        "hash32_3(Rjenkins1, 123, 456, 789)"
    );
    assert_eq!(
        hash::hash32_3(HashAlgorithm::Rjenkins1, 1, 2, 3),
        1935332395,
        "hash32_3(Rjenkins1, 1, 2, 3)"
    );
}

#[test]
fn test_rjenkins1_4_args() {
    assert_eq!(
        hash::hash32_4(HashAlgorithm::Rjenkins1, 1234, 10, 0, -7i32 as u32),
        2585201613,
        "hash32_4(Rjenkins1, 1234, 10, 0, -7 as u32)"
    );
    assert_eq!(
        hash::hash32_4(HashAlgorithm::Rjenkins1, 1234, 18, 1, -11i32 as u32),
        3008837707,
        "hash32_4(Rjenkins1, 1234, 18, 1, -11 as u32)"
    );
}

#[test]
fn test_rjenkins1_5_args() {
    assert_eq!(
        hash::hash32_5(HashAlgorithm::Rjenkins1, 1, 2, 3, 4, 5),
        1262657953,
        "hash32_5(Rjenkins1, 1, 2, 3, 4, 5)"
    );
}

#[test]
fn test_hash_name() {
    assert_eq!(hash::HashAlgorithm::Rjenkins1.name(), "rjenkins1");
    assert_eq!(hash::crush_hash_name_from_i32(0), "rjenkins1");
    assert_eq!(hash::crush_hash_name_from_i32(1), "unknown"); // Test unknown type
}

#[test]
fn test_hash_algorithm_from_i32() {
    assert_eq!(HashAlgorithm::from_i32(0), Some(HashAlgorithm::Rjenkins1));
    assert_eq!(HashAlgorithm::from_i32(1), None);
    assert_eq!(HashAlgorithm::from_i32(-1), None);
}
