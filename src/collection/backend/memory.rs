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

use std::{
    collections::{HashMap, LinkedList},
    sync::Mutex,
};

use crate::{
    collection::Collection,
    variables::{KeyExclusions, VariableValue},
};

pub struct InMemoryPerProcess {
    m_name: String,
    collection: HashMap<String, LinkedList<String>>,
    lock: Mutex<()>,
}

impl Collection for InMemoryPerProcess {
    fn new(name: &str) -> InMemoryPerProcess {
        InMemoryPerProcess {
            m_name: name.to_string(),
            // todo: whether key is case insensitive?
            collection: HashMap::with_capacity(1000),
            lock: Mutex::new(()),
        }
    }

    fn store(&mut self, key: String, value: String) {
        if let Ok(_) = self.lock.try_lock() {
            self.collection
                .entry(key)
                .or_insert_with(LinkedList::new)
                .push_back(value);
        }
    }

    fn update_first(&mut self, key: &str, value: &str) -> bool {
        let mut ok = false;
        if let Ok(_) = self.lock.try_lock() {
            self.collection.entry(key.to_string()).and_modify(|ll| {
                if let Some(v) = ll.front_mut() {
                    *v = value.to_string();
                    ok = true;
                }
            });
        }

        ok
    }

    fn store_or_update_first(&mut self, key: &str, value: &str) -> bool {
        if !self.update_first(key, value) {
            self.store(key.to_string(), value.to_string());
        }

        return true;
    }

    fn del(&mut self, key: &str) {
        if let Ok(_) = self.lock.try_lock() {
            self.collection.remove(key);
        }
    }

    fn resolve_first(&self, key: &str) -> Option<&str> {
        match self.collection.get(key) {
            None => None,
            Some(ll) => ll.front().and_then(|first| Some(first.as_str())),
        }
    }

    fn resolve_single_match(&self, key: &str, mut l: Vec<VariableValue>) {
        self.collection.get(key).iter().for_each(|ll| {
            for v in ll.iter() {
                l.push(VariableValue::new_with_collection(&self.m_name, key, v));
            }
        });
    }

    fn resolve_multi_matches(&self, key: &str, mut l: Vec<VariableValue>, ke: KeyExclusions) {
        l.reserve(15);
        if key.len() == 0 {
            self.collection.iter().for_each(|(k, ll)| {
                if !ke.to_omit(k) {
                    ll.iter().for_each(|v| {
                        l.insert(0, VariableValue::new_with_collection(&self.m_name, k, v))
                    })
                }
            })
        } else {
            self.collection.get(key).iter().for_each(|ll| {
                if !ke.to_omit(key) {
                    ll.iter().for_each(|v| {
                        l.insert(0, VariableValue::new_with_collection(&self.m_name, key, v))
                    })
                }
            })
        }
    }

    fn resolve_regular_expression(
        &self,
        re_key: &str,
        mut l: Vec<VariableValue>,
        ke: KeyExclusions,
    ) {
        self.collection
            .iter()
            .filter(|(k, _)| !ke.to_omit(k))
            // todo: Regex
            // .filter(|(k, _)| regex_search(re_key))
            .for_each(|(k, ll)| {
                ll.iter().for_each(|v| {
                    l.insert(0, VariableValue::new_with_collection(&self.m_name, k, v))
                })
            })
    }
}
