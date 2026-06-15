use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong",
        )
            .into_response()
    }
}