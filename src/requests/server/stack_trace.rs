use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum StackTraceRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceArguments {
    /// Retrieve the stacktrace for this thread.
    thread_id: u32,

    /// The index of the first frame to return; if omitted frames start at 0.
    start_frame: Option<u32>,

    /// The maximum number of frames to return. If levels is not specified or 0, all frames are
    /// returned.
    levels: Option<u32>,

    /// Specifies details on how to format the stack frames.
    /// The attribute is only honored by a debug adapter if the capability
    /// 'supportsValueFormattingOptions' is true.
    format: Option<crate::types::StackFrameFormat>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceResponse {
    body: StackTraceResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceResponseBody {
    /// Content of the source reference.
    content: String,

    /// Optional content type (mime type) of the source.
    mime_type: Option<String>,
}

impl Request for StackTraceRequest {
    type Arguments = StackTraceArguments;
    type Result = StackTraceResponse;

    const COMMAND: &'static str = "stackTrace";
}
