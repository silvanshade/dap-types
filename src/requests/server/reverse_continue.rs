use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ReverseContinueRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseContinueArguments {
    /// Execute 'reverseContinue' for this thread.
    frame_id: u32,
}

pub type ReverseContinueResponse = ();

impl Request for ReverseContinueRequest {
    type Arguments = ReverseContinueArguments;
    type Result = ReverseContinueResponse;

    const COMMAND: &'static str = "reverseContinue";
}
