use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DisconnectRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectArguments {
    /// A value of true indicates that this 'disconnect' request is part of a restart sequence.
    restart: Option<bool>,

    /// Indicates whether the debuggee should be terminated when the debugger is disconnected. If
    /// unspecified, the debug adapter is free to do whatever it thinks is best. The attribute is
    /// only honored by a debug adapter if the capability 'supportTerminateDebuggee' is true.
    terminate_debuggee: Option<bool>,
}

pub type DisconnectResponse = ();

impl Request for DisconnectRequest {
    type Arguments = DisconnectArguments;
    type Result = DisconnectResponse;

    const COMMAND: &'static str = "disconnect";
}
