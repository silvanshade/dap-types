use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConfigurationDoneRequest {}

pub type ConfigurationDoneArguments = ();

pub type ConfigurationDoneResponse = ();

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConfigurationDoneResponseBody {
    /// The possible completions for .
    targets: Vec<crate::types::CompletionItem>,
}

impl Request for ConfigurationDoneRequest {
    type Arguments = ConfigurationDoneArguments;
    type Result = ConfigurationDoneResponse;

    const COMMAND: &'static str = "configurationDone";
}
