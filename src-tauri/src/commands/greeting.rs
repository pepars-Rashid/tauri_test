use crate::error::AppError;

#[tauri::command]
#[specta::specta]
pub fn greet(name: String) -> Result<String, AppError> {
    Ok(format!("Hello, {name}!"))
}