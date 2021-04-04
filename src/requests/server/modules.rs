use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ModuleRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleArguments {
    /// The index of the first module to return; if omitted modules start at 0.
    start_module: Option<u32>,

    /// The number of modules to return. If moduleCount is not specified or 0, all modules are
    /// returned.
    module_count: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleResponse {
    body: ModuleResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleResponseBody {
    /// All modules or range of modules.
    modules: Vec<crate::types::Module>,

    /// The total number of modules available.
    total_modules: Option<u32>,
}

impl Request for ModuleRequest {
    type Arguments = ModuleArguments;
    type Result = ModuleResponse;

    const COMMAND: &'static str = "modules";
}
