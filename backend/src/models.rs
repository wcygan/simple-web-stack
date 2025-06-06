use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub user_id: Uuid,
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

// Pagination parameters for listing tasks
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

// Sort order for pagination
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

// Paginated response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

// Pagination metadata
#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub page_size: u32,
    pub total_items: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_previous: bool,
}

impl PaginationMeta {
    pub fn new(page: u32, page_size: u32, total_items: u64) -> Self {
        let total_pages = if total_items == 0 {
            1
        } else {
            ((total_items - 1) / page_size as u64 + 1) as u32
        };
        
        Self {
            page,
            page_size,
            total_items,
            total_pages,
            has_next: page < total_pages,
            has_previous: page > 1,
        }
    }
}

impl PaginationParams {
    pub fn validate(&mut self) {
        // Ensure page is at least 1
        if self.page == 0 {
            self.page = 1;
        }
        
        // Limit page size to reasonable bounds
        if self.page_size == 0 || self.page_size > 100 {
            self.page_size = default_page_size();
        }
    }
    
    pub fn offset(&self) -> u64 {
        ((self.page - 1) * self.page_size) as u64
    }
    
    pub fn limit(&self) -> u64 {
        self.page_size as u64
    }
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

// Authentication Models

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
}

// Auth Payloads

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

// Search parameters for filtering tasks
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,      // Search query for title
    pub status: Option<String>, // Filter by status: "completed", "pending", "all"
}

// Combined query parameters for tasks endpoint
#[derive(Debug, Deserialize)]
pub struct TaskQueryParams {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(flatten)]
    pub search: SearchParams,
}
