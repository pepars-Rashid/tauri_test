# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Adding a New Tauri Command (Typed IPC via tauri-specta)

Rust is the API. `tauri-specta` generates `src/bindings.ts` from your Rust
commands. The frontend imports `commands` from `../bindings` and never calls
raw `invoke`.

### The 3-file rule

To add a new command you touch exactly 3 files:

| File | What to do |
|------|-----------|
| `src-tauri/src/commands/<domain>.rs` | Create the domain file: define a struct with `#[derive(Serialize, Type)]`, then a `#[tauri::command] #[specta::specta]` function returning `Result<T, AppError>`. |
| `src-tauri/src/commands/mod.rs` | Add `pub mod <domain>;` and `pub use <domain>::<function>;`. |
| `src-tauri/src/bindings.rs` | Add the function to `collect_commands![...]`. |

Then run `cargo build` or `cargo tauri dev` to regenerate `src/bindings.ts`.

### Copy-paste template

**1. Create the domain file** (`src-tauri/src/commands/<domain>.rs`):

```rust
use crate::error::AppError;
use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
pub struct MyResponse {
    pub name: String,
    pub age: i32,
    pub grades: Vec<i32>,
}

#[tauri::command]
#[specta::specta]
pub async fn my_command(
    state: tauri::State<'_, DatabaseConnection>,
    id: i32,
) -> Result<MyResponse, AppError> {
    // DB query here...
    Ok(MyResponse { ... })
}
```

**2. Register the module** (`src-tauri/src/commands/mod.rs`):

```rust
pub mod greeting;
pub mod users;
pub mod students;
pub mod my_domain;           // <-- add this

pub use greeting::greet;
pub use users::get_user;
pub use students::get_user;
pub use my_domain::my_command; // <-- add this
```

**3. Register the command** (`src-tauri/src/bindings.rs`):

```rust
Builder::<tauri::Wry>::new().commands(collect_commands![
    crate::commands::greeting::greet,
    crate::commands::users::get_user,
    crate::commands::students::get_user,
    crate::commands::my_domain::my_command, // <-- add this
])
```

**4. Regenerate bindings:**

```bash
cargo build          # or: cargo tauri dev
```

This regenerates `src/bindings.ts` automatically.

### Consume from frontend

```tsx
import { commands } from "../bindings";

const result = await commands.my_command(1);
// result: MyResponse (fully typed)
```

### Rules (do not violate)

- Use `i32`, not `i64`/`u64` — Specta forbids BigInt-style types for TypeScript.
- No command bodies in `lib.rs` or `commands/mod.rs`.
- No raw `invoke` in components — only `commands.*` from `../bindings`.
- No `src/api/client.ts`, `src/api/services/`, or `src/api/hooks/`.
- No Zod schemas; no custom `TauriError` class; `AppError` is never wrapped in `Option<T>`.

### Type safety

If you change a Rust command signature or struct, `tauri-specta` regenerates
`src/bindings.ts` and TypeScript will fail at build time until the frontend is
updated. That is the point: Rust is the source of truth.
