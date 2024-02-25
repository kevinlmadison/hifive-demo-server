use crate::model::Command;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct CommandData {
    pub command: Command,
}

#[derive(Serialize, Debug)]
pub struct SingleCommandResponse {
    pub status: String,
    pub data: CommandData,
}
