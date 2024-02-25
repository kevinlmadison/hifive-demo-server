use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Command {
    pub value: String,
    pub updatedAt: Option<DateTime<Utc>>
}

pub type DB = Arc<Mutex<Command>>;

pub fn command_db() -> DB {
    Arc::new(Mutex::new(Command {
        value: "GREEN".to_string(),
        updatedAt: Some(chrono::Utc::now())
    }))
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateCommand {
    pub value: Option<String>
}
