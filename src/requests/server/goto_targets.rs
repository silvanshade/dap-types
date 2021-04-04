use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum GotoTargetsRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoTargetsArguments {
    /// The source location for which the goto targets are determined.
    source: crate::types::Source,

    /// The line location for which the goto targets are determined.
    line: u32,

    /// An optional column location for which the goto targets are determined.
    column: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoTargetsResponse {
    body: GotoTargetsResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoTargetsResponseBody {
    /// The possible goto targets of the specified location.
    targets: Vec<crate::types::GotoTarget>,
}

impl Request for GotoTargetsRequest {
    type Arguments = GotoTargetsArguments;
    type Result = GotoTargetsResponse;

    const COMMAND: &'static str = "gotoTargets";
}
