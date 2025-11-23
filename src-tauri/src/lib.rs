// Tauri IPC modules
mod commands;
mod error;
mod types;

// Re-export types for use in tests
pub use commands::*;
pub use error::*;
pub use types::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_profiles,
            commands::get_profile,
            commands::get_active_profile,
            commands::create_profile,
            commands::delete_profile,
            commands::activate_profile,
            commands::get_frameworks,
            commands::get_plugins,
            commands::get_themes,
            commands::get_prompt_engines,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
