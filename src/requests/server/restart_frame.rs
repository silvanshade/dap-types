use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum RestartFrameRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestartFrameArguments {
    /// Restart this stackframe.
    frame_id: u32,
}

pub type RestartFrameResponse = ();

impl Request for RestartFrameRequest {
    type Arguments = RestartFrameArguments;
    type Result = RestartFrameResponse;

    const COMMAND: &'static str = "restartFrame";
}
