use crate::Request;
use serde::{de, Deserialize, Serialize, Serializer};
use std::fmt;

#[derive(Debug)]
pub enum EvaluateRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateArguments {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateResponse {
    /// The expression to evaluate.
    expression: String,

    /// Evaluate the expression in the scope of this stack frame. If not specified, the expression
    /// is evaluated in the global scope.
    frame_id: Option<u32>,

    #[rustfmt::skip]
    /// The context in which the evaluate request is run.
    /// Values:
    ///   'watch': evaluate is run in a watch.
    ///   'repl': evaluate is run from REPL console.
    ///   'hover': evaluate is run from a data hover.
    ///   'clipboard': evaluate is run to generate the value that will be stored in the clipboard.
    ///     The attribute is only honored by a debug adapter if the capability 'supportsClipboardContext' is true.
    ///   etc.
    context: EvaluateResponseContext,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EvaluateResponseContext {
    Clipboard,
    Hover,
    Repl,
    Watch,
    String(String),
}

impl Request for EvaluateRequest {
    type Arguments = EvaluateArguments;
    type Result = EvaluateResponse;

    const COMMAND: &'static str = "evaluate";
}

impl<'de> Deserialize<'de> for EvaluateResponseContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct EvaluateResponseContextVisitor;
        impl<'de> de::Visitor<'de> for EvaluateResponseContextVisitor {
            type Value = EvaluateResponseContext;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                for str in &["clipboard", "hover", "repl", "watch"] {
                    write!(formatter, "\"{}\", ", str)?;
                }
                write!(formatter, "or any other string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let result = match value {
                    "clipboard" => EvaluateResponseContext::Clipboard,
                    "hover" => EvaluateResponseContext::Hover,
                    "repl" => EvaluateResponseContext::Repl,
                    "watch" => EvaluateResponseContext::Watch,
                    _ => EvaluateResponseContext::String(String::from(value)),
                };
                Ok(result)
            }
        }

        deserializer.deserialize_any(EvaluateResponseContextVisitor)
    }
}

impl Serialize for EvaluateResponseContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            EvaluateResponseContext::Clipboard => "clipboard",
            EvaluateResponseContext::Hover => "hover",
            EvaluateResponseContext::Repl => "repl",
            EvaluateResponseContext::Watch => "watch",
            EvaluateResponseContext::String(value) => value.as_str(),
        };
        serializer.serialize_str(value)
    }
}
