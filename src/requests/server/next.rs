use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum NextRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NextArguments {
    /// Execute 'next' for this thread.
    thread_id: u32,

    /// Optional granularity to step. If no granularity is specified, a granularity of 'statement'
    /// is assumed.
    granularity: Option<crate::types::SteppingGranularity>,
}

pub type NextResponse = ();

impl Request for NextRequest {
    type Arguments = NextArguments;
    type Result = NextResponse;

    const COMMAND: &'static str = "next";
}
