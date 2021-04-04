use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum BreakpointLocationsRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocationsArguments {
    /// The source location of the breakpoints; either 'source.path' or 'source.reference' must be
    /// specified.
    source: crate::types::Source,

    /// Start line of range to search possible breakpoint locations in. If only the line is
    /// specified, the request returns all possible locations in that line.
    line: u32,

    /// Optional start column of range to search possible breakpoint locations in. If no start
    /// column is given, the first column in the start line is assumed.
    column: Option<u32>,

    /// Optional end line of range to search possible breakpoint locations in. If no end line is
    /// given, then the end line is assumed to be the start line.
    end_line: Option<u32>,

    /// Optional end column of range to search possible breakpoint locations in. If no end column is
    /// given, then it is assumed to be in the last column of the end line.
    end_column: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocationsResponse {
    body: BreakpointLocationsResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocationsResponseBody {
    /// Sorted set of possible breakpoint locations.
    breakpoints: Vec<crate::types::BreakpointLocation>,
}

impl Request for BreakpointLocationsRequest {
    type Arguments = BreakpointLocationsArguments;
    type Result = BreakpointLocationsResponse;

    const COMMAND: &'static str = "breakpointLocations";
}
