use std::fs;
use std::path::{Path, PathBuf};

use super::defaults::default_settings;
use super::model::{ApplicationSettings, SUPPORTED_SCHEMA_VERSION};
use super::SettingsError;

const SETTINGS_FILE_NAME: &str = "settings.json";

/// Resolved path for global settings (`$XDG_CONFIG_HOME/maestro/settings.json`).
pub fn settings_file_path() -> Result<PathBuf, SettingsError> {
    dirs::config_dir()
        .map(|base| base.join(super::defaults::APP_DIR_NAME).join(SETTINGS_FILE_NAME))
        .ok_or_else(|| {
            SettingsError::DataDirUnavailable(
                "could not resolve config directory (XDG_CONFIG_HOME / ~/.config)".into(),
            )
        })
}

/// Loads and saves `ApplicationSettings` on disk.
#[derive(Debug, Clone)]
pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn open_default() -> Result<Self, SettingsError> {
        Ok(Self::new(settings_file_path()?))
    }

    #[allow(dead_code)] // Used by unit tests; keep for callers that need the resolved path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Load from disk, or return factory defaults when the file is missing (not persisted).
    pub fn load(&self) -> Result<ApplicationSettings, SettingsError> {
        if !self.path.exists() {
            return default_settings();
        }

        let raw = fs::read_to_string(&self.path).map_err(SettingsError::Io)?;
        let settings: ApplicationSettings =
            serde_json::from_str(&raw).map_err(|e| SettingsError::InvalidJson(e.to_string()))?;

        if !settings.is_schema_supported() {
            return Err(SettingsError::UnsupportedSchemaVersion {
                found: settings.schema_version,
                supported: SUPPORTED_SCHEMA_VERSION,
            });
        }

        Ok(settings)
    }

    /// Atomic write (temp file + rename).
    pub fn save(&self, settings: &ApplicationSettings) -> Result<(), SettingsError> {
        if !settings.is_schema_supported() {
            return Err(SettingsError::UnsupportedSchemaVersion {
                found: settings.schema_version,
                supported: SUPPORTED_SCHEMA_VERSION,
            });
        }

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(SettingsError::Io)?;
        }

        let json =
            serde_json::to_string_pretty(settings).map_err(|e| SettingsError::Message(e.to_string()))?;
        let tmp_path = self.path.with_extension("json.tmp");
        fs::write(&tmp_path, json).map_err(SettingsError::Io)?;
        fs::rename(&tmp_path, &self.path).map_err(SettingsError::Io)?;
        Ok(())
    }

    /// Persist only when no settings file exists yet (first-run / profile-access path).
    pub fn save_if_absent(&self, settings: &ApplicationSettings) -> Result<bool, SettingsError> {
        if self.path.exists() {
            return Ok(false);
        }
        self.save(settings)?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::model::{UiTheme, CURRENT_SCHEMA_VERSION};
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_store() -> SettingsStore {
        let n = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("maestro-settings-test-{n}.json"));
        let _ = fs::remove_file(&path);
        SettingsStore::new(path)
    }

    #[test]
    fn load_missing_file_returns_defaults_without_creating_file() {
        let store = temp_store();
        let settings = store.load().expect("load defaults");
        assert_eq!(settings.schema_version, CURRENT_SCHEMA_VERSION);
        assert!(!store.path().exists());
    }

    #[test]
    fn save_then_load_round_trips() {
        let store = temp_store();
        let mut settings = default_settings().expect("defaults");
        settings.theme = UiTheme::Dark;

        store.save(&settings).expect("save");
        assert!(store.path().exists());

        let loaded = store.load().expect("load");
        assert_eq!(loaded, settings);
    }

    #[test]
    fn rejects_unsupported_schema_version_on_load() {
        let store = temp_store();
        let parent = store.path().parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        fs::write(
            store.path(),
            r#"{"schema_version":99,"profiles_root":"/tmp/p","theme":"system"}"#,
        )
        .unwrap();

        let err = store.load().unwrap_err();
        assert!(matches!(
            err,
            SettingsError::UnsupportedSchemaVersion { found: 99, .. }
        ));
    }

    #[test]
    fn save_if_absent_writes_once() {
        let store = temp_store();
        let settings = default_settings().expect("defaults");

        assert!(store.save_if_absent(&settings).expect("first save"));
        assert!(!store.save_if_absent(&settings).expect("second attempt"));

        let loaded = store.load().expect("load persisted");
        assert_eq!(loaded.profiles_root, settings.profiles_root);
    }

    #[test]
    fn load_legacy_logging_and_browser_defaults_and_save_omits_them() {
        let store = temp_store();
        let parent = store.path().parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        fs::write(
            store.path(),
            r#"{
                "schema_version": 1,
                "profiles_root": "/tmp/maestro-legacy-profiles",
                "default_browser_executable": "/usr/bin/firefox",
                "default_browser_family": "chromium_like",
                "logging_verbosity": "info",
                "theme": "system",
                "assisted_profile_capture_enabled": false
            }"#,
        )
        .unwrap();

        let loaded = store.load().expect("load legacy").normalize();
        assert_eq!(loaded.profiles_root, "/tmp/maestro-legacy-profiles");

        store.save(&loaded).expect("save");
        let written = fs::read_to_string(store.path()).expect("read saved");
        assert!(
            !written.contains("logging_verbosity"),
            "saved settings must omit legacy logging_verbosity, got: {written}"
        );
        assert!(
            !written.contains("assisted_profile_capture_enabled"),
            "saved settings must omit legacy assisted flag, got: {written}"
        );
        assert!(
            !written.contains("default_browser_executable"),
            "saved settings must omit legacy browser executable, got: {written}"
        );
        assert!(
            !written.contains("default_browser_family"),
            "saved settings must omit legacy browser family, got: {written}"
        );
    }
}
