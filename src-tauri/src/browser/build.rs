//! Construct browser argv from profile browser blocks (`browser-launch` spec).

use crate::profiles::ProfileBrowserBlock;
use crate::settings::BrowserFamily;

use super::BrowserError;

/// Prepared program + arguments for [`tokio::process::Command`] or [`std::process::Command`],
/// plus non-blocking warnings for the activation summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltBrowserCommand {
    pub program: String,
    pub args: Vec<String>,
    pub warnings: Vec<String>,
}

impl BuiltBrowserCommand {
    pub fn argv_for_spawn(&self) -> (&str, &[String]) {
        (&self.program, &self.args)
    }
}

/// Chromium-like: `--new-window`, optional `--user-data-dir=…`, then trailing URLs.
pub fn build_chromium_like_command(
    block: &ProfileBrowserBlock,
) -> Result<BuiltBrowserCommand, BrowserError> {
    if block.family != BrowserFamily::ChromiumLike {
        return Err(BrowserError::Message(format!(
            "expected chromium_like browser family, got {:?}",
            block.family
        )));
    }
    if block.executable.trim().is_empty() {
        return Err(BrowserError::Message(
            "browser executable must not be empty".into(),
        ));
    }

    let mut args = vec!["--new-window".to_string()];
    if let Some(dir) = &block.user_data_dir {
        let t = dir.trim();
        if !t.is_empty() {
            args.push(format!("--user-data-dir={t}"));
        }
    }
    for url in &block.urls {
        args.push(url.clone());
    }

    Ok(BuiltBrowserCommand {
        program: block.executable.clone(),
        args,
        warnings: vec![],
    })
}

#[cfg(test)]
mod chromium_tests {
    use super::*;
    use crate::profiles::ProfileBrowserBlock;

    fn chrome_block(
        exe: &str,
        user_data: Option<&str>,
        urls: Vec<&str>,
    ) -> ProfileBrowserBlock {
        ProfileBrowserBlock {
            family: BrowserFamily::ChromiumLike,
            executable: exe.into(),
            user_data_dir: user_data.map(String::from),
            firefox_profile: None,
            firefox_no_remote: None,
            urls: urls.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    #[test]
    fn chromium_new_window_user_data_dir_then_urls() {
        let b = chrome_block(
            "chromium",
            Some("/tmp/iso"),
            vec!["https://a.example", "https://b.example"],
        );
        let cmd = build_chromium_like_command(&b).unwrap();
        assert_eq!(cmd.program, "chromium");
        assert_eq!(
            cmd.args,
            vec![
                "--new-window".to_string(),
                "--user-data-dir=/tmp/iso".to_string(),
                "https://a.example".into(),
                "https://b.example".into(),
            ]
        );
    }

    #[test]
    fn chromium_skips_empty_user_data_segment_but_preserves_new_window() {
        let b = chrome_block("google-chrome", Some("   "), vec!["https://x"]);
        let cmd = build_chromium_like_command(&b).unwrap();
        assert_eq!(
            cmd.args,
            vec![
                "--new-window".to_string(),
                "https://x".to_string(),
            ]
        );
    }
}
