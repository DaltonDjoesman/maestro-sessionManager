mod activation;
mod browser;
mod capture;
mod clipboard;
mod editors;
mod platform;
mod process_launcher;
mod profiles;
mod settings;

use activation::{ActivateSessionResult, ActivationPreviewStep};
use profiles::{
    CreateProfileResult, DuplicateProfileResult, ProfileCatalogEntry, ProfileDirectory,
    SessionProfile,
};

use browser::SystemDefaultBrowserHint;
use capture::RunningAppCandidate;
use settings::{ApplicationSettings, SettingsManager};
use tauri::Manager;

#[tauri::command]
fn get_settings(manager: tauri::State<'_, SettingsManager>) -> ApplicationSettings {
    manager.get()
}

#[tauri::command]
fn read_clipboard_text() -> Result<String, String> {
    clipboard::read_text()
}

#[tauri::command]
fn detect_system_default_browser() -> SystemDefaultBrowserHint {
    browser::detect_system_default_browser()
}

#[tauri::command]
fn validate_profiles_root(path: String) -> Result<(), String> {
    settings::validate_profiles_root_path(std::path::Path::new(path.trim()))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn save_settings(
    settings: ApplicationSettings,
    manager: tauri::State<'_, SettingsManager>,
) -> Result<ApplicationSettings, String> {
    manager.update_and_save(settings).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_assistant_running_apps() -> Result<Vec<RunningAppCandidate>, String> {
    Ok(capture::list_running_app_candidates())
}

#[tauri::command]
fn list_session_profiles(
    manager: tauri::State<'_, SettingsManager>,
) -> Result<Vec<ProfileCatalogEntry>, String> {
    let dir = ProfileDirectory::new(manager.get().profiles_root_path());
    let out = dir.catalog().map_err(|e| e.to_string())?;
    let _ = manager.persist_if_absent();
    Ok(out)
}

#[tauri::command]
fn load_session_profile(
    path: String,
    manager: tauri::State<'_, SettingsManager>,
) -> Result<SessionProfile, String> {
    ProfileDirectory::new(manager.get().profiles_root_path())
        .load_file(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn save_session_profile(
    path: String,
    profile: SessionProfile,
    manager: tauri::State<'_, SettingsManager>,
) -> Result<(), String> {
    ProfileDirectory::new(manager.get().profiles_root_path())
        .save_file(&path, &profile)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn create_session_profile(
    manager: tauri::State<'_, SettingsManager>,
) -> Result<CreateProfileResult, String> {
    let dir = ProfileDirectory::new(manager.get().profiles_root_path());
    let out = dir.create_profile().map_err(|e| e.to_string())?;
    let _ = manager.persist_if_absent();
    Ok(out)
}

#[tauri::command]
fn delete_session_profile(
    path: String,
    manager: tauri::State<'_, SettingsManager>,
) -> Result<(), String> {
    ProfileDirectory::new(manager.get().profiles_root_path())
        .delete_file(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn duplicate_session_profile_with_name(
    path: String,
    display_name: String,
    manager: tauri::State<'_, SettingsManager>,
) -> Result<DuplicateProfileResult, String> {
    ProfileDirectory::new(manager.get().profiles_root_path())
        .duplicate_file_with_display_name(&path, &display_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn import_session_profile_json(
    json: String,
    display_name: String,
    manager: tauri::State<'_, SettingsManager>,
) -> Result<DuplicateProfileResult, String> {
    ProfileDirectory::new(manager.get().profiles_root_path())
        .import_profile_json(&json, &display_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn preview_session_activation(
    path: String,
    profile: SessionProfile,
) -> Result<Vec<ActivationPreviewStep>, String> {
    activation::preview_session_activation(&profile, &path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn activate_session_profile(
    path: String,
    profile: SessionProfile,
    _manager: tauri::State<'_, SettingsManager>,
) -> Result<ActivateSessionResult, String> {
    activation::activate_session_profile(&profile, &path)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // WebKitGTK may probe GStreamer DMA formats on Linux; disabling DMABUF avoids known
    // gst-plugin-scanner assertion noise and blank-window issues (Tauri Linux graphics guide).
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let manager = SettingsManager::open_default().map_err(|e| {
                Box::new(e) as Box<dyn std::error::Error>
            })?;
            app.manage(manager);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            read_clipboard_text,
            detect_system_default_browser,
            validate_profiles_root,
            save_settings,
            list_assistant_running_apps,
            list_session_profiles,
            load_session_profile,
            save_session_profile,
            create_session_profile,
            delete_session_profile,
            duplicate_session_profile_with_name,
            import_session_profile_json,
            preview_session_activation,
            activate_session_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
