use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum StepInRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepInArguments {
    /// Execute 'stepIn' for this thread.
    thread_id: u32,

    /// Optional id of the target to step into.
    target_id: Option<u32>,

    /// Optional granularity to step. If no granularity is specified, a granularity of 'statement'
    /// is assumed.
    granularity: Option<crate::types::SteppingGranularity>,
}

pub type StepInResponse = ();

impl Request for StepInRequest {
    type Arguments = StepInArguments;
    type Result = StepInResponse;

    const COMMAND: &'static str = "stepIn";
}
