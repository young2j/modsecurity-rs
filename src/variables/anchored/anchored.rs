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

use crate::{
    transaction::Transaction,
    variables::{VariableOrigin, VariableValue},
};

pub struct AnchoredVariable<'a> {
    m_transaction: &'a Transaction,
    m_offset: usize,
    m_name: &'a str,
    m_value: String,
    m_var: VariableValue,
}

impl<'a> AnchoredVariable<'a> {
    pub fn new<'t: 'a>(t: &'t Transaction, name: &'a str) -> AnchoredVariable<'a> {
        AnchoredVariable {
            m_transaction: t,
            m_offset: 0,
            m_name: name,
            m_value: String::new(),
            m_var: VariableValue::new(name, None),
        }
    }

    pub fn unset(&mut self) {
        self.m_value.clear();
    }

    pub fn set(&mut self, value: &'a str, offset: usize) {
        self.m_value = value.to_string();
        self.m_offset = offset;

        let origin = VariableOrigin::new_with_props(value.len(), offset);
        self.m_var.add_origin(origin);
    }

    pub fn set_with_length(&mut self, value: &'a str, offset: usize, length: usize) {
        self.m_value = value.to_string();
        self.m_offset = offset;

        let origin = VariableOrigin::new_with_props(length, offset);
        self.m_var.add_origin(origin);
    }

    pub fn append(&mut self, value: &'a str, offset: usize, space_separator: bool) {
        if space_separator && value.len() > 0 {
            self.m_value.push(' ');
            self.m_value.push_str(value);
        } else {
            self.m_value.push_str(value);
        }
        self.m_offset = offset;

        let origin = VariableOrigin::new_with_props(value.len(), offset);
        self.m_var.add_origin(origin);
    }

    pub fn append_with_length(
        &mut self,
        value: &str,
        offset: usize,
        length: usize,
        space_separator: bool,
    ) {
        if space_separator && value.len() > 0 {
            self.m_value.push(' ');
            self.m_value.push_str(value);
        } else {
            self.m_value.push_str(value);
        }
        self.m_offset = offset;

        let origin = VariableOrigin::new_with_props(length, offset);
        self.m_var.add_origin(origin);
    }

    pub fn evaluate(&self) -> &str {
        &self.m_value
    }

    pub fn evaluate_variable_values(&mut self, mut l: Vec<VariableValue>) {
        if self.m_name.is_empty() {
            return;
        }
        self.m_var.set_value(&self.m_value);

        let new_var = VariableValue::new_from(&self.m_var);
        l.push(new_var);
    }

    pub fn resolve_first(&self) -> String {
        self.m_value.clone()
    }
}
