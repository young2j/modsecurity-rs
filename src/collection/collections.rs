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

use super::{backend::InMemoryPerProcess, Collection};

pub struct Collections<C>
where
    C: Collection,
{
    pub m_global_collection_key: String,
    pub m_ip_collection_key: String,
    pub m_session_collection_key: String,
    pub m_user_collection_key: String,
    pub m_resource_collection_key: String,

    pub m_global_collection: C,
    pub m_ip_collection: C,
    pub m_session_collection: C,
    pub m_user_collection: C,
    pub m_resource_collection: C,
    pub m_tx_collection: InMemoryPerProcess,
}

impl<C> Collections<C>
where
    C: Collection,
{
    pub fn new(global: C, ip: C, session: C, user: C, resource: C) -> Collections<C> {
        Collections {
            m_global_collection_key: String::from(""),
            m_ip_collection_key: String::from(""),
            m_session_collection_key: String::from(""),
            m_user_collection_key: String::from(""),
            m_resource_collection_key: String::from(""),

            m_global_collection: global,
            m_ip_collection: ip,
            m_session_collection: session,
            m_user_collection: user,
            m_resource_collection: resource,

            m_tx_collection: InMemoryPerProcess::new("TX"),
        }
    }
}
