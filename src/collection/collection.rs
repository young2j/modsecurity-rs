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

use crate::variables::{KeyExclusions, VariableValue};

pub trait Collection {
    fn new(name: &str) -> Self;
    fn store(&mut self, key: String, value: String);
    fn update_first(&mut self, key: &str, value: &str) -> bool;
    fn store_or_update_first(&mut self, key: &str, value: &str) -> bool;
    fn del(&mut self, key: &str);
    fn resolve_first(&self, key: &str) -> Option<&str>;
    fn resolve_single_match(&self, key: &str, l: Vec<VariableValue>);
    fn resolve_multi_matches(&self, key: &str, l: Vec<VariableValue>, ke: KeyExclusions);
    fn resolve_regular_expression(&self, re_key: &str, l: Vec<VariableValue>, ke: KeyExclusions);

    /* store */
    fn store_with_compartment(&mut self, key: String, value: String, compartment: String) {
        let nkey = format!("{compartment}::{key}");
        self.store(nkey, value);
    }

    fn store_with_compartments(
        &mut self,
        key: String,
        value: String,
        compartment: String,
        compartment2: String,
    ) {
        let nkey = format!("{compartment}::{compartment2}::{key}");
        self.store(nkey, value);
    }

    /* storeOrUpdateFirst */
    fn store_or_update_first_with_compartment(
        &mut self,
        key: String,
        value: String,
        compartment: String,
    ) -> bool {
        let nkey = format!("{compartment}::{key}");
        return self.store_or_update_first(&nkey, &value);
    }

    fn store_or_update_first_with_compartments(
        &mut self,
        key: String,
        value: String,
        compartment: String,
        compartment2: String,
    ) -> bool {
        let nkey = format!("{compartment}::{compartment2}::{key}");
        return self.store_or_update_first(&nkey, &value);
    }

    /* updateFirst */
    fn update_first_with_compartment(
        &mut self,
        key: String,
        value: String,
        compartment: String,
    ) -> bool {
        let nkey = format!("{compartment}::{key}");
        return self.update_first(&nkey, &value);
    }

    fn update_first_with_compartments(
        &mut self,
        key: String,
        value: String,
        compartment: String,
        compartment2: String,
    ) -> bool {
        let nkey = format!("{compartment}::{compartment2}::{key}");
        return self.update_first(&nkey, &value);
    }

    /* del */
    fn del_with_compartment(&mut self, key: String, compartment: String) {
        let nkey = format!("{compartment}::{key}");
        self.del(&nkey);
    }

    fn del_with_compartments(&mut self, key: String, compartment: String, compartment2: String) {
        let nkey = format!("{compartment}::{compartment2}::{key}");
        self.del(&nkey);
    }

    /* resolveFirst */
    fn resolve_first_with_compartment(&self, key: String, compartment: String) -> Option<&str> {
        let nkey = format!("{compartment}::{key}");
        return self.resolve_first(&nkey);
    }

    fn resolve_first_with_compartments(
        &self,
        key: String,
        compartment: String,
        compartment2: String,
    ) -> Option<&str> {
        let nkey = format!("{compartment}::{compartment2}::{key}");
        return self.resolve_first(&nkey);
    }

    /* resolveSingleMatch */
    fn resolve_single_match_with_compartment(
        &self,
        key: String,
        compartment: String,
        l: Vec<VariableValue>,
    ) {
        let nkey = format!("{compartment}::{key}");
        self.resolve_single_match(&nkey, l);
    }

    fn resolve_single_match_with_compartments(
        &self,
        key: String,
        compartment: String,
        compartment2: String,
        l: Vec<VariableValue>,
    ) {
        let nkey = format!("{compartment}::{compartment2}::{key}");
        self.resolve_single_match(&nkey, l);
    }

    /* resolveMultiMatches */
    fn resolve_multi_matches_with_compartment(
        &self,
        key: String,
        compartment: String,
        l: Vec<VariableValue>,
        ke: KeyExclusions,
    ) {
        let nkey = format!("{compartment}::{key}");
        self.resolve_multi_matches(&nkey, l, ke);
    }

    fn resolve_multi_matches_with_compartments(
        &self,
        key: String,
        compartment: String,
        compartment2: String,
        l: Vec<VariableValue>,
        ke: KeyExclusions,
    ) {
        let nkey = format!("{compartment}::{compartment2}::{key}");
        self.resolve_multi_matches(&nkey, l, ke)
    }

    /* resolveRegularExpression */
    fn resolve_regular_expression_with_compartment(
        &self,
        key: String,
        compartment: String,
        l: Vec<VariableValue>,
        ke: KeyExclusions,
    ) {
        let nkey = format!("{compartment}::{key}");
        self.resolve_regular_expression(&nkey, l, ke);
    }

    fn resolve_regular_expression_with_compartments(
        &self,
        key: String,
        compartment: String,
        compartment2: String,
        l: Vec<VariableValue>,
        ke: KeyExclusions,
    ) {
        let nkey = format!("{compartment}::{compartment2}::{key}");
        self.resolve_regular_expression(&nkey, l, ke);
    }
}
