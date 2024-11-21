// DTO stands for Data Transfer Object,
// which is a type of object that moves data between processes

use core::str;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize };
use validator::Validate;

use crate::models::{User, UserRole};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto{
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be more than 6 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Confirm password is required"),
        must_match(other = "password", message = "password do not match")
    )]
    pub password_confirm: String,
}
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be more than 6 characters")
    )]
    pub password: String,
}
#[derive(Validate, Serialize, Deserialize)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1))]
    pub limit: Option<usize>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    #[serde(rename="createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename="updatedAt")]
    pub updated_at: DateTime<Utc>,
}
impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto{
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            verified: user.verified,
            role: user.role.to_str().to_string(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap()
        }
    }
    pub fn filter_users(user: &[User]) -> Vec<FilterUserDto> {
        user.iter().map(FilterUserDto::filter_user).collect()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct  UserResponseDto {
    pub status: String,
    pub data: UserData
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: i64
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String
}
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String
}
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}
#[derive(Validate, Debug, Clone, Serialize, Deserialize)]
pub struct RoleUpdateDto {
    #[validate(custom = "validate_user_role")]
    pub role: UserRole,
}
fn validate_user_role(role: &UserRole) -> Result<(), validator::ValidationError> {
    match role {
        UserRole::Admin | UserRole::User => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_role"))
    }
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdateDto {
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be more than 6 characters")
    )]
    pub new_password: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be more than 6 characters"),
        must_match(other="new_password", message="Password do not match")
    )]
    pub new_password_confirm: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be more than 6 characters")
    )]
    pub old_password: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct ForgotPasswordRequestDto {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
}
#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequestDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be more than 6 characters")
    )]
    pub new_password: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be more than 6 characters"),
        must_match(other="new_password", message="Password do not match")
    )]
    pub new_password_confirm: String,
}