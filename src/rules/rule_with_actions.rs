// Copyright 2023 young2j
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

use std::rc::Rc;

use crate::transaction::Transaction;

use super::{Rule, RuleMessage};

pub struct RuleWithActions {}

impl Rule for RuleWithActions {
    fn evaluate(&self, transaction: &Transaction) -> bool {
        todo!()
    }

    fn evaluate_rule_message(&self, transaction: &Transaction, rm: Rc<RuleMessage>) -> bool {
        todo!()
    }

    fn get_file_name(&self) -> &str {
        todo!()
    }

    fn get_line_number(&self) -> i32 {
        todo!()
    }
}
