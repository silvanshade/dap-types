use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SetFunctionBreakpointsRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFunctionBreakpointsArguments {
    /// The function names of the breakpoints.
    breakpoints: Vec<crate::types::FunctionBreakpoint>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFunctionBreakpointsResponse {
    body: SetFunctionBreakpointsResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFunctionBreakpointsResponseBody {
    /// Information about the breakpoints. The array elements correspond to the elements of the
    /// 'breakpoints' array.
    breakpoints: Vec<crate::types::Breakpoint>,
}

impl Request for SetFunctionBreakpointsRequest {
    type Arguments = SetFunctionBreakpointsArguments;
    type Result = SetFunctionBreakpointsResponse;

    const COMMAND: &'static str = "setFunctionBreakpoints";
}
