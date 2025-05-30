use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

// Payload for creating a task (omits id, created_at, updated_at as they are auto-generated/managed)
#[derive(Debug, Deserialize)]
pub struct CreateTaskPayload {
    pub title: String,
}

// Payload for updating a task (all fields optional)
#[derive(Debug, Deserialize)]
pub struct UpdateTaskPayload {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
