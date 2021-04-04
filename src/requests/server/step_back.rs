use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum StepBackRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepBackArguments {
    /// Execute 'stepBack' for this thread.
    thread_id: u32,

    /// Optional granularity to step. If no granularity is specified, a granularity of 'statement'
    /// is assumed.
    granularity: Option<crate::types::SteppingGranularity>,
}

pub type StepBackResponse = ();

impl Request for StepBackRequest {
    type Arguments = StepBackArguments;
    type Result = StepBackResponse;

    const COMMAND: &'static str = "stepBack";
}
