use serde::{de, Serialize};

pub mod events;
pub mod requests;
pub mod types;

pub trait RequestResponse {
    type Arguments: de::DeserializeOwned + Serialize;
    type Body: de::DeserializeOwned + Serialize;
    const COMMAND: &'static str;
}
