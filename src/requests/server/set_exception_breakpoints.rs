use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SetExceptionBreakpointsRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExceptionBreakpointsArguments {
    /// Set of exception filters specified by their ID. The set of all possible exception filters is
    /// defined by the 'exceptionBreakpointFilters' capability. The 'filter' and 'filterOptions'
    /// sets are additive.
    filters: Vec<String>,

    /// Set of exception filters and their options. The set of all possible exception filters is
    /// defined by the 'exceptionBreakpointFilters' capability. This attribute is only honored by a
    /// debug adapter if the capability 'supportsExceptionFilterOptions' is true. The 'filter' and
    /// 'filterOptions' sets are additive.
    filter_options: Option<Vec<crate::types::ExceptionFilterOptions>>,

    /// Configuration options for selected exceptions. The attribute is only honored by a debug
    /// adapter if the capability 'supportsExceptionOptions' is true.
    exception_options: Option<Vec<crate::types::ExceptionOptions>>,
}

pub type SetExceptionBreakpointsResponse = ();

impl Request for SetExceptionBreakpointsRequest {
    type Arguments = SetExceptionBreakpointsArguments;
    type Result = SetExceptionBreakpointsResponse;

    const COMMAND: &'static str = "setExceptionBreakpoints";
}
