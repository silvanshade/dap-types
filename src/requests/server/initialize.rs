use crate::Request;
use serde::{de, Deserialize, Serialize, Serializer};
use std::fmt;

#[derive(Debug)]
pub enum InitializeRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeArguments {
    /// The ID of the (frontend) client using this adapter.
    #[serde(rename = "clientID")]
    client_id: String,

    /// The human readable name of the (frontend) client using this adapter.
    client_name: Option<String>,

    /// The ID of the debug adapter.
    #[serde(rename = "adapterID")]
    adapter_id: String,

    /// The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US or de-CH.
    locale: Option<String>,

    /// If true all line numbers are 1-based (default).
    lines_start_at_1: Option<bool>,

    /// If true all column numbers are 1-based (default).
    columns_start_at_1: Option<bool>,

    /// Determines in what format paths are specified. The default is 'path', which is the native
    /// format.   Values: 'path', 'uri', etc.
    path_format: Option<InitializeArgumentsPathFormat>,

    /// Client supports the optional type attribute for variables.
    supports_variable_type: Option<bool>,

    /// Client supports the paging of variables.
    supports_variable_paging: Option<bool>,

    /// Client supports the runInTerminal request.
    supports_run_in_terminal_request: Option<bool>,

    /// Client supports memory references.
    supports_memory_references: Option<bool>,

    /// Client supports progress reporting.
    supports_progress_reporting: Option<bool>,

    /// Client supports the invalidated event.
    supports_invalidated_event: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InitializeArgumentsPathFormat {
    Path,
    Uri,
    String(String),
}

pub type InitializeResponse = ();

impl Request for InitializeRequest {
    type Arguments = InitializeArguments;
    type Result = InitializeResponse;

    const COMMAND: &'static str = "initialize";
}

impl<'de> Deserialize<'de> for InitializeArgumentsPathFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct InitializeArgumentsPathFormatVisitor;
        impl<'de> de::Visitor<'de> for InitializeArgumentsPathFormatVisitor {
            type Value = InitializeArgumentsPathFormat;

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
                    "path" => InitializeArgumentsPathFormat::Path,
                    "uri" => InitializeArgumentsPathFormat::Uri,
                    _ => InitializeArgumentsPathFormat::String(String::from(value)),
                };
                Ok(result)
            }
        }

        deserializer.deserialize_any(InitializeArgumentsPathFormatVisitor)
    }
}

impl Serialize for InitializeArgumentsPathFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            InitializeArgumentsPathFormat::Path => "path",
            InitializeArgumentsPathFormat::Uri => "uri",
            InitializeArgumentsPathFormat::String(value) => value.as_str(),
        };
        serializer.serialize_str(value)
    }
}
