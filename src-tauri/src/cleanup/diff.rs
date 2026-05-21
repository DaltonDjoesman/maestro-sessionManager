//! Allowed executable set and divergence list vs running processes (`context-cleanup` spec).

use std::collections::HashSet;

use crate::activation::skip::executable_basename;
use crate::platform::ProcessCandidate;
use crate::profiles::SessionProfile;

/// Lowercased basenames implied by the profile (applications, browser, cleanup extras).
pub fn allowed_executable_basenames(profile: &SessionProfile) -> HashSet<String> {
    let mut allowed = HashSet::new();
    for app in &profile.applications {
        allowed.insert(executable_basename(&app.executable));
    }
    if let Some(browser) = profile.browser.as_ref() {
        allowed.insert(executable_basename(&browser.executable));
    }
    if let Some(cleanup) = profile.cleanup.as_ref() {
        for extra in &cleanup.allow_extra_basenames {
            let t = extra.trim();
            if !t.is_empty() {
                allowed.insert(t.to_lowercase());
            }
        }
    }
    allowed
}

/// Running processes whose executable basename is not in the profile-allowed set.
pub fn compute_divergences(
    candidates: &[ProcessCandidate],
    allowed: &HashSet<String>,
) -> Vec<ProcessCandidate> {
    candidates
        .iter()
        .filter(|c| !allowed.contains(&c.executable_basename))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{
        ApplicationLaunchEntry, ProfileBrowserBlock, ProfileCleanupRules, SessionProfile,
    };
    use crate::settings::BrowserFamily;

    fn sample_profile() -> SessionProfile {
        SessionProfile {
            schema_version: 1,
            session_id: "s".into(),
            name: "N".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: "/usr/bin/gedit".into(),
                args: vec![],
                cwd: None,
                skip_if_running: None,
            }],
            browser: Some(ProfileBrowserBlock {
                family: BrowserFamily::ChromiumLike,
                executable: "/snap/bin/chromium".into(),
                user_data_dir: None,
                firefox_profile: None,
                firefox_no_remote: None,
                urls: vec![],
            }),
            cleanup: Some(ProfileCleanupRules {
                allow_extra_basenames: vec!["dockerd".into()],
            }),
        }
    }

    #[test]
    fn allowed_contains_app_browser_and_cleanup_extras() {
        let p = sample_profile();
        let a = allowed_executable_basenames(&p);
        assert!(a.contains("gedit"));
        assert!(a.contains("chromium"));
        assert!(a.contains("dockerd"));
    }

    #[test]
    fn divergences_exclude_allowed_basenames() {
        let p = sample_profile();
        let allowed = allowed_executable_basenames(&p);
        let candidates = vec![
            ProcessCandidate {
                pid: 10,
                executable_basename: "random-app".into(),
                cmd_preview: "random-app".into(),
            },
            ProcessCandidate {
                pid: 11,
                executable_basename: "gedit".into(),
                cmd_preview: "gedit".into(),
            },
        ];
        let div = compute_divergences(&candidates, &allowed);
        assert_eq!(div.len(), 1);
        assert_eq!(div[0].pid, 10);
    }
}
