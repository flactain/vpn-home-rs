use axum::{Json, http::StatusCode, response::IntoResponse};
use log::error;

use crate::entities::{dto::response_dto::ResponseDto, errors::AppError};

#[derive(Debug)]
pub enum HttpResponse<T> {
    Ok(T),
    Created(T),
    Updated,
    NotFound(AppError),
    ServerError(AppError),
    InvalidInput(AppError),
}

impl<T> IntoResponse for HttpResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Ok(data) => {
                (StatusCode::OK, Json(ResponseDto::new("Success", data))).into_response()
            }
            Self::Created(data) => {
                (StatusCode::CREATED, Json(ResponseDto::new("Created", data))).into_response()
            }
            Self::Updated => (
                StatusCode::NO_CONTENT,
                Json(ResponseDto::new("Success", "")),
            )
                .into_response(),
            Self::NotFound(app_error) => (
                StatusCode::NOT_FOUND,
                Json(ResponseDto::new(app_error.to_string().as_str(), "")),
            )
                .into_response(),
            Self::InvalidInput(app_error) => (
                StatusCode::BAD_REQUEST,
                Json(ResponseDto::new(app_error.to_string().as_str(), "")),
            )
                .into_response(),
            Self::ServerError(app_error) => {
                error!("{:?}", app_error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseDto::new(app_error.to_string().as_str(), "")),
                )
                    .into_response()
            }
        }
    }
}
