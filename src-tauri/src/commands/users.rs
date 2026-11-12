use crate::error::AppError;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[tauri::command]
#[specta::specta]
pub async fn get_user(
    state: tauri::State<'_, DatabaseConnection>,
    id: i32,
) -> Result<User, AppError> {
    let _ = (state, id); // placeholder until the entity is wired up
    Err(AppError::NotFound { message: format!("user {id} not found") })
}