use std::fs;
use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProfilesRootValidationError {
    #[error("profiles path cannot be empty")]
    Empty,
    #[error("profiles path is not a directory")]
    NotADirectory,
    #[error("profiles directory is not writable")]
    NotWritable,
    #[error("parent directory does not exist or is not writable")]
    ParentNotWritable,
}

/// Validates the profiles root before save (spec: must be a writable directory).
pub fn validate_profiles_root_path(path: &Path) -> Result<(), ProfilesRootValidationError> {
    let trimmed = path.to_string_lossy().trim().to_string();
    if trimmed.is_empty() {
        return Err(ProfilesRootValidationError::Empty);
    }

    let path = Path::new(&trimmed);

    if path.exists() {
        if !path.is_dir() {
            return Err(ProfilesRootValidationError::NotADirectory);
        }
        if !is_directory_writable(path) {
            return Err(ProfilesRootValidationError::NotWritable);
        }
        return Ok(());
    }

    let Some(parent) = path.parent() else {
        return Err(ProfilesRootValidationError::ParentNotWritable);
    };

    if !parent.exists() || !parent.is_dir() || !is_directory_writable(parent) {
        return Err(ProfilesRootValidationError::ParentNotWritable);
    }

    Ok(())
}

fn is_directory_writable(dir: &Path) -> bool {
    let probe = dir.join(".maestro_write_probe");
    match fs::File::create(&probe) {
        Ok(file) => {
            drop(file);
            let _ = fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let n = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("maestro-validate-{name}-{n}"))
    }

    #[test]
    fn rejects_empty_path() {
        let err = validate_profiles_root_path(Path::new("  ")).unwrap_err();
        assert_eq!(err, ProfilesRootValidationError::Empty);
    }

    #[test]
    fn accepts_existing_writable_directory() {
        let dir = temp_dir("writable");
        fs::create_dir_all(&dir).unwrap();
        validate_profiles_root_path(&dir).expect("writable dir");
    }

    #[test]
    fn accepts_missing_directory_when_parent_is_writable() {
        let parent = temp_dir("parent");
        fs::create_dir_all(&parent).unwrap();
        let child = parent.join("profiles");
        validate_profiles_root_path(&child).expect("creatable path");
    }

    #[test]
    fn rejects_file_path() {
        let file = temp_dir("file");
        fs::write(&file, b"x").unwrap();
        let err = validate_profiles_root_path(&file).unwrap_err();
        assert_eq!(err, ProfilesRootValidationError::NotADirectory);
    }

    #[test]
    #[cfg(unix)]
    fn rejects_non_writable_directory() {
        let dir = temp_dir("readonly");
        fs::create_dir_all(&dir).unwrap();
        let mut perms = fs::metadata(&dir).unwrap().permissions();
        perms.set_mode(0o555);
        fs::set_permissions(&dir, perms).unwrap();

        let err = validate_profiles_root_path(&dir).unwrap_err();
        assert_eq!(err, ProfilesRootValidationError::NotWritable);

        let mut perms = fs::metadata(&dir).unwrap().permissions();
        perms.set_mode(0o755);
        let _ = fs::set_permissions(&dir, perms);
    }
}
