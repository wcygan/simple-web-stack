use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
    pub jti: String, // JWT ID (session ID)
}

#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry_hours: i64,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key".to_string()),
            token_expiry_hours: env::var("TOKEN_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
        }
    }
}

#[derive(Clone)]
pub struct AuthService {
    config: AuthConfig,
}

impl AuthService {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    /// Generate a JWT token for a user
    pub fn generate_token(
        &self,
        user_id: Uuid,
        session_id: Uuid,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let expiry = now + Duration::hours(self.config.token_expiry_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiry.timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: session_id.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_ref()),
        )
    }

    /// Validate and decode a JWT token
    pub fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_ref()),
            &Validation::default(),
        )
    }

    /// Extract user ID from token
    pub fn get_user_id_from_token(&self, token: &str) -> Result<Uuid, Box<dyn std::error::Error>> {
        let token_data = self.validate_token(token)?;
        let user_id = Uuid::parse_str(&token_data.claims.sub)?;
        Ok(user_id)
    }

    /// Extract session ID from token
    pub fn get_session_id_from_token(&self, token: &str) -> Result<Uuid, Box<dyn std::error::Error>> {
        let token_data = self.validate_token(token)?;
        let session_id = Uuid::parse_str(&token_data.claims.jti)?;
        Ok(session_id)
    }

    /// Check if token is expired
    pub fn is_token_expired(&self, token: &str) -> bool {
        match self.validate_token(token) {
            Ok(_) => false,
            Err(err) => matches!(err.kind(), jsonwebtoken::errors::ErrorKind::ExpiredSignature),
        }
    }

    /// Get token expiry duration
    pub fn get_token_expiry_duration(&self) -> chrono::Duration {
        Duration::hours(self.config.token_expiry_hours)
    }
}

/// Hash a password using bcrypt
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

/// Generate a secure random token for session management
pub fn generate_session_token() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const TOKEN_LEN: usize = 64;
    let mut rng = rand::thread_rng();

    (0..TOKEN_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Hash a session token for storage
pub fn hash_session_token(token: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "test_password";
        let hash = hash_password(password).unwrap();
        
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_session_token_generation() {
        let token = generate_session_token();
        assert_eq!(token.len(), 64);
        
        // Generate another token to ensure they're different
        let token2 = generate_session_token();
        assert_ne!(token, token2);
    }

    #[test]
    fn test_jwt_token_generation_and_validation() {
        let config = AuthConfig {
            jwt_secret: "test_secret".to_string(),
            token_expiry_hours: 24,
        };
        let auth_service = AuthService::new(config);
        
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        
        let token = auth_service.generate_token(user_id, session_id).unwrap();
        let token_data = auth_service.validate_token(&token).unwrap();
        
        assert_eq!(token_data.claims.sub, user_id.to_string());
        assert_eq!(token_data.claims.jti, session_id.to_string());
    }

    #[test]
    fn test_user_id_extraction() {
        let config = AuthConfig {
            jwt_secret: "test_secret".to_string(),
            token_expiry_hours: 24,
        };
        let auth_service = AuthService::new(config);
        
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        
        let token = auth_service.generate_token(user_id, session_id).unwrap();
        let extracted_user_id = auth_service.get_user_id_from_token(&token).unwrap();
        
        assert_eq!(user_id, extracted_user_id);
    }
}