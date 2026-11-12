use sea_orm::Database;
use tauri::Manager;

mod bindings;
mod commands;
mod error;

use crate::bindings::specta_builder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    #[cfg(debug_assertions)]
    builder
        .export(specta_typescript::Typescript::default(), "../src/bindings.ts")
        .expect("failed to export TS bindings");

    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("no app data dir");
            std::fs::create_dir_all(&app_dir).ok();
            let db_path = app_dir.join("app.db");
            let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

            tauri::async_runtime::block_on(async {
                let db = Database::connect(&db_url).await.expect("DB connect failed");
                app.manage(db);
            });

            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}