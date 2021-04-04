use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum TerminateRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateArguments {
    /// A value of true indicates that this 'terminate' request is part of a restart sequence.
    restart: Option<bool>,
}

pub type TerminateResponse = ();

impl Request for TerminateRequest {
    type Arguments = TerminateArguments;
    type Result = TerminateResponse;

    const COMMAND: &'static str = "terminate";
}
