use crate::Request;

#[derive(Debug)]
pub enum RestartRequest {}

pub type RestartArguments = ();

pub type RestartResponse = ();

impl Request for RestartRequest {
    type Arguments = RestartArguments;
    type Result = RestartResponse;

    const COMMAND: &'static str = "restart";
}
