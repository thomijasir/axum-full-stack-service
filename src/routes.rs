use std::sync::Arc;

use axum::{middleware, Extension, Router};
use tower_http::trace::TraceLayer;
use crate::AppState;
use crate::middleware::auth;
use crate::services::auth::auth_handler;
use crate::services::users::users_handler;

pub fn create(app_state: Arc<AppState>) -> Router {
    let api_routes = Router::new()
        .nest("/auth", auth_handler())
        .nest("/users", users_handler()
            .layer(middleware::from_fn(auth))
        )
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    Router::new().nest("/v1/api", api_routes)
}