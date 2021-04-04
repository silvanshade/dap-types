use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ExceptionInfoRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionInfoArguments {
    /// Thread for which exception information should be retrieved.
    thread_id: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionInfoResponse {
    body: ExceptionInfoResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionInfoResponseBody {
    /// ID of the exception that was thrown.
    exception_id: String,

    /// Descriptive text for the exception provided by the debug adapter.
    description: Option<String>,

    /// Mode that caused the exception notification to be raised.
    break_mode: crate::types::ExceptionBreakMode,

    /// Detailed information about the exception.
    details: Option<crate::types::ExceptionDetails>,
}

impl Request for ExceptionInfoRequest {
    type Arguments = ExceptionInfoArguments;
    type Result = ExceptionInfoResponse;

    const COMMAND: &'static str = "exceptionInfo";
}
