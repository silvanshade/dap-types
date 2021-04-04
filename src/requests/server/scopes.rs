use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ScopesRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesArguments {
    /// Retrieve the scopes for this stackframe.
    frame_id: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesResponse {
    body: ScopesResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesResponseBody {
    /// The scopes of the stackframe. If the array has length zero, there are no scopes available.
    scopes: Vec<crate::types::Scope>,
}

impl Request for ScopesRequest {
    type Arguments = ScopesArguments;
    type Result = ScopesResponse;

    const COMMAND: &'static str = "scopes";
}
