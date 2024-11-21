use std::sync::Arc;
use axum::{middleware, Extension, Json, Router};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::{get, put};
use validator::Validate;
use crate::{
    AppState,
    db::UserExt,
    models::UserRole,
    error::{HttpError,ErrorMessage},
    middleware::{role_check, JWTAuthMiddleware},
    utils::{password},
    dtos::{
        FilterUserDto, UserResponseDto,
        RoleUpdateDto, RequestQueryDto, UserListResponseDto,
        NameUpdateDto, UserPasswordUpdateDto, Response
    }
};

pub fn users_handler() -> Router {
    Router::new()
        .route("/me",
               get(get_me)
                   .layer(middleware::from_fn(|state, req, next|{
                       role_check(state, req, next, vec![UserRole::Admin, UserRole::User])
                   }))
        )
        .route("/users",
                   get(get_users)
                   .layer(middleware::from_fn(|state, req, next| {
                       role_check(state, req, next, vec![UserRole::Admin])
                   }))
        )
        .route("/name", put(update_user_name))
        .route("/role", put(update_user_role))
        .route("/password", put(update_user_password))
}

pub async fn  get_me(
    Extension(user): Extension<JWTAuthMiddleware>
) -> Result<impl IntoResponse, HttpError> {
    let filtered_user = FilterUserDto::filter_user(&user.user);
    let response_data = UserResponseDto{
        status: "success".to_string(),
        data: filtered_user
    };

    Ok(Json(response_data))
}

// Paginated Users Fetch Data
pub async fn get_users(
    Query(query_params): Query<RequestQueryDto>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    query_params.validate()
        .map_err(|err| HttpError::bad_request(err.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(1);

    let users = state.db_client
        .get_users(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user_count = state.db_client
        .get_user_count()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = UserListResponseDto{
        status: String::from("success"),
        users: FilterUserDto::filter_users(&users),
        results: user_count,
    };
    Ok(Json(response))
}
pub async fn update_user_name(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Json(body): Json<NameUpdateDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let result = app_state.db_client.
        update_user_name(user_id.clone(), &body.name)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let filtered_user = FilterUserDto::filter_user(&result);

    let response = UserResponseDto {
        data: filtered_user,
        status: "success".to_string(),
    };

    Ok(Json(response))
}

pub async fn update_user_role(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Json(body): Json<RoleUpdateDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let result = app_state.db_client
        .update_user_role(user_id.clone(), body.role)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let filtered_user = FilterUserDto::filter_user(&result);

    let response = UserResponseDto {
        data: filtered_user,
        status: "success".to_string(),
    };

    Ok(Json(response))
}

pub async fn update_user_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Json(body): Json<UserPasswordUpdateDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).expect("fail parse user id");
    let result = app_state.db_client
        .get_user(Some(user_id.clone()), None, None, None).await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let user = result.ok_or(HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()))?;
    let password_match = password::compare(&body.old_password, &user.password)
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    if !password_match {
        return Err(HttpError::bad_request("old password is incorrect".to_string()));
    }

    let hash_password = password::hash(&body.new_password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    app_state.db_client
        .update_user_password(user_id.clone(), hash_password).await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let response = Response {
        message: "Password update successfully".to_string(),
        status: "success",
    };

    Ok(Json(response))

}