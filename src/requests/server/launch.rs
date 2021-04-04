use crate::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub enum LaunchRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchArguments {
    /// If noDebug is true the launch request should launch the program without enabling debugging.
    no_debug: Option<bool>,

    /// Optional data from the previous, restarted session. The data is sent as the 'restart'
    /// attribute of the 'terminated' event. The client should leave the data intact.
    __restart: Option<Value>,
}

pub type LaunchResponse = ();

impl Request for LaunchRequest {
    type Arguments = LaunchArguments;
    type Result = LaunchResponse;

    const COMMAND: &'static str = "launch";
}
