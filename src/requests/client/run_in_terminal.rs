use crate::Request;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug)]
pub enum RunInTerminalRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInTerminalArguments {
    /// What kind of terminal to launch. Values: 'integrated', 'external', etc.
    kind: Option<RunInTerminalKind>,

    /// Optional title of the terminal.
    title: Option<String>,

    /// Working directory for the command. For non-empty, valid paths this typically results in
    /// execution of a change directory command.
    cwd: String,

    /// List of arguments. The first argument is the command to run.
    args: Vec<String>,

    /// Environment key-value pairs that are added to or removed from the default environment.
    env: Option<Map<String, Value>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RunInTerminalKind {
    External,
    Integrated,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInTerminalResponse {
    body: RunInTerminalResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInTerminalResponseBody {
    /// The process ID. The value should be less than or equal to 2147483647 (2^31-1).
    process_id: Option<u32>,

    /// The process ID of the terminal shell. The value should be less than or equal to 2147483647
    /// (2^31-1).
    shell_process_id: Option<u32>,
}

impl Request for RunInTerminalRequest {
    type Arguments = RunInTerminalArguments;
    type Result = RunInTerminalResponse;

    const COMMAND: &'static str = "runInTerminal";
}
