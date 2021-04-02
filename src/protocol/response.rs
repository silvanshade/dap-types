use serde::{de, Deserialize, Serialize, Serializer};
use serde_json::{Map, Value};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Response {
    seq: u32,
    #[serde(rename = "type")]
    kind: ResponseType,
    request_seq: u32,
    success: bool,
    command: String,
    message: ResponseMessage,
    body: Option<Map<String, Value>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    Response,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResponseMessage {
    Cancelled,
    String(String),
}

impl<'de> Deserialize<'de> for ResponseMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ResponseMessageVisitor;
        impl<'de> de::Visitor<'de> for ResponseMessageVisitor {
            type Value = ResponseMessage;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "expecting: \"cancelled\", or any other string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "cancelled" => Ok(ResponseMessage::Cancelled),
                    _ => Ok(ResponseMessage::String(String::from(value))),
                }
            }
        }

        deserializer.deserialize_any(ResponseMessageVisitor)
    }
}

impl Serialize for ResponseMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            ResponseMessage::Cancelled => "cancelled",
            ResponseMessage::String(value) => value.as_str(),
        };

        serializer.serialize_str(value)
    }
}
