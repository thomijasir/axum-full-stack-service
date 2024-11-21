use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde::{Serialize, Deserialize};
use std::fmt;
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String
}
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).expect("Failed convert to string"))
    }
}
// #[derive(Debug, PartialEq)]
// pub enum StatusMessage {
//     SUCCESS,
//     FAIL,
// }
// impl StatusMessage {
//     fn to_str(&self) -> String {
//         match self {
//             StatusMessage::SUCCESS => "success".to_string(),
//             StatusMessage::FAIL => "fail".to_string(),
//             _ => "Unknown status".to_owned()
//         }
//     }
// }

// impl ToString for StatusMessage {
//     fn to_string(&self) -> String {
//         self.to_str().to_owned()
//     }
// }

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InvalidHashFormat,
    HashingError,
    InvalidToken,
    // ServiceError,
    WrongCredentials,
    EmailExists,
    // UserNotFound,
    UserNoLongerExists,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::EmptyPassword => "Empty password".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(length) => format!("Exceeded maximum password length: {}", length),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::HashingError => "Invalid hashing format".to_string(),
            ErrorMessage::InvalidToken => "Invalid token".to_string(),
            // ErrorMessage::ServiceError => "Service error".to_string(),
            ErrorMessage::WrongCredentials => "Wrong credentials".to_string(),
            ErrorMessage::EmailExists => "Email already exists".to_string(),
            // ErrorMessage::UserNotFound => "User not found".to_string(),
            ErrorMessage::UserNoLongerExists => "User no longer exists".to_string(),
            ErrorMessage::TokenNotProvided => "Token not provided".to_string(),
            ErrorMessage::PermissionDenied => "Permission denied".to_string(),
            ErrorMessage::UserNotAuthenticated => "User not authenticated".to_string(),
            _ => "Unknown error".to_owned()
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new (message: impl Into<String>, status: StatusCode) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }
    pub fn server_error (message: impl Into<String>) -> Self {
        HttpError{
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }
    pub fn unique_constraint_violation (message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::CONFLICT
        }
    }
    pub fn unauthorized (message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::UNAUTHORIZED
        }
    }
    pub fn into_http_response(self) -> Response {
        let error_body = ErrorResponse{
            status: "fail".to_string(),
            message: self.message.clone()
        };
        (self.status, Json(error_body)).into_response()
    }
}
impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}
impl std::error::Error for HttpError {}
impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}