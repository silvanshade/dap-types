use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum StepOutRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepOutArguments {
    /// Execute 'stepOut' for this thread.
    thread_id: u32,

    /// Optional granularity to step. If no granularity is specified, a granularity of 'statement'
    /// is assumed.
    granularity: Option<crate::types::SteppingGranularity>,
}

pub type StepOutResponse = ();

impl Request for StepOutRequest {
    type Arguments = StepOutArguments;
    type Result = StepOutResponse;

    const COMMAND: &'static str = "stepOut";
}
