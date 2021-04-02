use serde::{de, Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ErrorResponse {
    seq: u32,
    kind: ErrorResponseType,
    request_seq: u32,
    success: bool,
    command: String,
    message: crate::protocol::ResponseMessage,
    body: Option<Map<String, Value>>,
    #[serde(skip_serializing)]
    error: Option<crate::types::Message>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorResponseType {
    #[serde(rename = "response")]
    ErrorResponse,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorResponseMessage {
    Cancelled,
    String(String),
}

impl<'de> Deserialize<'de> for ErrorResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ErrorResponseVisitor;
        impl<'de> de::Visitor<'de> for ErrorResponseVisitor {
            type Value = ErrorResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "struct ErrorResponse")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut seq = None;
                let mut kind = None;
                let mut request_seq = None;
                let mut success = None;
                let mut command = None;
                let mut message = None;
                let mut body = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "seq" => {
                            if seq.is_some() {
                                return Err(de::Error::duplicate_field("seq"));
                            }
                            seq = Some(map.next_value()?);
                        },
                        "kind" => {
                            if kind.is_some() {
                                return Err(de::Error::duplicate_field("kind"));
                            }
                            kind = Some(map.next_value()?);
                        },
                        "request_seq" => {
                            if request_seq.is_some() {
                                return Err(de::Error::duplicate_field("request_seq"));
                            }
                            request_seq = Some(map.next_value()?);
                        },
                        "success" => {
                            if success.is_some() {
                                return Err(de::Error::duplicate_field("success"));
                            }
                            success = Some(map.next_value()?);
                        },
                        "command" => {
                            if command.is_some() {
                                return Err(de::Error::duplicate_field("command"));
                            }
                            command = Some(map.next_value()?);
                        },
                        "message" => {
                            if command.is_some() {
                                return Err(de::Error::duplicate_field("message"));
                            }
                            message = Some(map.next_value()?);
                        },
                        "body" => {
                            if command.is_some() {
                                return Err(de::Error::duplicate_field("body"));
                            }
                            body = Some(map.next_value()?);
                        },
                        _ => {},
                    }
                }

                let seq = seq.ok_or_else(|| de::Error::missing_field("seq"))?;
                let kind = kind.ok_or_else(|| de::Error::missing_field("kind"))?;
                let request_seq = request_seq.ok_or_else(|| de::Error::missing_field("request_seq"))?;
                let success = success.ok_or_else(|| de::Error::missing_field("success"))?;
                let command = command.ok_or_else(|| de::Error::missing_field("command"))?;
                let message = message.ok_or_else(|| de::Error::missing_field("message"))?;
                let body: Option<Map<String, Value>> = body.ok_or_else(|| de::Error::missing_field("body"))?;

                let mut error = None;

                if let Some(ref body) = body {
                    if let Some(result) = body.get("error") {
                        let result = result.clone();
                        let result = serde_json::from_value(result).map_err(de::Error::custom)?;
                        error = Some(result);
                    }
                }

                Ok(ErrorResponse {
                    seq,
                    kind,
                    request_seq,
                    success,
                    command,
                    message,
                    body,
                    error,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "seq",
            "kind",
            "request_seq",
            "success",
            "command",
            "message",
            "body",
            "error",
        ];
        let result = deserializer.deserialize_struct("ErrorReponse", FIELDS, ErrorResponseVisitor)?;
        Ok(result)
    }
}
