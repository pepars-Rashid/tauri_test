use tauri_specta::{Builder, collect_commands};

pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        crate::commands::greeting::greet,
        crate::commands::users::get_user,
        crate::commands::students::my_command,
    ])
}

// ╔══════════════════════════════════════════════════════════════════════╗
// ║  HOW TO ADD A NEW TAURI COMMAND                                      ║
// ║ ───────────────────────────────────────────────────────────────────  ║
// ║  1. Create src-tauri/src/commands/<domain>.rs                        ║
// ║     - Define a struct with #[derive(Serialize, Type)]               ║
// ║     - Add #[tauri::command] #[specta::specta] fn ... -> Result<_, AppError>
// ║  2. Add `pub mod <domain>;` to src-tauri/src/commands/mod.rs          ║
// ║     - Add `pub use <domain>::<function>;` to commands/mod.rs          ║
// ║  3. Add the command to collect_commands![...] above                   ║
// ║  4. Run `cargo build` or `cargo tauri dev` to regenerate src/bindings.ts
// ║                                                                       ║
// ║  Rules:                                                               ║
// ║  • Use i32, not i64/u64 (Specta forbids BigInt-style types for TS)   ║
// ║  • No command bodies in lib.rs or commands/mod.rs                    ║
// ║  • No raw `invoke` in frontend — only `commands.*` from ../bindings   ║
// ╚══════════════════════════════════════════════════════════════════════╝
