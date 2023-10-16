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

use crate::variables::VariableValue;

use super::AnchoredSetVariable;

pub struct AnchoredSetVariableTranslationProxy<'a> {
    m_name: &'a str,
    m_fount: Rc<AnchoredSetVariable<'a>>,
    m_translate: fn(&'a str, Vec<VariableValue>),
}

impl<'a> AnchoredSetVariableTranslationProxy<'a> {
    pub fn new(
        name: &'a str,
        fount: Rc<AnchoredSetVariable<'a>>,
    ) -> AnchoredSetVariableTranslationProxy<'a> {
        let m_translate = |name: &str, mut l: Vec<VariableValue>| {};
        AnchoredSetVariableTranslationProxy {
            m_name: name,
            m_fount: fount,
            m_translate,
        }
    }
}
