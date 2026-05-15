mod activation;
mod browser;
mod cleanup;
mod platform;
mod profiles;
mod settings;

use platform::LinuxPlatform;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, platform_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
