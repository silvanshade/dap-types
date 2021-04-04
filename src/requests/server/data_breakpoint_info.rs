use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DataBreakpointInfoRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBreakpointInfoArguments {
    /// Reference to the Variable container if the data breakpoint is requested for a child of the
    /// container.
    variables_reference: Option<u32>,

    /// The name of the Variable's child to obtain data breakpoint information for. If
    /// variablesReference isn’t provided, this can be an expression.
    name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBreakpointInfoResponse {
    body: DataBreakpointInfoResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBreakpointInfoResponseBody {
    /// An identifier for the data on which a data breakpoint can be registered with the
    /// setDataBreakpoints request or null if no data breakpoint is available.
    data_id: Option<String>,

    /// UI string that describes on what data the breakpoint is set on or why a data breakpoint is
    /// not available.
    description: String,

    /// Optional attribute listing the available access types for a potential data breakpoint. A UI
    /// frontend could surface this information.
    access_types: Option<Vec<crate::types::DataBreakpointAccessType>>,

    /// Optional attribute indicating that a potential data breakpoint could be persisted across
    /// sessions.
    can_persist: Option<bool>,
}

impl Request for DataBreakpointInfoRequest {
    type Arguments = DataBreakpointInfoArguments;
    type Result = DataBreakpointInfoResponse;

    const COMMAND: &'static str = "dataBreakpointInfo";
}
