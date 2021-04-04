use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum GotoRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoArguments {
    /// Set the goto target for this thread.
    thread_id: u32,

    /// The location where the debuggee will continue to run.
    target_id: u32,
}

pub type GotoResponse = ();

pub type GotoResponseBody = ();

impl Request for GotoRequest {
    type Arguments = GotoArguments;
    type Result = GotoResponse;

    const COMMAND: &'static str = "goto";
}
