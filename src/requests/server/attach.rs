use crate::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub enum AttachRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachArguments {
    __restart: Option<Value>,
}

pub type AttachResponse = ();

impl Request for AttachRequest {
    type Arguments = AttachArguments;
    type Result = AttachResponse;

    const COMMAND: &'static str = "attach";
}
