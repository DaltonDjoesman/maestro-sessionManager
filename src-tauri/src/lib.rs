mod activation;
mod browser;
mod cleanup;
mod platform;
mod profiles;
mod settings;

use platform::{LinuxPlatform, PlatformContext};
use settings::{ApplicationSettings, SettingsManager};
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! Maestro core is ready.")
}

#[tauri::command]
fn platform_name() -> String {
    LinuxPlatform::new()
        .map(|p| p.platform_name().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

#[tauri::command]
fn get_settings(manager: tauri::State<'_, SettingsManager>) -> ApplicationSettings {
    manager.settings.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let manager = SettingsManager::open_default().map_err(|e| {
                Box::new(e) as Box<dyn std::error::Error>
            })?;
            app.manage(manager);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, platform_name, get_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
