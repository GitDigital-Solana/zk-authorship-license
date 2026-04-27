src-tauri/src/main.rs

```rust
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::register::analyze_and_register,
            commands::license::issue_license,
            commands::prove::prove_license,
        ])
        .setup(|app| {
            let db_path = app.path_resolver().app_data_dir().unwrap().join("zk_app.db");
            db::initialize(&db_path).expect("Failed to init DB");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```
