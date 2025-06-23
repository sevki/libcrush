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

use mycrush::types::{Rule, RuleMask, RuleOp, RuleStep};

#[test]
fn test_rule_op_from_u32() {
    assert_eq!(RuleOp::from_u32(0), Some(RuleOp::Noop));
    assert_eq!(RuleOp::from_u32(1), Some(RuleOp::Take));
    assert_eq!(RuleOp::from_u32(13), Some(RuleOp::SetChooseLeafStable));
    assert_eq!(RuleOp::from_u32(5), None); // Opcode 5 is not defined
    assert_eq!(RuleOp::from_u32(14), None); // Out of bounds
}

#[test]
fn test_rule_creation_new_with_len() {
    let rule = Rule::new_with_len(3, 0, 1, 2, 3);
    assert_eq!(rule.mask.ruleset, 0);
    assert_eq!(rule.mask.type_0, 1);
    assert_eq!(rule.mask.min_size, 2);
    assert_eq!(rule.mask.max_size, 3);
    assert_eq!(rule.len(), 3);
    assert!(!rule.is_empty());
    for step in &rule.steps {
        assert_eq!(step.op, RuleOp::Noop); // Default step
    }
}

#[test]
fn test_rule_set_step_at() {
    let mut rule = Rule::new_with_len(2, 0, 0, 1, 1);
    rule.set_step_at(0, RuleOp::Take, -1, 0);
    rule.set_step_at(1, RuleOp::Emit, 0, 0);

    assert_eq!(rule.steps[0], RuleStep::new(RuleOp::Take, -1, 0));
    assert_eq!(rule.steps[1], RuleStep::new(RuleOp::Emit, 0, 0));
}

#[test]
#[should_panic]
fn test_rule_set_step_at_out_of_bounds() {
    let mut rule = Rule::new_with_len(1, 0, 0, 1, 1);
    rule.set_step_at(1, RuleOp::Take, -1, 0); // Index 1 is out of bounds for len 1
}

#[test]
fn test_rule_creation_new_and_add_step() {
    let mut rule = Rule::new(0, 1, 2, 3, 4); // Start with capacity 0, effectively
    assert!(rule.is_empty());
    assert_eq!(rule.len(), 0);

    rule.add_step(RuleOp::Take, -1, 0);
    assert_eq!(rule.len(), 1);
    assert_eq!(rule.steps[0], RuleStep::new(RuleOp::Take, -1, 0));
    assert_eq!(rule.mask.ruleset, 1);
    assert_eq!(rule.mask.type_0, 2);
    assert_eq!(rule.mask.min_size, 3);
    assert_eq!(rule.mask.max_size, 4);

    rule.add_step(RuleOp::Emit, 0, 0);
    assert_eq!(rule.len(), 2);
    assert_eq!(rule.steps[1], RuleStep::new(RuleOp::Emit, 0, 0));
}

#[test]
fn test_rule_step_constructor() {
    let step = RuleStep::new(RuleOp::ChooseFirstN, 3, 2);
    assert_eq!(step.op, RuleOp::ChooseFirstN);
    assert_eq!(step.arg1, 3);
    assert_eq!(step.arg2, 2);
}
