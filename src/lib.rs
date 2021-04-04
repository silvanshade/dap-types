use serde::{de, Serialize};

pub mod events;
pub mod requests;
pub mod types;

pub trait Request {
    type Arguments: de::DeserializeOwned + Serialize;
    type Result: de::DeserializeOwned + Serialize;
    const COMMAND: &'static str;
}
