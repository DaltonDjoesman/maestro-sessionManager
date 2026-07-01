//! Discover, load, validate, and persist session profiles under the configured root.

use std::fs;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use super::model::{SessionProfile, CURRENT_PROFILE_SCHEMA_VERSION};
use super::ProfileError;

/// One row for the profile catalog (valid profile or invalid JSON on disk).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileCatalogEntry {
    pub file_path: String,
    pub file_name: String,
    pub valid: bool,
    pub session_id: Option<String>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// When `valid`, number of application launch rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications_count: Option<u32>,
    /// When `valid`, whether the profile launches browser only (browser set and no apps).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_only: Option<bool>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileResult {
    pub file_path: String,
    pub profile: SessionProfile,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateProfileResult {
    pub file_path: String,
    pub profile: SessionProfile,
}

pub struct ProfileDirectory {
    root: PathBuf,
}

impl ProfileDirectory {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn catalog(&self) -> Result<Vec<ProfileCatalogEntry>, ProfileError> {
        if !self.root.exists() {
            return Ok(vec![]);
        }
        if !self.root.is_dir() {
            return Err(ProfileError::RootUnavailable(format!(
                "{} is not a directory",
                self.root.display()
            )));
        }

        let mut entries = Vec::new();
        for dir_entry in fs::read_dir(&self.root)? {
            let dir_entry = dir_entry?;
            let path = dir_entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
                continue;
            };
            if !ext.eq_ignore_ascii_case("json") {
                continue;
            }

            let file_path = path.to_string_lossy().into_owned();
            let file_name = path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();

            let bytes = match fs::read(&path) {
                Ok(b) => b,
                Err(e) => {
                    entries.push(ProfileCatalogEntry {
                        file_path: file_path.clone(),
                        file_name: file_name.clone(),
                        valid: false,
                        session_id: None,
                        name: None,
                        error: Some(format!("read {}: {e}", path.display())),
                        applications_count: None,
                        browser_only: None,
                    });
                    continue;
                }
            };

            match parse_and_validate(&file_path, &bytes) {
                Ok(profile) => {
                    let browser_only = profile.browser.is_some() && profile.applications.is_empty();
                    let applications_count = profile.applications.len() as u32;
                    entries.push(ProfileCatalogEntry {
                        file_path: file_path.clone(),
                        file_name,
                        valid: true,
                        session_id: Some(profile.session_id.clone()),
                        name: Some(profile.name.clone()),
                        error: None,
                        applications_count: Some(applications_count),
                        browser_only: Some(browser_only),
                    })
                }
                Err(e) => entries.push(ProfileCatalogEntry {
                    file_path: file_path.clone(),
                    file_name,
                    valid: false,
                    session_id: None,
                    name: None,
                    error: Some(e.to_string()),
                    applications_count: None,
                    browser_only: None,
                }),
            }
        }

        entries.sort_by(|a, b| a.file_name.cmp(&b.file_name));
        Ok(entries)
    }

    pub fn load_file(&self, user_path: &str) -> Result<SessionProfile, ProfileError> {
        let path = self.resolve_existing(user_path)?;
        let bytes = fs::read(&path)?;
        let display = path.to_string_lossy().into_owned();
        parse_and_validate(&display, &bytes)
    }

    pub fn save_file(&self, user_path: &str, profile: &SessionProfile) -> Result<(), ProfileError> {
        let path = self.resolve_under_root(user_path, true)?;
        profile.validate(&path.to_string_lossy())?;

        let mut to_save = profile.clone();
        to_save.schema_version = CURRENT_PROFILE_SCHEMA_VERSION;

        atomic_write_json(&path, &to_save)?;
        Ok(())
    }

    pub fn create_profile(&self) -> Result<CreateProfileResult, ProfileError> {
        fs::create_dir_all(&self.root)?;
        let profile = SessionProfile::new_blank();
        let path = self.root.join(format!("{}.json", profile.session_id));
        profile.validate(&path.to_string_lossy())?;
        atomic_write_json(&path, &profile)?;
        Ok(CreateProfileResult {
            file_path: path.to_string_lossy().into_owned(),
            profile,
        })
    }

    pub fn delete_file(&self, user_path: &str) -> Result<(), ProfileError> {
        let path = self.resolve_existing(user_path)?;
        fs::remove_file(&path)?;
        Ok(())
    }

    pub fn duplicate_file(&self, user_path: &str) -> Result<DuplicateProfileResult, ProfileError> {
        fs::create_dir_all(&self.root)?;
        let mut profile = self.load_file(user_path)?;
        profile.session_id = Uuid::new_v4().to_string();
        profile.name = format!("{} (copy)", profile.name);
        profile.schema_version = CURRENT_PROFILE_SCHEMA_VERSION;

        let dest = self.root.join(format!("{}.json", profile.session_id));
        profile.validate(&dest.to_string_lossy())?;
        atomic_write_json(&dest, &profile)?;
        Ok(DuplicateProfileResult {
            file_path: dest.to_string_lossy().into_owned(),
            profile,
        })
    }

    /// Duplicate like [`Self::duplicate_file`], but set the new profile display name explicitly.
    pub fn duplicate_file_with_display_name(
        &self,
        user_path: &str,
        display_name: &str,
    ) -> Result<DuplicateProfileResult, ProfileError> {
        fs::create_dir_all(&self.root)?;
        let name = display_name.trim();
        if name.is_empty() {
            return Err(ProfileError::Validation {
                path: user_path.to_string(),
                message: "display name must not be empty".into(),
            });
        }
        let mut profile = self.load_file(user_path)?;
        profile.session_id = Uuid::new_v4().to_string();
        profile.name = name.to_string();
        profile.schema_version = CURRENT_PROFILE_SCHEMA_VERSION;

        let dest = self.root.join(format!("{}.json", profile.session_id));
        profile.validate(&dest.to_string_lossy())?;
        atomic_write_json(&dest, &profile)?;
        Ok(DuplicateProfileResult {
            file_path: dest.to_string_lossy().into_owned(),
            profile,
        })
    }

    /// Parse JSON from another machine/editor, validate, assign a new `session_id`, set `name`, and save under this root.
    pub fn import_profile_json(&self, json: &str, display_name: &str) -> Result<DuplicateProfileResult, ProfileError> {
        fs::create_dir_all(&self.root)?;
        let name = display_name.trim();
        if name.is_empty() {
            return Err(ProfileError::Validation {
                path: "import".into(),
                message: "display name must not be empty".into(),
            });
        }

        let bytes = json.as_bytes();
        let mut profile: SessionProfile = serde_json::from_slice(bytes).map_err(|e| {
            ProfileError::InvalidJson {
                path: "import.json".into(),
                message: e.to_string(),
            }
        })?;
        profile.validate("<import>")?;
        profile.session_id = Uuid::new_v4().to_string();
        profile.name = name.to_string();
        profile.schema_version = CURRENT_PROFILE_SCHEMA_VERSION;

        let dest = self.root.join(format!("{}.json", profile.session_id));
        profile.validate(&dest.to_string_lossy())?;
        atomic_write_json(&dest, &profile)?;
        Ok(DuplicateProfileResult {
            file_path: dest.to_string_lossy().into_owned(),
            profile,
        })
    }

    fn resolve_existing(&self, user_path: &str) -> Result<PathBuf, ProfileError> {
        self.resolve_under_root(user_path, false)
    }

    /// Resolve `user_path` (absolute or relative to root).
    fn resolve_under_root(&self, user_path: &str, allow_missing: bool) -> Result<PathBuf, ProfileError> {
        let raw = Path::new(user_path.trim());
        let candidate = if raw.is_absolute() {
            raw.to_path_buf()
        } else {
            self.root.join(raw)
        };

        let root_canon = canonicalize_parent_chain(&self.root)?;
        if candidate.exists() {
            let target_canon = fs::canonicalize(&candidate)?;
            if !target_canon.starts_with(&root_canon) {
                return Err(ProfileError::PathOutsideRoot);
            }
            return Ok(target_canon);
        }

        if !allow_missing {
            return Err(ProfileError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("profile file not found: {}", candidate.display()),
            )));
        }

        let normalized = normalize_under(&root_canon, &candidate)?;
        Ok(normalized)
    }
}

fn parse_and_validate(path_label: &str, bytes: &[u8]) -> Result<SessionProfile, ProfileError> {
    let profile: SessionProfile = serde_json::from_slice(bytes).map_err(|e| ProfileError::InvalidJson {
        path: path_label.to_string(),
        message: e.to_string(),
    })?;
    profile.validate(path_label)?;
    Ok(profile)
}

fn atomic_write_json(path: &Path, profile: &SessionProfile) -> Result<(), ProfileError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(profile).map_err(|e| ProfileError::Validation {
        path: path.to_string_lossy().into_owned(),
        message: format!("serialize: {e}"),
    })?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

/// Canonical base path for containment checks (creates directory if missing).
fn canonicalize_parent_chain(root: &Path) -> Result<PathBuf, ProfileError> {
    fs::create_dir_all(root)?;
    Ok(fs::canonicalize(root)?)
}

fn normalize_under(root_canon: &Path, candidate: &Path) -> Result<PathBuf, ProfileError> {
    let joined = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root_canon.join(candidate)
    };

    let mut stack: Vec<std::ffi::OsString> = Vec::new();
    let mut has_root = false;

    for c in joined.components() {
        match c {
            std::path::Component::Prefix(_) => {}
            std::path::Component::RootDir => {
                has_root = true;
                stack.clear();
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                stack.pop();
            }
            std::path::Component::Normal(o) => stack.push(o.to_os_string()),
        }
    }

    let mut out = PathBuf::new();
    if has_root {
        out.push("/");
    }
    for seg in stack {
        out.push(seg);
    }

    if !out.starts_with(root_canon) {
        return Err(ProfileError::PathOutsideRoot);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::model::ApplicationLaunchEntry;

    fn temp_root() -> PathBuf {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("maestro-prof-{n}"))
    }

    #[test]
    fn atomic_write_round_trip() {
        let root = temp_root();
        fs::create_dir_all(&root).unwrap();
        let dir = ProfileDirectory::new(root.clone());
        let p = SessionProfile::new_blank();
        let path = root.join(format!("{}.json", p.session_id));
        atomic_write_json(&path, &p).unwrap();
        let loaded = dir.load_file(path.to_str().unwrap()).unwrap();
        assert_eq!(loaded.session_id, p.session_id);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn catalog_marks_invalid_json() {
        let root = temp_root();
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("bad.json"), b"{ not json").unwrap();
        let dir = ProfileDirectory::new(root.clone());
        let cat = dir.catalog().unwrap();
        assert_eq!(cat.len(), 1);
        assert!(!cat[0].valid);
        assert!(cat[0].error.as_ref().unwrap().contains("bad.json"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn import_profile_json_assigns_new_id() {
        let root = temp_root();
        fs::create_dir_all(&root).unwrap();
        let dir = ProfileDirectory::new(root.clone());
        let created = dir.create_profile().unwrap();
        let json = fs::read_to_string(&created.file_path).unwrap();
        let imported = dir.import_profile_json(&json, "Imported name").unwrap();
        assert_ne!(imported.profile.session_id, created.profile.session_id);
        assert_eq!(imported.profile.name, "Imported name");
        assert!(fs::metadata(&imported.file_path).is_ok());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn duplicate_creates_new_id_and_file() {
        let root = temp_root();
        fs::create_dir_all(&root).unwrap();
        let dir = ProfileDirectory::new(root.clone());
        let created = dir.create_profile().unwrap();
        let dup = dir.duplicate_file(&created.file_path).unwrap();
        assert_ne!(dup.profile.session_id, created.profile.session_id);
        assert!(dup.profile.name.ends_with("(copy)"));
        assert_ne!(created.file_path, dup.file_path);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn rejects_save_with_empty_executable() {
        let root = temp_root();
        fs::create_dir_all(&root).unwrap();
        let dir = ProfileDirectory::new(root.clone());
        let mut p = SessionProfile::new_blank();
        p.applications.push(ApplicationLaunchEntry {
            executable: "".into(),
            args: vec![],
            cwd: None,
            skip_if_running: None,
        });
        let path = root.join("test.json");
        let err = dir.save_file(path.to_str().unwrap(), &p).unwrap_err();
        assert!(matches!(err, ProfileError::Validation { .. }));
        let _ = fs::remove_dir_all(&root);
    }
}
