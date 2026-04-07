# Rust Code Style Guide (Tauri Backend)

## 1. General

- Use **Rust 2024 edition** or latest stable
- Enable strict clippy: `#![deny(clippy::all)]` in lib
- Run `cargo clippy` before commits
- No `unsafe` code without documented justification

## 2. Naming Conventions

| Entity | Convention | Example |
|--------|------------|---------|
| Crate | kebab-case | `ark-server-manager` |
| Module | snake_case | `mod server_config;` |
| Struct | PascalCase | `struct ServerProfile` |
| Enum | PascalCase | `enum ServerStatus` |
| Function | snake_case | `fn read_ini_file()` |
| Variable | snake_case | `let profile_id = ...` |
| Constant | SCREAMING_SNAKE | `const DEFAULT_PORT: u16 = 27015;` |
| Type Alias | PascalCase | `type Result<T> = Result<T, Error>;` |

## 3. Error Handling

- Use `thiserror` for error types
- Propagate errors with `?` operator
- Never use `unwrap()` in production code
- Never use `expect()` with generic messages

```rust
// Good
#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Server not found: {0}")]
    NotFound(String),
    #[error("Failed to start server: {0}")]
    StartFailed(#[from] io::Error),
}

// Bad
fn get_server() -> Server {
    servers.get(&id).unwrap() // NO!
}
```

## 4. Tauri Commands

```rust
// All public Tauri commands in src/commands.rs
#[tauri::command]
pub async fn start_server(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // ... implementation
    Ok(())
}
```

## 5. Module Structure

```
src-tauri/src/
├── main.rs           # Entry point, Tauri builder setup
├── lib.rs            # Re-exports, module declarations
├── commands/         # Tauri command handlers
│   ├── mod.rs
│   ├── server.rs     # Server lifecycle commands
│   └── config.rs     # INI/config commands
├── models/           # Data structures
│   ├── mod.rs
│   └── profile.rs
├── services/         # Business logic
│   ├── mod.rs
│   ├── server_manager.rs
│   └── ini_parser.rs
└── error.rs          # Error types
```

## 6. Async

- Use **tokio** as the async runtime (Tauri default)
- Use `#[tokio::main]` in main.rs
- Avoid blocking calls in async context — use async alternatives
- Profile async code for performance regressions

## 7. Logging

- Use **tracing** for structured logging
- Log at appropriate levels: `ERROR` for failures, `INFO` for significant events, `DEBUG` for details
- Include context in logs (profile_id, server_name, etc.)

```rust
tracing::info!(profile_id = %profile.id, "Starting ARK server");
tracing::error!(error = %e, "Failed to read server config");
```

## 8. Security

- Validate all inputs from frontend before use
- No `allow(unreachable_code)` or `allow(clippy::todo)`
- Use `std::process::Command` safely — never pass unsanitized user input as shell commands
- Sandbox file system access to `%APPDATA%\ArkServerManager` and ARK server directories
