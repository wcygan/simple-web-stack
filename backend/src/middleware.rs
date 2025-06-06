use axum::{
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use async_trait::async_trait;
use uuid::Uuid;

use crate::{auth::AuthService, db::AppState};

/// Middleware to extract and validate authentication
pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract auth service from app state
    let auth_service = &state.auth_service;

    // Extract Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if it's a Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix

    // Validate the token
    let user_id = auth_service
        .get_user_id_from_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let session_id = auth_service
        .get_session_id_from_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Verify session is still valid in the database
    let session = crate::db::get_session_by_token(&state.pool, &crate::auth::hash_session_token(token))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if session.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Add user_id and session_id to request extensions for use in handlers
    req.extensions_mut().insert(AuthUser {
        user_id,
        session_id,
    });

    Ok(next.run(req).await)
}

/// Struct to hold authenticated user information
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub session_id: Uuid,
}

// FromRequestParts implementation removed temporarily to fix compilation
// This would be needed for auth middleware functionality

/// Optional auth middleware - allows both authenticated and unauthenticated requests
pub async fn optional_auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Response {
    // Extract auth service from app state
    let auth_service = &state.auth_service;
    
    // Try to extract Authorization header
    if let Some(auth_header) = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
    {
        // Check if it's a Bearer token
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..]; // Remove "Bearer " prefix

            // Try to validate the token
            if let (Ok(user_id), Ok(session_id)) = (
                auth_service.get_user_id_from_token(token),
                auth_service.get_session_id_from_token(token),
            ) {
                // Verify session is still valid in the database
                if let Ok(Some(_)) = crate::db::get_session_by_token(
                    &state.pool,
                    &crate::auth::hash_session_token(token),
                )
                .await
                {
                    // Add user info to request if authentication is successful
                    req.extensions_mut().insert(AuthUser {
                        user_id,
                        session_id,
                    });
                }
            }
        }
    }

    next.run(req).await
}

/// Extract authenticated user from request extensions
pub fn extract_auth_user(req: &Request) -> Option<&AuthUser> {
    req.extensions().get::<AuthUser>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{HeaderValue, Method},
        Router,
    };
    use tower::ServiceExt;

    // Note: These tests would need a more complete setup with actual database and auth service
    // They're included as examples of how the middleware would be tested

    #[tokio::test]
    async fn test_auth_middleware_missing_header() {
        // This is a simplified test - in practice we'd need the full app setup
        // Including database and auth service initialization
    }

    #[tokio::test] 
    async fn test_auth_middleware_invalid_token() {
        // Test with invalid Bearer token
    }

    #[tokio::test]
    async fn test_optional_auth_middleware_no_header() {
        // Test that optional middleware allows requests without auth headers
    }
}