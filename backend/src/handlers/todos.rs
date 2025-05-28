use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::MySqlPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::todo::{CreateTodoRequest, Todo, UpdateTodoRequest};

#[derive(Debug, Deserialize)]
pub struct ListTodosQuery {
    completed: Option<bool>,
}

pub async fn list_todos(
    State(pool): State<MySqlPool>,
    Query(query): Query<ListTodosQuery>,
) -> impl IntoResponse {
    info!("Listing todos with query: {:?}", query);
    let todos = match query.completed {
        Some(completed) => {
            sqlx::query_as::<_, Todo>(
                "SELECT * FROM todos WHERE completed = ? ORDER BY created_at DESC",
            )
            .bind(completed)
            .fetch_all(&pool)
            .await
        }
        None => {
            sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at DESC")
                .fetch_all(&pool)
                .await
        }
    };

    match todos {
        Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
        Err(e) => {
            error!("Failed to list todos: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to list todos" })),
            )
                .into_response()
        }
    }
}

pub async fn create_todo(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateTodoRequest>,
) -> impl IntoResponse {
    info!("Creating todo with payload: {:?}", payload);
    let todo = Todo::new(payload.title);

    match sqlx::query(
        "INSERT INTO todos (id, title, completed, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(todo.id)
    .bind(&todo.title)
    .bind(todo.completed)
    .bind(todo.created_at)
    .bind(todo.updated_at)
    .execute(&pool)
    .await
    {
        Ok(_) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => {
            error!("Failed to create todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to create todo" })),
            )
                .into_response()
        }
    }
}

pub async fn get_todo(State(pool): State<MySqlPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    info!("Getting todo with id: {}", id);
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(todo) => (StatusCode::OK, Json(todo)).into_response(),
        Err(sqlx::Error::RowNotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Todo not found" })),
        )
            .into_response(),
        Err(e) => {
            error!("Failed to get todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to get todo" })),
            )
                .into_response()
        }
    }
}

pub async fn update_todo(
    State(pool): State<MySqlPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodoRequest>,
) -> impl IntoResponse {
    info!("Updating todo with id: {} and payload: {:?}", id, payload);
    let mut transaction = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to begin transaction: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to update todo" })),
            )
                .into_response();
        }
    };

    let todo = match sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = ? FOR UPDATE")
        .bind(id)
        .fetch_one(&mut *transaction)
        .await
    {
        Ok(todo) => todo,
        Err(sqlx::Error::RowNotFound) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Todo not found" })),
            )
                .into_response();
        }
        Err(e) => {
            error!("Failed to fetch todo for update: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to update todo" })),
            )
                .into_response();
        }
    };

    let title = payload.title.unwrap_or(todo.title);
    let completed = payload.completed.unwrap_or(todo.completed);
    let updated_at = Utc::now();

    match sqlx::query("UPDATE todos SET title = ?, completed = ?, updated_at = ? WHERE id = ?")
        .bind(&title)
        .bind(completed)
        .bind(updated_at)
        .bind(id)
        .execute(&mut *transaction)
        .await
    {
        Ok(_) => {
            if let Err(e) = transaction.commit().await {
                error!("Failed to commit transaction: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": "Failed to update todo" })),
                )
                    .into_response();
            }
            let updated_todo = Todo {
                id,
                title,
                completed,
                created_at: todo.created_at, // Retain original created_at
                updated_at,
            };
            (StatusCode::OK, Json(updated_todo)).into_response()
        }
        Err(e) => {
            error!("Failed to update todo: {}", e);
            // Attempt to rollback transaction, but ignore error if rollback fails
            let _ = transaction.rollback().await;
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to update todo" })),
            )
                .into_response()
        }
    }
}

pub async fn delete_todo(State(pool): State<MySqlPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    info!("Deleting todo with id: {}", id);
    match sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({ "error": "Todo not found" })),
                )
                    .into_response()
            } else {
                (StatusCode::NO_CONTENT, "").into_response()
            }
        }
        Err(e) => {
            error!("Failed to delete todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to delete todo" })),
            )
                .into_response()
        }
    }
}
