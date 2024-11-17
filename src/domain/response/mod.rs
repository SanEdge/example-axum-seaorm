use serde::Serialize;
use core::fmt;
use std::fmt::Formatter;

mod category;
mod post;
mod comment;
mod user;

use crate::utils::AppError;

pub use self::category::CategoryResponse;
pub use self::post::{
    PostResponse,
    PostRelationResponse
};
pub use self::comment::CommentResponse;
pub use self::user::UserResponse;


#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

impl<T: Serialize> fmt::Display for ApiResponse<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "{}", json),
            Err(e) => write!(f, "Error serializing ApiResponse to JSON: {}", e),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        let (status, message) = match error {
            AppError::DbError(_) => ("error".to_string(), "Database error occurred".to_string()),
            AppError::HashingError(_) => ("error".to_string(), "Error during password hashing".to_string()),
            AppError::NotFound(ref msg) => ("error".to_string(), msg.clone()),
            AppError::TokenExpiredError => ("error".to_string(), "Token has expired".to_string()),
            AppError::TokenValidationError => ("error".to_string(), "Token validation failed".to_string()),
            AppError::TokenGenerationError(_) => ("error".to_string(), "Token generation failed".to_string()),
            AppError::BcryptError(ref msg) => ("error".to_string(), format!("Bcrypt error: {}", msg)),
            AppError::InvalidCredentials => ("error".to_string(), "Invalid credentials".to_string()),
            AppError::EmailAlreadyExists => ("error".to_string(), "Email already exists".to_string()),
        };
        ErrorResponse { status, message }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}