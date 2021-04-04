use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SetInstructionBreakpointsRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInstructionBreakpointsArguments {
    /// The instruction references of the breakpoints
    breakpoints: Vec<crate::types::InstructionBreakpoint>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInstructionBreakpointsResponse {
    body: SetInstructionBreakpointsResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInstructionBreakpointsResponseBody {
    /// Information about the breakpoints. The array elements correspond to the elements of the
    /// 'breakpoints' array.
    breakpoints: Vec<crate::types::Breakpoint>,
}

impl Request for SetInstructionBreakpointsRequest {
    type Arguments = SetInstructionBreakpointsArguments;
    type Result = SetInstructionBreakpointsResponse;

    const COMMAND: &'static str = "setInstructionBreakpoints";
}
