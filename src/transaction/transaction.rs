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

use crate::variables::anchored::{
    AnchoredSetVariable, AnchoredSetVariableTranslationProxy, AnchoredVariable,
};

pub struct Transaction {}

pub struct TransactionAnchoredVariables<'t> {
    m_variable_offset: i32,

    m_variable_args_names: AnchoredSetVariableTranslationProxy<'t>,
    m_variable_args_get_names: AnchoredSetVariableTranslationProxy<'t>,
    m_variable_args_post_names: AnchoredSetVariableTranslationProxy<'t>,

    m_variable_response_content_type: AnchoredVariable<'t>,
    m_variable_arg_scombined_size: AnchoredVariable<'t>,
    m_variable_auth_type: AnchoredVariable<'t>,
    m_variable_files_combined_size: AnchoredVariable<'t>,
    m_variable_full_request: AnchoredVariable<'t>,
    m_variable_full_request_length: AnchoredVariable<'t>,
    m_variable_inbound_data_error: AnchoredVariable<'t>,
    m_variable_matched_var: AnchoredVariable<'t>,
    m_variable_matched_var_name: AnchoredVariable<'t>,
    m_variable_msc_pcre_error: AnchoredVariable<'t>,
    m_variable_msc_pcre_limits_exceeded: AnchoredVariable<'t>,
    m_variable_multipart_boundary_quoted: AnchoredVariable<'t>,
    m_variable_multipart_boundary_white_space: AnchoredVariable<'t>,
    m_variable_multipart_crlf_lf_lines: AnchoredVariable<'t>,
    m_variable_multipart_data_after: AnchoredVariable<'t>,
    m_variable_multipart_data_before: AnchoredVariable<'t>,
    m_variable_multipart_file_limit_exceeded: AnchoredVariable<'t>,
    m_variable_multipart_header_folding: AnchoredVariable<'t>,
    m_variable_multipart_invalid_header_folding: AnchoredVariable<'t>,
    m_variable_multipart_invalid_part: AnchoredVariable<'t>,
    m_variable_multipart_invalid_quoting: AnchoredVariable<'t>,
    m_variable_multipart_lf_line: AnchoredVariable<'t>,
    m_variable_multipart_missing_semicolon: AnchoredVariable<'t>,
    m_variable_multipart_strict_error: AnchoredVariable<'t>,
    m_variable_multipart_unmatched_boundary: AnchoredVariable<'t>,
    m_variable_outbound_data_error: AnchoredVariable<'t>,
    m_variable_path_info: AnchoredVariable<'t>,
    m_variable_query_string: AnchoredVariable<'t>,
    m_variable_remote_addr: AnchoredVariable<'t>,
    m_variable_remote_host: AnchoredVariable<'t>,
    m_variable_remote_port: AnchoredVariable<'t>,
    m_variable_reqbody_error: AnchoredVariable<'t>,
    m_variable_reqbody_error_msg: AnchoredVariable<'t>,
    m_variable_reqbody_processor_error: AnchoredVariable<'t>,
    m_variable_reqbody_processor_error_msg: AnchoredVariable<'t>,
    m_variable_reqbody_processor: AnchoredVariable<'t>,
    m_variable_request_basename: AnchoredVariable<'t>,
    m_variable_request_body: AnchoredVariable<'t>,
    m_variable_request_body_length: AnchoredVariable<'t>,
    m_variable_request_filename: AnchoredVariable<'t>,
    m_variable_request_line: AnchoredVariable<'t>,
    m_variable_request_method: AnchoredVariable<'t>,
    m_variable_request_protocol: AnchoredVariable<'t>,
    m_variable_request_uri: AnchoredVariable<'t>,
    m_variable_request_uri_raw: AnchoredVariable<'t>,
    m_variable_resource: AnchoredVariable<'t>,
    m_variable_response_body: AnchoredVariable<'t>,
    m_variable_response_content_length: AnchoredVariable<'t>,
    m_variable_response_protocol: AnchoredVariable<'t>,
    m_variable_response_status: AnchoredVariable<'t>,
    m_variable_server_addr: AnchoredVariable<'t>,
    m_variable_server_name: AnchoredVariable<'t>,
    m_variable_server_port: AnchoredVariable<'t>,
    m_variable_session_id: AnchoredVariable<'t>,
    m_variable_unique_id: AnchoredVariable<'t>,
    m_variable_url_encoded_error: AnchoredVariable<'t>,
    m_variable_user_id: AnchoredVariable<'t>,

    m_variable_args: Rc<AnchoredSetVariable<'t>>,
    m_variable_args_get: Rc<AnchoredSetVariable<'t>>,
    m_variable_args_post: Rc<AnchoredSetVariable<'t>>,
    m_variable_request_headers_names: AnchoredSetVariable<'t>,
    m_variable_response_headers_names: AnchoredSetVariable<'t>,
    m_variable_files_sizes: AnchoredSetVariable<'t>,
    m_variable_files_names: AnchoredSetVariable<'t>,
    m_variable_files_tmp_content: AnchoredSetVariable<'t>,
    m_variable_multipart_file_name: AnchoredSetVariable<'t>,
    m_variable_multipart_name: AnchoredSetVariable<'t>,
    m_variable_matched_vars_names: AnchoredSetVariable<'t>,
    m_variable_matched_vars: AnchoredSetVariable<'t>,
    m_variable_files: AnchoredSetVariable<'t>,
    m_variable_request_cookies: AnchoredSetVariable<'t>,
    m_variable_request_headers: AnchoredSetVariable<'t>,
    m_variable_response_headers: AnchoredSetVariable<'t>,
    m_variable_geo: AnchoredSetVariable<'t>,
    m_variable_request_cookies_names: AnchoredSetVariable<'t>,
    m_variable_files_tmp_names: AnchoredSetVariable<'t>,
    m_variable_multipart_part_headers: AnchoredSetVariable<'t>,
}

impl<'t> TransactionAnchoredVariables<'t> {
    fn new(t: &Transaction) -> TransactionAnchoredVariables {
        let m_variable_args = Rc::new(AnchoredSetVariable::new(t, "ARGS"));
        let m_variable_args_get = Rc::new(AnchoredSetVariable::new(t, "ARGS_GET"));
        let m_variable_args_post = Rc::new(AnchoredSetVariable::new(t, "ARGS_POST"));

        TransactionAnchoredVariables {
            m_variable_offset: 0,
            m_variable_args_names: AnchoredSetVariableTranslationProxy::new(
                "ARGS_NAMES",
                m_variable_args.clone(),
            ),
            m_variable_args_get_names: AnchoredSetVariableTranslationProxy::new(
                "ARGS_GET_NAMES",
                m_variable_args_get.clone(),
            ),
            m_variable_args_post_names: AnchoredSetVariableTranslationProxy::new(
                "ARGS_POST_NAMES",
                m_variable_args_post.clone(),
            ),

            m_variable_response_content_type: AnchoredVariable::new(t, "RESPONSE_CONTENT_TYPE"),
            m_variable_arg_scombined_size: AnchoredVariable::new(t, "ARGS_COMBINED_SIZE"),
            m_variable_auth_type: AnchoredVariable::new(t, "AUTH_TYPE"),
            m_variable_files_combined_size: AnchoredVariable::new(t, "FILES_COMBINED_SIZE"),
            m_variable_full_request: AnchoredVariable::new(t, "FULL_REQUEST"),
            m_variable_full_request_length: AnchoredVariable::new(t, "FULL_REQUEST_LENGTH"),
            m_variable_inbound_data_error: AnchoredVariable::new(t, "INBOUND_DATA_ERROR"),
            m_variable_matched_var: AnchoredVariable::new(t, "MATCHED_VAR"),
            m_variable_matched_var_name: AnchoredVariable::new(t, "MATCHED_VAR_NAME"),
            m_variable_msc_pcre_error: AnchoredVariable::new(t, "MSC_PCRE_ERROR"),
            m_variable_msc_pcre_limits_exceeded: AnchoredVariable::new(
                t,
                "MSC_PCRE_LIMITS_EXCEEDED",
            ),
            m_variable_multipart_boundary_quoted: AnchoredVariable::new(
                t,
                "MULTIPART_BOUNDARY_QUOTED",
            ),
            m_variable_multipart_boundary_white_space: AnchoredVariable::new(
                t,
                "MULTIPART_BOUNDARY_WHITESPACE",
            ),
            m_variable_multipart_crlf_lf_lines: AnchoredVariable::new(t, "MULTIPART_CRLF_LF_LINES"),
            m_variable_multipart_data_after: AnchoredVariable::new(t, "MULTIPART_DATA_AFTER"),
            m_variable_multipart_data_before: AnchoredVariable::new(t, "MULTIPART_DATA_BEFORE"),
            m_variable_multipart_file_limit_exceeded: AnchoredVariable::new(
                t,
                "MULTIPART_FILE_LIMIT_EXCEEDED",
            ),
            m_variable_multipart_header_folding: AnchoredVariable::new(
                t,
                "MULTIPART_HEADER_FOLDING",
            ),
            m_variable_multipart_invalid_header_folding: AnchoredVariable::new(
                t,
                "MULTIPART_INVALID_HEADER_FOLDING",
            ),
            m_variable_multipart_invalid_part: AnchoredVariable::new(t, "MULTIPART_INVALID_PART"),
            m_variable_multipart_invalid_quoting: AnchoredVariable::new(
                t,
                "MULTIPART_INVALID_QUOTING",
            ),
            m_variable_multipart_lf_line: AnchoredVariable::new(t, "MULTIPART_LF_LINE"),
            m_variable_multipart_missing_semicolon: AnchoredVariable::new(
                t,
                "MULTIPART_MISSING_SEMICOLON",
            ),
            m_variable_multipart_strict_error: AnchoredVariable::new(t, "MULTIPART_STRICT_ERROR"),
            m_variable_multipart_unmatched_boundary: AnchoredVariable::new(
                t,
                "MULTIPART_UNMATCHED_BOUNDARY",
            ),
            m_variable_outbound_data_error: AnchoredVariable::new(t, "OUTBOUND_DATA_ERROR"),
            m_variable_path_info: AnchoredVariable::new(t, "PATH_INFO"),
            m_variable_query_string: AnchoredVariable::new(t, "QUERY_STRING"),
            m_variable_remote_addr: AnchoredVariable::new(t, "REMOTE_ADDR"),
            m_variable_remote_host: AnchoredVariable::new(t, "REMOTE_HOST"),
            m_variable_remote_port: AnchoredVariable::new(t, "REMOTE_PORT"),
            m_variable_reqbody_error: AnchoredVariable::new(t, "REQBODY_ERROR"),
            m_variable_reqbody_error_msg: AnchoredVariable::new(t, "REQBODY_ERROR_MSG"),
            m_variable_reqbody_processor_error: AnchoredVariable::new(t, "REQBODY_PROCESSOR_ERROR"),
            m_variable_reqbody_processor_error_msg: AnchoredVariable::new(
                t,
                "REQBODY_PROCESSOR_ERROR_MSG",
            ),
            m_variable_reqbody_processor: AnchoredVariable::new(t, "REQBODY_PROCESSOR"),
            m_variable_request_basename: AnchoredVariable::new(t, "REQUEST_BASENAME"),
            m_variable_request_body: AnchoredVariable::new(t, "REQUEST_BODY"),
            m_variable_request_body_length: AnchoredVariable::new(t, "REQUEST_BODY_LENGTH"),
            m_variable_request_filename: AnchoredVariable::new(t, "REQUEST_FILENAME"),
            m_variable_request_line: AnchoredVariable::new(t, "REQUEST_LINE"),
            m_variable_request_method: AnchoredVariable::new(t, "REQUEST_METHOD"),
            m_variable_request_protocol: AnchoredVariable::new(t, "REQUEST_PROTOCOL"),
            m_variable_request_uri: AnchoredVariable::new(t, "REQUEST_URI"),
            m_variable_request_uri_raw: AnchoredVariable::new(t, "REQUEST_URI_RAW"),
            m_variable_resource: AnchoredVariable::new(t, "RESOURCE"),
            m_variable_response_body: AnchoredVariable::new(t, "RESPONSE_BODY"),
            m_variable_response_content_length: AnchoredVariable::new(t, "RESPONSE_CONTENT_LENGTH"),
            m_variable_response_protocol: AnchoredVariable::new(t, "RESPONSE_PROTOCOL"),
            m_variable_response_status: AnchoredVariable::new(t, "RESPONSE_STATUS"),
            m_variable_server_addr: AnchoredVariable::new(t, "SERVER_ADDR"),
            m_variable_server_name: AnchoredVariable::new(t, "SERVER_NAME"),
            m_variable_server_port: AnchoredVariable::new(t, "SERVER_PORT"),
            m_variable_session_id: AnchoredVariable::new(t, "SESSIONID"),
            m_variable_unique_id: AnchoredVariable::new(t, "UNIQUE_ID"),
            m_variable_url_encoded_error: AnchoredVariable::new(t, "URLENCODED_ERROR"),
            m_variable_user_id: AnchoredVariable::new(t, "USERID"),

            m_variable_args: m_variable_args.clone(),
            m_variable_args_get: m_variable_args_get.clone(),
            m_variable_args_post: m_variable_args_post.clone(),
            m_variable_request_headers_names: AnchoredSetVariable::new(t, "REQUEST_HEADERS_NAMES"),
            m_variable_response_headers_names: AnchoredSetVariable::new(
                t,
                "RESPONSE_HEADERS_NAMES",
            ),
            m_variable_files_sizes: AnchoredSetVariable::new(t, "FILES_SIZES"),
            m_variable_files_names: AnchoredSetVariable::new(t, "FILES_NAMES"),
            m_variable_files_tmp_content: AnchoredSetVariable::new(t, "FILES_TMP_CONTENT"),
            m_variable_multipart_file_name: AnchoredSetVariable::new(t, "MULTIPART_FILENAME"),
            m_variable_multipart_name: AnchoredSetVariable::new(t, "MULTIPART_NAME"),
            m_variable_matched_vars_names: AnchoredSetVariable::new(t, "MATCHED_VARS_NAMES"),
            m_variable_matched_vars: AnchoredSetVariable::new(t, "MATCHED_VARS"),
            m_variable_files: AnchoredSetVariable::new(t, "FILES"),
            m_variable_request_cookies: AnchoredSetVariable::new(t, "REQUEST_COOKIES"),
            m_variable_request_headers: AnchoredSetVariable::new(t, "REQUEST_HEADERS"),
            m_variable_response_headers: AnchoredSetVariable::new(t, "RESPONSE_HEADERS"),
            m_variable_geo: AnchoredSetVariable::new(t, "GEO"),
            m_variable_request_cookies_names: AnchoredSetVariable::new(t, "REQUEST_COOKIES_NAMES"),
            m_variable_files_tmp_names: AnchoredSetVariable::new(t, "FILES_TMPNAMES"),
            m_variable_multipart_part_headers: AnchoredSetVariable::new(
                t,
                "MULTIPART_PART_HEADERS",
            ),
        }
    }
}
