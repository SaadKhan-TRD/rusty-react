use axum::Extension;
use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::SqlitePool;

use crate::handlers::note_handler::{create_note, delete_note, get_notes};
use crate::handlers::ui_handler::ui_handler;

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { ui_handler(axum::extract::Path("".into())).await }),
        )
        .route("/{*file}", get(ui_handler))
        .route("/api/notes", get(get_notes))
        .route("/api/notes", post(create_note))
        .route("/api/notes/{id}", delete(delete_note))
        .layer(Extension(pool))
}
