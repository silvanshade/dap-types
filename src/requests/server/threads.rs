use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ThreadsRequest {}

pub type ThreadsArguments = ();

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadsResponse {
    body: ThreadsResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadsResponseBody {
    /// All threads.
    threads: Vec<crate::types::Thread>,
}

impl Request for ThreadsRequest {
    type Arguments = ThreadsArguments;
    type Result = ThreadsResponse;

    const COMMAND: &'static str = "threads";
}
