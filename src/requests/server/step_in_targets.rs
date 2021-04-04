use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum StepInTargetsRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepInTargetsArguments {
    /// The stack frame for which to retrieve the possible stepIn targets.
    frame_id: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepInTargetsResponse {
    body: StepInTargetsResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepInTargetsResponseBody {
    /// The possible stepIn targets of the specified source location.
    targets: Vec<crate::types::StepInTarget>,
}

impl Request for StepInTargetsRequest {
    type Arguments = StepInTargetsArguments;
    type Result = StepInTargetsResponse;

    const COMMAND: &'static str = "stepInTargets";
}
