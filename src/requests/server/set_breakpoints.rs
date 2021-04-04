use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SetBreakpointsRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsArguments {
    /// The source location of the breakpoints; either 'source.path' or 'source.reference' must be
    /// specified.
    source: crate::types::Source,

    /// The code locations of the breakpoints.
    breakpoints: Option<Vec<crate::types::SourceBreakpoint>>,

    /// Deprecated: The code locations of the breakpoints.
    lines: Option<Vec<u32>>,

    /// A value of true indicates that the underlying source has been modified which results in new
    /// breakpoint locations.
    source_modified: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsResponse {
    body: SetBreakpointsResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsResponseBody {
    /// Information about the breakpoints. The array elements are in the same order as the elements
    /// of the 'breakpoints' (or the deprecated 'lines') array in the arguments.
    breakpoints: Vec<crate::types::Breakpoint>,
}

impl Request for SetBreakpointsRequest {
    type Arguments = SetBreakpointsArguments;
    type Result = SetBreakpointsResponse;

    const COMMAND: &'static str = "setBreakpoints";
}
