//! Pre-spawn profile validation (`session-activation` spec).

use crate::profiles::SessionProfile;

use super::ActivationError;

/// Run all checks that must pass before any browser or application spawn.
pub fn validate_before_activation(profile: &SessionProfile, path_label: &str) -> Result<(), ActivationError> {
    profile.validate(path_label)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::ApplicationLaunchEntry;

    #[test]
    fn rejects_profile_before_spawn_semantics() {
        let p = SessionProfile {
            schema_version: 1,
            session_id: "id".into(),
            name: "n".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: "".into(),
                args: vec![],
                cwd: None,
                skip_if_running: None,
                browser: None,
            }],
            browser: None,
            cleanup: None,
        };
        assert!(validate_before_activation(&p, "/x.json").is_err());
    }

    #[test]
    fn accepts_valid_minimal_profile() {
        let p = SessionProfile {
            schema_version: 1,
            session_id: "sid".into(),
            name: "N".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: "/bin/true".into(),
                args: vec![],
                cwd: None,
                skip_if_running: None,
                browser: None,
            }],
            browser: None,
            cleanup: None,
        };
        validate_before_activation(&p, "/ok.json").expect("valid");
    }
}
