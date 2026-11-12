use crate::error::AppError;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
pub struct Student {
    pub name: String,
    pub age: i32,
    pub grades: Vec<i32>,
}

#[tauri::command]
#[specta::specta]
pub async fn my_command(
    state: tauri::State<'_, DatabaseConnection>,
    id: i32,
) -> Result<Student, AppError> {
    // DB query here...
    Ok(Student {
        name: "Ahmed".into(),
        age: 37,
        grades: vec![10, 20, 44],
    })
}
