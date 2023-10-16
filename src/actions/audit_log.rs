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

use crate::rules::{RuleMessage, RuleWithActions};
use crate::transaction::Transaction;

use super::action::{Action, BaseAction, Kind};

pub struct AuditLog {
    base: BaseAction,
    action_kind: Kind,
    m_is_none: bool,
    temporary_action: bool,
}

impl AuditLog {
    pub fn new(action: &str) -> AuditLog {
        AuditLog {
            base: BaseAction::new(action),
            action_kind: Kind::RunTimeOnlyIfMatchKind,
            m_is_none: false,
            temporary_action: false,
        }
    }
    pub fn new_kind(action: &str, kind: Kind) -> AuditLog {
        AuditLog {
            base: BaseAction::new(action),
            action_kind: kind,
            m_is_none: false,
            temporary_action: false,
        }
    }
}

impl Action for AuditLog {
    fn evaluate(&self, value: &str, transaction: &Transaction) -> String {
        self.base.evaluate(value, transaction)
    }

    fn evaluate_rule_actions(&self, ra: &RuleWithActions, transaction: &Transaction) -> bool {
        self.base.evaluate_rule_actions(ra, transaction)
    }

    fn evaluate_rule_actions_with_message(
        &self,
        ra: &RuleWithActions,
        transaction: &Transaction,
        rm: std::rc::Rc<RuleMessage>,
    ) -> bool {
        todo!()
    }
}
