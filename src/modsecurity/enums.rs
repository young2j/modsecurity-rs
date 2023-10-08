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

pub enum Phases {
    /**
     *
     * The connection is the very first information that ModSecurity can
     * inspect. It is expected to happens before the virtual host name be
     * resolved. This phase is expected to happen immediately after a
     * connection is established.
     *
     */
    ConnectionPhase,
    /**
     *
     * The "URI" phase happens just after the web server (or any other
     * application that you may use with ModSecurity) have the acknowledgement
     * of the full request URI.
     *
     */
    UriPhase,
    /**
     *
     * The "RequestHeaders" phase happens when the server has all the
     * information about the headers. Notice however, that it is expected to
     * happen prior to the reception of the request body (if any).
     *
     */
    RequestHeadersPhase,
    /**
     *
     * At the "RequestHeaders" phase, ModSecurity is expected to inspect the
     * content of a request body, that does not happens when the server has all
     * the content but prior to that, when the body transmission started.
     * ModSecurity can ask the webserver to block (or make any other disruptive
     * action) while the client is still transmitting the data.
     *
     */
    RequestBodyPhase,
    /**
     *
     * The "ResponseHeaders" happens just before all the response headers are
     * ready to be delivery to the client.
     *
     */
    ResponseHeadersPhase,
    /**
     *
     * Same as "RequestBody" the "ResponseBody" phase perform a stream
     * inspection which may result in a disruptive action.
     *
     */
    ResponseBodyPhase,
    /**
     *
     * The last phase is the logging phase. At this phase ModSecurity will
     * generate the internal logs, there is no need to hold the request at
     * this point as this phase does not produce any kind of action.
     *
     */
    LoggingPhase,
    /**
     * Just a marking for the expected number of phases.
     *
     */
    NumberOfPhases,
}

/**
 *
 * Properties used to configure the general log callback.
 *
 */
pub enum LogProperty {
    /**
     *
     * Original ModSecurity text log entry. The same entry that can be found
     * within the Apache error_log (in the 2.x family)
     *
     */
    TextLogProperty = 1,
    /**
     *
     * Instead of return the text log entry an instance of the class
     * RuleMessages is returned.
     *
     */
    RuleMessageLogProperty = 2,
    /**
     * This property only makes sense with the utilization of the
     * RuleMessageLogProperty. Without this property set the RuleMessage
     * structure will not be filled with the information of the hightlight.
     *
     * Notice that the highlight can be calculate post-analisys. Calculate it
     * during the analisys may delay the analisys process.
     *
     */
    IncludeFullHighlightLogProperty = 4,
}
