use std::fmt::Debug;

use log::error;
use sqlx::{self};
use thiserror::Error;

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
