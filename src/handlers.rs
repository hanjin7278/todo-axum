use crate::db::DataBase;
use crate::models::{CreateTodoRequest, Todo, UpdateTodoRequest};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

// 使用 Arc 包装 Database 以便共享
pub type AppState = Arc<DataBase>;

/// GET /todos
pub async fn list_todos(State(db): State<AppState>) -> Result<Json<Vec<Todo>>, StatusCode> {
    db.list_todos().await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// GET /todos/:id
pub async fn get_todo(State(db): State<AppState>, Path(id): Path<String>) -> Result<Json<Todo>, StatusCode> {
    match db.get_todo(&id).await {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// POST /todos
pub async fn create_todo(State(db): State<AppState>, Json(payload): Json<CreateTodoRequest>) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    match db.create_todo(payload).await {
        Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// PUT /todos/:id
pub async fn update_todo(State(db): State<AppState>, Path(id): Path<String>, Json(payload): Json<UpdateTodoRequest>) -> Result<Json<Todo>, StatusCode> {
    match db.update_todo(&id, payload).await {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// DELETE /todos/:id
pub async fn delete_todo(State(db): State<AppState>, Path(id): Path<String>) -> Result<StatusCode, StatusCode> {
    match db.delete_todo(&id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
