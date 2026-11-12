use serde::Serialize;
use specta::Type;

#[derive(Debug, thiserror::Error, Clone, Serialize, Type)]
#[serde(tag = "type")]
pub enum AppError {
    #[error("{message}")]
    NotFound { message: String },
    #[error("{message}")]
    Validation { message: String },
    #[error("{message}")]
    Internal { message: String },
}