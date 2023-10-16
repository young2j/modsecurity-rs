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

use crate::rules::{RuleMessage, RuleWithActions};
use crate::transaction::Transaction;

/**
 *
 * Define the action kind regarding to the execution time.
 *
 *
 */
pub enum Kind {
    /**
     *
     * Action that are executed while loading the configuration. For instance
     * the rule ID or the rule phase.
     *
     */
    ConfigurationKind,
    /**
     *
     * Those are actions that demands to be executed before call the operator.
     * For instance the tranformations.
     *
     *
     */
    RunTimeBeforeMatchAttemptKind,
    /**
     *
     * Actions that are executed after the execution of the operator, only if
     * the operator returned Match (or True). For instance the disruptive
     * actions.
     *
     */
    RunTimeOnlyIfMatchKind,
}

pub trait Action {
    fn init(&self, err: &str) -> bool {
        true
    }

    fn is_disruptive() -> bool {
        false
    }
    fn evaluate(&self, value: &str, transaction: &Transaction) -> String;

    fn evaluate_rule_actions(&self, ra: &RuleWithActions, transaction: &Transaction) -> bool;

    fn evaluate_rule_actions_with_message(
        &self,
        ra: &RuleWithActions,
        transaction: &Transaction,
        rm: Rc<RuleMessage>,
    ) -> bool {
        self.evaluate_rule_actions(ra, transaction)
    }
}

pub struct BaseAction {
    m_name: String,
    m_parser_payload: String,
}

impl BaseAction {
    pub fn new(act: &str) -> BaseAction {
        let mut base_action = BaseAction {
            m_name: String::new(),
            m_parser_payload: String::new(),
        };
        base_action.set_name_and_payload(act);

        base_action
    }

    pub fn set_name_and_payload(&mut self, data: &str) {
        let t = "t:";
        let dt = data.trim_start_matches(t);
        match dt.find(":") {
            None => self.m_name = data.to_string(),
            Some(i) => {
                let (m_name, mut m_payload) = data.split_at(i);
                m_payload = m_payload.trim_start_matches("'");
                m_payload = m_payload.get(0..m_payload.len() - 1).unwrap();

                self.m_name = m_name.to_string();
                self.m_parser_payload = m_payload.to_string();
            }
        };
    }
}

impl Action for BaseAction {
    fn evaluate(&self, value: &str, transaction: &Transaction) -> String {
        value.to_string()
    }

    fn evaluate_rule_actions(&self, ra: &RuleWithActions, transaction: &Transaction) -> bool {
        true
    }
}
