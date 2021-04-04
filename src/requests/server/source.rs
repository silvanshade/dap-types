use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SourceRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceArguments {
    /// Specifies the source content to load. Either source.path or source.sourceReference must be
    /// specified.
    source: Option<crate::types::Source>,

    /// The reference to the source. This is the same as source.sourceReference.
    /// This is provided for backward compatibility since old backends do not understand the
    /// 'source' attribute.
    source_reference: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceResponse {
    body: SourceResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceResponseBody {
    /// Content of the source reference.
    content: String,

    /// Optional content type (mime type) of the source.
    mime_type: Option<String>,
}

impl Request for SourceRequest {
    type Arguments = SourceArguments;
    type Result = SourceResponse;

    const COMMAND: &'static str = "source";
}
