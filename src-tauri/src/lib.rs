// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("no app data dir");
            std::fs::create_dir_all(&app_dir).ok();
            let db_path = app_dir.join("app.db");
            let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

            tauri::async_runtime::block_on(async {
                let db = Database::connect(&db_url).await.expect("DB connect failed");
                // Run your migrations here
                // migrator::Migrator::up(&db, None).await.expect("Migration failed");
                app.manage(db);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}