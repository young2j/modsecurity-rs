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

use std::collections::{HashMap, LinkedList};

use crate::{
    transaction::Transaction,
    variables::{KeyExclusions, VariableOrigin, VariableValue},
};

pub struct AnchoredSetVariable<'a> {
    m_transaction: &'a Transaction,
    m_name: &'a str,
    m_set: HashMap<&'a str, LinkedList<VariableValue>>,
}

impl<'a> AnchoredSetVariable<'a> {
    pub fn new<'t: 'a>(t: &'t Transaction, name: &'a str) -> AnchoredSetVariable<'a> {
        AnchoredSetVariable {
            m_transaction: t,
            m_name: name,
            // todo: whether key is case insensitive?
            // todo: Whether concurrency safety is required?
            m_set: HashMap::with_capacity(1000),
        }
    }

    pub fn unset(&mut self) {
        self.m_set.clear()
    }

    pub fn set(&mut self, key: &'a str, value: &str, offset: usize) {
        let origin = VariableOrigin::new_with_props(value.len(), offset);
        let mut var = VariableValue::new_with_collection(self.m_name, key, value);
        var.add_origin(origin);

        self.m_set
            .entry(key)
            .or_insert_with(LinkedList::new)
            .push_back(var);
    }

    pub fn set_with_length(&mut self, key: &'a str, value: &str, offset: usize, length: usize) {
        let origin = VariableOrigin::new_with_props(length, offset);
        let mut var = VariableValue::new_with_collection(self.m_name, key, value);
        var.add_origin(origin);

        self.m_set
            .entry(key)
            .or_insert_with(LinkedList::new)
            .push_back(var);
    }

    pub fn resolve(&self, mut l: Vec<VariableValue>) {
        self.m_set.values().for_each(|ll| {
            ll.iter().for_each(|vv| {
                let new_vv = VariableValue::new_from(vv);
                l.insert(0, new_vv);
            })
        });
    }

    pub fn resolve_with_exclusions(&self, mut l: Vec<VariableValue>, ke: KeyExclusions) {
        self.m_set.iter().for_each(|(key, ll)| {
            if !ke.to_omit(key) {
                ll.iter().for_each(|vv| {
                    let new_vv = VariableValue::new_from(vv);
                    l.insert(0, new_vv);
                })
            } else {
                // todo: debug info
            }
        });
    }

    pub fn resolve_by_key(&self, key: &str, mut l: Vec<VariableValue>) {
        if let Some(ll) = self.m_set.get(key) {
            ll.iter().for_each(|vv| {
                let new_vv = VariableValue::new_from(vv);
                l.push(new_vv);
            })
        }
    }

    pub fn resolve_first(&self, key: &str) -> Option<&str> {
        match self.m_set.get(key) {
            None => None,
            Some(ll) => ll.front().and_then(|first| Some(first.get_value())),
        }
    }

    pub fn resolve_regular_expression(&self, regex: &str, mut l: Vec<VariableValue>) {
        self.m_set.iter().for_each(|(key, ll)| {
            // todo: Regex
            // if regex_search(regex, key) {
            ll.iter().for_each(|vv| {
                let new_vv = VariableValue::new_from(vv);
                l.insert(0, new_vv);
            })
            // }
        })
    }

    pub fn resolve_regular_expression_with_exlusions(
        &self,
        regex: &str,
        mut l: Vec<VariableValue>,
        ke: KeyExclusions,
    ) {
        self.m_set.iter().for_each(|(key, ll)| {
            // todo: Regex
            // if regex_search(regex, key) &&
            if !ke.to_omit(key) {
                ll.iter().for_each(|vv| {
                    let new_vv = VariableValue::new_from(vv);
                    l.insert(0, new_vv);
                })
            } else {
                // todo: debug info
            }
        })
    }
}
