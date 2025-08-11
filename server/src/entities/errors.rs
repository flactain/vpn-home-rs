use std::fmt::Debug;

use axum::{Json, http::StatusCode, response::IntoResponse};
use log::error;
use sqlx::{self};
use thiserror::Error;

use crate::entities::dto::response_dto::ResponseDto;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Resource Not Found")]
    NotFound,

    #[error("Invalid Input: {0}")]
    InvalidInput(String),

    #[error["Something Go Wrong"]]
    DatabaseError(#[source] sqlx::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(sqlx_error: sqlx::Error) -> Self {
        match sqlx_error {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::DatabaseError(other),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ResponseDto::new(self.to_string().as_str(), "")),
            )
                .into_response(),
            Self::InvalidInput(app_error) => (
                StatusCode::BAD_REQUEST,
                Json(ResponseDto::new(app_error.to_string().as_str(), "")),
            )
                .into_response(),
            Self::DatabaseError(sqlx_error) => {
                error!("{:?}", sqlx_error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseDto::new("Something Go Wrong", "")),
                )
                    .into_response()
            }
            Self::AnyhowError(anyhow_error) => {
                error!("{:?}", anyhow_error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseDto::new(anyhow_error.to_string().as_str(), "")),
                )
                    .into_response()
            }
        }
    }
}
