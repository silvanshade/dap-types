use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SetVariableRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableArguments {
    /// The reference of the variable container.
    variables_reference: u32,

    /// The name of the variable in the container.
    name: String,

    /// The value of the variable.
    value: String,

    /// Specifies details on how to format the response value.
    format: Option<crate::types::ValueFormat>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResponse {
    body: SetVariableResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResponseBody {
    /// The new value of the variable.
    value: String,

    /// The type of the new value. Typically shown in the UI when hovering over the value.
    #[serde(rename = "type")]
    kind: String,

    /// If variablesReference is > 0, the new value is structured and its children can be retrieved
    /// by passing variablesReference to the VariablesRequest. The value should be less than or
    /// equal to 2147483647 (2^31-1).
    variables_reference: Option<u32>,

    /// The number of named child variables.
    /// The client can use this optional information to present the variables in a paged UI and
    /// fetch them in chunks. The value should be less than or equal to 2147483647 (2^31-1).
    named_variables: Option<u32>,

    /// The number of indexed child variables.
    /// The client can use this optional information to present the variables in a paged UI and
    /// fetch them in chunks. The value should be less than or equal to 2147483647 (2^31-1).
    indexed_variables: Option<u32>,
}

impl Request for SetVariableRequest {
    type Arguments = SetVariableArguments;
    type Result = SetVariableResponse;

    const COMMAND: &'static str = "setVariable";
}
