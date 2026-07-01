use serde::{Deserialize, Serialize};

/// Latest settings schema written by this build.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Highest settings `schema_version` this build can load without migration.
pub const SUPPORTED_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BrowserFamily {
    ChromiumLike,
    Firefox,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LogVerbosity {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UiTheme {
    System,
    Light,
    Dark,
}

/// Global application settings persisted as JSON (see `application-settings` spec).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ApplicationSettings {
    pub schema_version: u32,
    /// Root directory for session profile JSON files.
    pub profiles_root: String,
    /// Absolute path or name resolved on `PATH` when launching the default browser.
    pub default_browser_executable: Option<String>,
    pub default_browser_family: Option<BrowserFamily>,
    pub logging_verbosity: LogVerbosity,
    pub theme: UiTheme,
    /// When true, UI may offer “running apps” assistance in the profile editor (`assisted-profile-capture` spec).
    #[serde(default)]
    pub assisted_profile_capture_enabled: bool,
}

impl ApplicationSettings {
    pub fn profiles_root_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.profiles_root)
    }

    pub fn is_schema_supported(&self) -> bool {
        self.schema_version <= SUPPORTED_SCHEMA_VERSION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_with_snake_case_fields() {
        let settings = ApplicationSettings {
            schema_version: CURRENT_SCHEMA_VERSION,
            profiles_root: "/home/user/.local/share/maestro/profiles".into(),
            default_browser_executable: Some("firefox".into()),
            default_browser_family: Some(BrowserFamily::Firefox),
            logging_verbosity: LogVerbosity::Info,
            theme: UiTheme::System,
            assisted_profile_capture_enabled: false,
        };

        let json = serde_json::to_string(&settings).expect("serialize");
        assert!(json.contains("\"schema_version\":1"));
        assert!(json.contains("\"profiles_root\""));
        assert!(json.contains("\"default_browser_executable\""));
        assert!(json.contains("\"logging_verbosity\":\"info\""));
        assert!(json.contains("\"theme\":\"system\""));
        assert!(json.contains("\"assisted_profile_capture_enabled\":false"));
    }

    #[test]
    fn round_trips_from_json() {
        let raw = r#"{
            "schema_version": 1,
            "profiles_root": "/tmp/maestro-profiles",
            "default_browser_executable": null,
            "default_browser_family": "chromium_like",
            "logging_verbosity": "warn",
            "theme": "dark"
        }"#;

        let settings: ApplicationSettings = serde_json::from_str(raw).expect("deserialize");
        assert_eq!(settings.schema_version, 1);
        assert_eq!(settings.profiles_root, "/tmp/maestro-profiles");
        assert_eq!(
            settings.default_browser_family,
            Some(BrowserFamily::ChromiumLike)
        );
        assert_eq!(settings.logging_verbosity, LogVerbosity::Warn);
        assert_eq!(settings.theme, UiTheme::Dark);
        assert!(!settings.assisted_profile_capture_enabled);
    }

    #[test]
    fn deserializes_settings_without_assistant_flag_defaults_false() {
        let raw = r#"{
            "schema_version": 1,
            "profiles_root": "/tmp/p",
            "default_browser_executable": null,
            "default_browser_family": null,
            "logging_verbosity": "info",
            "theme": "system"
        }"#;
        let s: ApplicationSettings = serde_json::from_str(raw).expect("ok");
        assert!(!s.assisted_profile_capture_enabled);
    }
}
