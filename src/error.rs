use std::fmt::{Formatter, Display};

use sqlx;
use axum;

#[derive(Debug)]
pub enum AppError {
    Sqlx(sqlx::Error),
    Axum(axum::Error),
}

impl From<axum::Error> for AppError {
    fn from(err: axum::Error) -> Self{
        AppError::Axum(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self{
        AppError::Sqlx(err)
    }
}

impl Display for AppError {
    fn fmt(&self, frm: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            AppError::Axum(e) => {
                write!(frm, "web error: {}", e)
            }
            AppError::Sqlx(e) => {
                write!(frm, "database error: {}", e)
            }
        }
    }
}