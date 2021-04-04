use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SetDataBreakpointsRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDataBreakpointsArguments {
    /// The contents of this array replaces all existing data breakpoints. An empty array clears all
    /// data breakpoints.
    breakpoints: Option<Vec<crate::types::DataBreakpoint>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDataBreakpointsResponse {
    body: SetDataBreakpointsResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDataBreakpointsResponseBody {
    /// Information about the data breakpoints. The array elements correspond to the elements of the
    /// input argument 'breakpoints' array.
    breakpoints: Vec<crate::types::Breakpoint>,
}

impl Request for SetDataBreakpointsRequest {
    type Arguments = SetDataBreakpointsArguments;
    type Result = SetDataBreakpointsResponse;

    const COMMAND: &'static str = "setDataBreakpoints";
}
