use axum::{
    routing::get,
    Router,
};

use crate::{
    handler::{
        update_command_handler, get_command_handler, health_checker_handler,
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::command_db();

    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/api/command",
            get(get_command_handler)
                .patch(update_command_handler),
        )
        .with_state(db)
}
