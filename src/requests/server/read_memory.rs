use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ReadMemoryRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadMemoryArguments {
    /// Memory reference to the base location from which data should be read.
    memory_reference: String,

    /// Optional offset (in bytes) to be applied to the reference location before reading data. Can
    /// be negative.
    offset: Option<u32>,

    /// Number of bytes to read at the specified location and offset.
    count: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadMemoryResponse {
    body: Option<ReadyMemoryResponseBody>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadyMemoryResponseBody {
    /// The address of the first byte of data returned. Treated as a hex value if prefixed with
    /// '0x', or as a decimal value otherwise.
    address: String,

    /// The number of unreadable bytes encountered after the last successfully read byte. This can
    /// be used to determine the number of bytes that must be skipped before a subsequent
    /// 'readMemory' request will succeed.
    unreadable_bytes: Option<u32>,

    /// The bytes read from memory, encoded using base64.
    data: Option<String>,
}

impl Request for ReadMemoryRequest {
    type Arguments = ReadMemoryArguments;
    type Result = ReadMemoryResponse;

    const COMMAND: &'static str = "readMemory";
}
