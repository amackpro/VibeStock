use serde::{Deserialize, Serialize};

/// Configuration injected into the Svelte frontend via Tauri commands
#[derive(Debug, Serialize)]
pub struct AppConfig {
    pub api_base_url: String,
    pub app_version: String,
}

/// Returns runtime config to the frontend
#[tauri::command]
fn get_config() -> AppConfig {
    AppConfig {
        api_base_url: std::env::var("VIBESTOCK_API_URL")
            .unwrap_or_else(|_| "http://localhost:3000".into()),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

/// Opens an external URL in the system's default browser
#[tauri::command]
async fn open_external(url: String) -> Result<(), String> {
    open::that(&url).map_err(|e| e.to_string())
}

// =============================================================================
// Application entry point
// =============================================================================
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_config, open_external])
        .run(tauri::generate_context!())
        .expect("Error while running VibeStock desktop application");
}

fn main() {
    run();
}
