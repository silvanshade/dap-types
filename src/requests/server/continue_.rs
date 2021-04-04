use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ContinueRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueArguments {
    /// Continue execution for the specified thread (if possible). If the backend cannot continue on
    /// a single thread but will continue on all threads, it should set the 'allThreadsContinued'
    /// attribute in the response to true.
    thread_id: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueResponse {
    body: ContinueResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueResponseBody {
    /// If true, the 'continue' request has ignored the specified thread and continued all threads
    /// instead. If this attribute is missing a value of 'true' is assumed for backward
    /// compatibility.
    all_threads_continued: Option<bool>,
}

impl Request for ContinueRequest {
    type Arguments = ContinueArguments;
    type Result = ContinueResponse;

    const COMMAND: &'static str = "continue";
}
