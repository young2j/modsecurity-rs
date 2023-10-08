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

pub struct VariableOrigin {
    pub m_length: i32,
    pub m_offset: usize,
}

impl VariableOrigin {
    pub fn new() -> VariableOrigin {
        VariableOrigin {
            m_length: 0,
            m_offset: 0,
        }
    }

    pub fn to_text(&self) -> String {
        let text = format!("v{},{}", self.m_offset, self.m_length);
        text
    }
}
