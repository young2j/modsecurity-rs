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

use rand::{rngs::StdRng, SeedableRng};

use super::enums::LogProperty;
use crate::{collection::Collection, rules::RuleMessage};
use std::{
    any::Any,
    env::consts,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

pub const MODSECURITY_MAJOR: &str = "3";
pub const MODSECURITY_MINOR: &str = "0";
pub const MODSECURITY_PATCHLEVEL: &str = "10";
pub const MODSECURITY_TAG: &str = "";
pub const MODSECURITY_TAG_NUM: i32 = 100;
pub const MODSECURITY_VERSION_NUM: i32 = 30100100;

pub fn modsecurity_version() -> String {
    format!(
        "{}.{}.{}",
        MODSECURITY_MAJOR, MODSECURITY_MINOR, MODSECURITY_PATCHLEVEL
    )
}

/*
 * The callback is going to be called on every log request.
 *
 *
 * void *   Internal reference to be used by the API consumer. Whatever
 *          is set here will be passed on every call.
 * void *   Pointer to a const char * or RuleMessage class. The returned
 *          data is selected on the log register property.
 *
 * @note    Vide LogProperty enum to learn more about Log Properties.
 *
 */
// typedef void (*ModSecLogCb) (void *, const void *);
// todo: logcb definition
type ModSecLogCb = fn();

pub struct RuleWithOperator {}

pub struct ModSecurity<C>
where
    C: Collection,
{
    pub m_global_collection: C,
    pub m_resource_collection: C,
    pub m_ip_collection: C,
    pub m_session_collection: C,
    pub m_user_collection: C,
    m_connector: String,
    m_whoami: String,
    m_logcb: ModSecLogCb,
    m_log_properties: LogProperty,
}

impl<C> ModSecurity<C>
where
    C: Collection,
{
    pub fn new() -> Self {
        // todo: uniqueId

        // set random seed
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Could not get system time")
            .as_secs();
        StdRng::seed_from_u64(timestamp);

        // todo: curl or xml init?
        // curl_global_init(CURL_GLOBAL_ALL);
        // xmlInitParser();

        // todo: to CamelCase
        let platform = consts::OS;
        let m_whoami = format!("ModSecurity v{} ({})", modsecurity_version(), platform);
        ModSecurity {
            m_global_collection: C::new("GLOBAL"),
            m_resource_collection: C::new("RESOURCE"),
            m_ip_collection: C::new("IP"),
            m_session_collection: C::new("SESSION"),
            m_user_collection: C::new("USER"),
            m_connector: String::new(),
            m_whoami,
            // todo: logcb init
            m_logcb: || {},
            m_log_properties: LogProperty::TextLogProperty,
        }
    }

    /// Return information about this ModSecurity version and platform
    ///
    /// Platform and version are two questions that community will ask prior to
    /// provide support. Making it available internally and to the connector as well.
    ///
    /// This information maybe will be used by a log parser. If you want to
    /// update it, make it in a fashion that won't break the existent parsers.
    ///   (e.g. adding extra information _only_ to the end of the string)
    ///
    pub fn who_am_i(&self) -> &str {
        &self.m_whoami
    }

    /// Set information about the connector that is using the library.
    ///
    /// For the purpose of log it is necessary for modsecurity to understand which
    /// 'connector' is consuming the API.
    ///
    /// It is strongly recommended to set a information in the following
    ///       pattern:
    ///
    ///       ConnectorName vX.Y.Z-tag (something else)
    ///       For instance: ModSecurity-nginx v0.0.1-alpha (Whee)
    ///
    pub fn set_connector_information(&mut self, connector: &str) {
        self.m_connector = String::from(connector);
    }

    /// Returns the connector information.
    ///
    /// Returns whatever was set by 'set_connector_information'. Check
    /// set_connector_information documentation to understand the expected format.
    ///
    pub fn get_connector_information(&self) -> &str {
        &self.m_connector
    }

    pub fn server_log(&self, data: &dyn Any, rm: Rc<RuleMessage>) {
        // todo: server log
    }

    pub fn set_server_logcb(&mut self, cb: ModSecLogCb) {
        self.set_server_logcb_with_properties(cb, LogProperty::TextLogProperty);
    }

    pub fn set_server_logcb_with_properties(&mut self, cb: ModSecLogCb, properties: LogProperty) {
        self.m_logcb = cb;
        self.m_log_properties = properties;
    }

    // todo: json
    pub fn process_content_offset(
        content: &str,
        len: usize,
        match_string: &str,
        json: String,
        err: Vec<&str>,
    ) -> i32 {
        // Implement the `process_content_offset` function here
        0
    }
}
