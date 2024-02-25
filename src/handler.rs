use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    model::{Command, UpdateCommand, DB},
    response::{SingleCommandResponse, CommandData},
};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn get_command_handler(
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let maybe_command = db.lock().await;

    if let Some(command) =  Some(maybe_command) {
        let json_response = SingleCommandResponse {
            status: "success".to_string(),
            data: CommandData { command: command.clone() },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Command not found")
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn update_command_handler(
    State(db): State<DB>,
    Json(body): Json<UpdateCommand>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let maybe_command = db.lock().await;

    if let Some(mut command) = Some(maybe_command) {
        let datetime = chrono::Utc::now();
        let value = body
            .value
            .to_owned()
            .unwrap_or_else(|| command.value.to_owned());
        let payload = Command {
            value: if !value.is_empty() {
                value
            } else {
                command.value.to_owned()
            },
            updatedAt: Some(datetime),
        };
        *command = payload;

        let json_response = SingleCommandResponse {
            status: "success".to_string(),
            data: CommandData { command: command.clone() },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Command not found")
        });

        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}
