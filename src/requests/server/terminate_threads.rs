use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum TerminateThreadsRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateThreadsArguments {
    /// Ids of threads to be terminated.
    thread_ids: Option<u32>,
}

pub type TerminateThreadsResponse = ();

impl Request for TerminateThreadsRequest {
    type Arguments = TerminateThreadsArguments;
    type Result = TerminateThreadsResponse;

    const COMMAND: &'static str = "terminateThreads";
}
