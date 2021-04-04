use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DisassembleRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleArguments {
    /// Memory reference to the base location containing the instructions to disassemble.
    memory_reference: String,

    /// Optional offset (in bytes) to be applied to the reference location before disassembling. Can
    /// be negative.
    offset: Option<u32>,

    /// Optional offset (in instructions) to be applied after the byte offset (if any) before
    /// disassembling. Can be negative.
    instruction_offset: Option<u32>,

    /// Number of instructions to disassemble starting at the specified location and offset. An
    /// adapter must return exactly this number of instructions - any unavailable instructions
    /// should be replaced with an implementation-defined 'invalid instruction' value.
    instruction_count: u32,

    /// If true, the adapter should attempt to resolve memory addresses and other values to symbolic
    /// names.
    resolve_symbols: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleResponse {
    body: Option<DisassembleResponseBody>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleResponseBody {
    /// The list of disassembled instructions.
    instructions: Vec<crate::types::DisassembledInstruction>,
}

impl Request for DisassembleRequest {
    type Arguments = DisassembleArguments;
    type Result = DisassembleResponse;

    const COMMAND: &'static str = "disassemble";
}
