use std::path::PathBuf;

use super::model::{
    ApplicationSettings, LogVerbosity, UiTheme, CURRENT_SCHEMA_VERSION,
};
use super::SettingsError;

pub(crate) const APP_DIR_NAME: &str = "maestro";
const PROFILES_DIR_NAME: &str = "profiles";

/// Per-user Maestro data directory (`$XDG_DATA_HOME/maestro` or `~/.local/share/maestro`).
pub fn maestro_data_dir() -> Result<PathBuf, SettingsError> {
    dirs::data_local_dir()
        .map(|base| base.join(APP_DIR_NAME))
        .ok_or_else(|| {
            SettingsError::DataDirUnavailable(
                "could not resolve a local data directory (XDG_DATA_HOME / ~/.local/share)"
                    .into(),
            )
        })
}

/// Default profiles root: `<maestro_data_dir>/profiles`.
pub fn default_profiles_root() -> Result<PathBuf, SettingsError> {
    Ok(maestro_data_dir()?.join(PROFILES_DIR_NAME))
}

/// Factory defaults for first run (paths only; persistence is task 2.2).
pub fn default_settings() -> Result<ApplicationSettings, SettingsError> {
    let profiles_root = default_profiles_root()?;
    Ok(ApplicationSettings {
        schema_version: CURRENT_SCHEMA_VERSION,
        profiles_root: profiles_root.to_string_lossy().into_owned(),
        logging_verbosity: LogVerbosity::Info,
        theme: UiTheme::System,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::model::CURRENT_SCHEMA_VERSION;

    #[test]
    fn default_settings_use_current_schema_version() {
        let settings = default_settings().expect("defaults");
        assert_eq!(settings.schema_version, CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn default_profiles_root_lives_under_maestro_data_dir() {
        let data_dir = maestro_data_dir().expect("data dir");
        let profiles = default_profiles_root().expect("profiles root");
        assert_eq!(profiles, data_dir.join("profiles"));
    }

    #[test]
    fn default_settings_profiles_root_is_absolute() {
        let settings = default_settings().expect("defaults");
        let path = settings.profiles_root_path();
        assert!(path.is_absolute());
        assert!(settings.profiles_root.ends_with("maestro/profiles"));
    }
}
