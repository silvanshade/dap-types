use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum LoadedSourcesRequest {}

pub type LoadedSourcesArguments = ();

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedSourcesResponse {
    body: LoadedSourcesResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedSourcesResponseBody {
    /// Set of loaded sources.
    sources: Vec<crate::types::Source>,
}

impl Request for LoadedSourcesRequest {
    type Arguments = LoadedSourcesArguments;
    type Result = LoadedSourcesResponse;

    const COMMAND: &'static str = "loadedSources";
}
