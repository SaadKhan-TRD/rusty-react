use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sqlx::SqlitePool;

use crate::models::note::{CreateNote, Note};

pub async fn create_note(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<CreateNote>,
) -> Response {
    let result = sqlx::query("INSERT INTO notes (note) VALUES (?)")
        .bind(&payload.note)
        .execute(&pool)
        .await;

    match result {
        Ok(res) => {
            let id = res.last_insert_rowid();

            let note = sqlx::query_as::<_, Note>(
                "SELECT id, note, created_at, updated_at FROM notes WHERE id = ?",
            )
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();

            (StatusCode::CREATED, Json(note)).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": err.to_string() })),
        )
            .into_response(),
    }
}

pub async fn get_notes(Extension(pool): Extension<SqlitePool>) -> Response {
    let result = sqlx::query_as::<_, Note>("SELECT id, note, created_at, updated_at FROM notes")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": err.to_string() })),
        )
            .into_response(),
    }
}

pub async fn delete_note(Path(id): Path<i64>, Extension(pool): Extension<SqlitePool>) -> Response {
    let result = sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": err.to_string() })),
        )
            .into_response(),
    }
}
