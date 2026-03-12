// VibeStock Desktop — Tauri v2 backend entry point
//
// Exposes two Tauri commands to the Svelte frontend:
//   - get_config  : returns API base URL and app version
//   - open_url    : opens a URL in the system browser via tauri-plugin-opener
//
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;

/// Runtime configuration sent to the Svelte frontend.
#[derive(Debug, Serialize)]
pub struct AppConfig {
    pub api_base_url: String,
    pub app_version:  String,
}

/// Expose config to frontend — API URL can be overridden via VIBESTOCK_API_URL env var.
#[tauri::command]
fn get_config() -> AppConfig {
    AppConfig {
        api_base_url: std::env::var("VIBESTOCK_API_URL")
            .unwrap_or_else(|_| "http://localhost:3000".into()),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

// =============================================================================
// Application entry point
// =============================================================================
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_config])
        .run(tauri::generate_context!())
        .expect("Error while running VibeStock desktop application");
}

fn main() {
    run();
}
