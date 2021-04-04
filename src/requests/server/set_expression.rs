use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SetExpressionRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExpressionArguments {
    /// The l-value expression to assign to.
    expression: String,

    /// The value expression to assign to the l-value expression.
    value: String,

    /// Evaluate the expressions in the scope of this stack frame. If not specified, the expressions
    /// are evaluated in the global scope.
    frame_id: Option<u32>,

    /// Specifies how the resulting value should be formatted.
    format: Option<crate::types::ValueFormat>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExpressionResponse {
    body: SetExpressionResponseBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExpressionResponseBody {
    /// The new value of the expression.
    value: String,

    /// The optional type of the value.
    /// This attribute should only be returned by a debug adapter if the client has passed the value
    /// true for the 'supportsVariableType' capability of the 'initialize' request.
    #[serde(rename = "type")]
    kind: Option<String>,

    /// Properties of a value that can be used to determine how to render the result in the UI.
    presentation_hint: Option<crate::types::VariablePresentationHint>,

    /// If variablesReference is > 0, the value is structured and its children can be retrieved by
    /// passing variablesReference to the VariablesRequest. The value should be less than or equal
    /// to 2147483647 (2^31-1).
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

impl Request for SetExpressionRequest {
    type Arguments = SetExpressionArguments;
    type Result = SetExpressionResponse;

    const COMMAND: &'static str = "setExpression";
}
