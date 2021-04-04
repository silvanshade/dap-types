use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum PauseRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PauseArguments {
    /// Pause execution for this thread.
    thread_id: u32,
}

pub type PauseResponse = ();

impl Request for PauseRequest {
    type Arguments = PauseArguments;
    type Result = PauseResponse;

    const COMMAND: &'static str = "pause";
}
