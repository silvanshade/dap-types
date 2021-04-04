use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum CompletionsRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionsArguments {
    /// Returns completions in the scope of this stack frame. If not specified, the completions are
    /// returned for the global scope.
    frame_id: Option<u32>,

    /// One or more source lines. Typically this is the text a user has typed into the debug console
    /// before he asked for completion.
    text: String,

    /// The character position for which to determine the completion proposals.
    column: u32,

    /// An optional line for which to determine the completion proposals. If missing the first line
    /// of the text is assumed.
    line: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionsResponse {
    body: CompletionsResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionsResponseBody {
    /// The possible completions for .
    targets: Vec<crate::types::CompletionItem>,
}

impl Request for CompletionsRequest {
    type Arguments = CompletionsArguments;
    type Result = CompletionsResponse;

    const COMMAND: &'static str = "completions";
}
