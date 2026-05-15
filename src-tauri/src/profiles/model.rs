use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::settings::BrowserFamily;

/// Latest profile schema written by this build.
pub const CURRENT_PROFILE_SCHEMA_VERSION: u32 = 1;

/// Highest profile `schema_version` this build can load without migration.
pub const SUPPORTED_PROFILE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ApplicationLaunchEntry {
    pub executable: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub cwd: Option<String>,
}

/// Browser launch block aligned with `browser-launch` spec (families + isolation fields).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ProfileBrowserBlock {
    pub family: BrowserFamily,
    pub executable: String,
    #[serde(default)]
    pub user_data_dir: Option<String>,
    #[serde(default)]
    pub firefox_profile: Option<String>,
    #[serde(default)]
    pub firefox_no_remote: Option<bool>,
    #[serde(default)]
    pub urls: Vec<String>,
}

/// Optional rules for context cleanup (extensible).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileCleanupRules {
    /// Extra process basenames treated as part of this session during divergence checks.
    #[serde(default)]
    pub allow_extra_basenames: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct SessionProfile {
    pub schema_version: u32,
    pub session_id: String,
    pub name: String,
    #[serde(default)]
    pub applications: Vec<ApplicationLaunchEntry>,
    #[serde(default)]
    pub browser: Option<ProfileBrowserBlock>,
    #[serde(default)]
    pub cleanup: Option<ProfileCleanupRules>,
}

impl SessionProfile {
    pub fn new_blank() -> Self {
        Self {
            schema_version: CURRENT_PROFILE_SCHEMA_VERSION,
            session_id: Uuid::new_v4().to_string(),
            name: "New session".into(),
            applications: vec![],
            browser: None,
            cleanup: None,
        }
    }

    /// Validate structure and semantics after JSON parse (before activate / strict catalog).
    pub fn validate(&self, path_hint: &str) -> Result<(), super::ProfileError> {
        if self.schema_version == 0 {
            return Err(super::ProfileError::Validation {
                path: path_hint.to_string(),
                message: "schema_version must be a positive integer".into(),
            });
        }
        if self.schema_version > SUPPORTED_PROFILE_SCHEMA_VERSION {
            return Err(super::ProfileError::UnsupportedSchema {
                path: path_hint.to_string(),
                found: self.schema_version,
                supported: SUPPORTED_PROFILE_SCHEMA_VERSION,
            });
        }
        if self.session_id.trim().is_empty() {
            return Err(super::ProfileError::Validation {
                path: path_hint.to_string(),
                message: "session_id must not be empty".into(),
            });
        }
        if self.name.trim().is_empty() {
            return Err(super::ProfileError::Validation {
                path: path_hint.to_string(),
                message: "name must not be empty".into(),
            });
        }
        for (i, app) in self.applications.iter().enumerate() {
            if app.executable.trim().is_empty() {
                return Err(super::ProfileError::Validation {
                    path: path_hint.to_string(),
                    message: format!("applications[{i}].executable must not be empty"),
                });
            }
        }
        if let Some(browser) = &self.browser {
            if browser.executable.trim().is_empty() {
                return Err(super::ProfileError::Validation {
                    path: path_hint.to_string(),
                    message: "browser.executable must not be empty when browser block is set"
                        .into(),
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_minimal_profile() {
        let raw = r#"{
            "schema_version": 1,
            "session_id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Work",
            "applications": [
                { "executable": "cursor", "args": ["."], "cwd": "/home/u/proj" }
            ]
        }"#;
        let p: SessionProfile = serde_json::from_str(raw).expect("parse");
        assert_eq!(p.schema_version, 1);
        assert_eq!(p.applications.len(), 1);
        assert_eq!(p.applications[0].args, vec![".".to_string()]);
        p.validate("/tmp/x.json").expect("valid");
    }

    #[test]
    fn rejects_future_schema_after_parse() {
        let raw = r#"{
            "schema_version": 99,
            "session_id": "a",
            "name": "X",
            "applications": []
        }"#;
        let p: SessionProfile = serde_json::from_str(raw).expect("parse");
        let err = p.validate("/profiles/z.json").unwrap_err();
        assert!(matches!(
            err,
            crate::profiles::ProfileError::UnsupportedSchema { .. }
        ));
    }

    #[test]
    fn rejects_empty_application_executable() {
        let p = SessionProfile {
            schema_version: 1,
            session_id: "id".into(),
            name: "n".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: " ".into(),
                args: vec![],
                cwd: None,
            }],
            browser: None,
            cleanup: None,
        };
        assert!(p.validate("/p.json").is_err());
    }
}
