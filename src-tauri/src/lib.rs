mod activation;
mod browser;
mod cleanup;
mod platform;
mod process_launcher;
mod profiles;
mod settings;

use activation::ActivationStepSummary;
use cleanup::{CleanupDivergenceRow, CleanupTerminateResult};
use platform::{LinuxPlatform, PlatformContext};
use profiles::{
    CreateProfileResult, DuplicateProfileResult, ProfileCatalogEntry, ProfileDirectory,
    SessionProfile,
};

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
    manager.get()
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
fn duplicate_session_profile(
    path: String,
    manager: tauri::State<'_, SettingsManager>,
) -> Result<DuplicateProfileResult, String> {
    ProfileDirectory::new(manager.get().profiles_root_path())
        .duplicate_file(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn activate_session_profile(
    path: String,
    manager: tauri::State<'_, SettingsManager>,
) -> Result<Vec<ActivationStepSummary>, String> {
    let dir = ProfileDirectory::new(manager.get().profiles_root_path());
    let profile = dir.load_file(&path).map_err(|e| e.to_string())?;
    activation::activate_session_profile(&profile, &path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn list_cleanup_divergences(
    path: String,
    manager: tauri::State<'_, SettingsManager>,
) -> Result<Vec<CleanupDivergenceRow>, String> {
    let dir = ProfileDirectory::new(manager.get().profiles_root_path());
    let profile = dir.load_file(&path).map_err(|e| e.to_string())?;
    let platform = LinuxPlatform::new().map_err(|e| e.to_string())?;
    let candidates = platform
        .list_cleanup_process_candidates()
        .map_err(|e| e.to_string())?;
    let allowed = cleanup::allowed_executable_basenames(&profile);
    let divergent = cleanup::compute_divergences(&candidates, &allowed);
    Ok(divergent
        .iter()
        .map(CleanupDivergenceRow::from)
        .collect())
}

#[tauri::command]
async fn cleanup_terminate_processes(
    pids: Vec<u32>,
    force_kill_after_ms: Option<u64>,
) -> Result<Vec<CleanupTerminateResult>, String> {
    let mut out = Vec::with_capacity(pids.len());
    for pid in pids {
        match cleanup::terminate_process(pid, force_kill_after_ms).await {
            Ok(message) => out.push(CleanupTerminateResult {
                pid,
                ok: true,
                message,
            }),
            Err(message) => out.push(CleanupTerminateResult {
                pid,
                ok: false,
                message,
            }),
        }
    }
    Ok(out)
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
        .invoke_handler(tauri::generate_handler![
            greet,
            platform_name,
            get_settings,
            validate_profiles_root,
            save_settings,
            list_session_profiles,
            load_session_profile,
            save_session_profile,
            create_session_profile,
            delete_session_profile,
            duplicate_session_profile,
            activate_session_profile,
            list_cleanup_divergences,
            cleanup_terminate_processes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
