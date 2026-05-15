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

/// Dispatch by [`BrowserFamily`] for session activation.
pub fn build_browser_launch(block: &ProfileBrowserBlock) -> Result<BuiltBrowserCommand, BrowserError> {
    let (program, args) = match block.family {
        BrowserFamily::ChromiumLike => chromium_argv(block)?,
        BrowserFamily::Firefox => firefox_argv(block)?,
    };
    Ok(BuiltBrowserCommand {
        program,
        args,
        warnings: vec![],
    })
}

fn chromium_argv(block: &ProfileBrowserBlock) -> Result<(String, Vec<String>), BrowserError> {
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

    Ok((block.executable.clone(), args))
}

/// Chromium-like: `--new-window`, optional `--user-data-dir=…`, then trailing URLs.
pub fn build_chromium_like_command(
    block: &ProfileBrowserBlock,
) -> Result<BuiltBrowserCommand, BrowserError> {
    let (program, args) = chromium_argv(block)?;
    Ok(BuiltBrowserCommand {
        program,
        args,
        warnings: vec![],
    })
}

fn firefox_argv(block: &ProfileBrowserBlock) -> Result<(String, Vec<String>), BrowserError> {
    if block.family != BrowserFamily::Firefox {
        return Err(BrowserError::Message(format!(
            "expected firefox browser family, got {:?}",
            block.family
        )));
    }
    if block.executable.trim().is_empty() {
        return Err(BrowserError::Message(
            "browser executable must not be empty".into(),
        ));
    }

    let mut args = vec!["-new-window".to_string()];
    if let Some(p) = &block.firefox_profile {
        let t = p.trim();
        if !t.is_empty() {
            args.push("-P".to_string());
            args.push(t.to_string());
        }
    }
    if block.firefox_no_remote == Some(true) {
        args.push("-no-remote".to_string());
    }
    for url in &block.urls {
        args.push(url.clone());
    }

    Ok((block.executable.clone(), args))
}

/// Firefox: `-new-window`, optional `-P` profile, optional `-no-remote`, trailing URLs.
pub fn build_firefox_command(
    block: &ProfileBrowserBlock,
) -> Result<BuiltBrowserCommand, BrowserError> {
    let (program, args) = firefox_argv(block)?;
    Ok(BuiltBrowserCommand {
        program,
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
                "https://a.example".to_string(),
                "https://b.example".to_string(),
            ]
        );
    }

    #[test]
    fn chromium_skips_empty_user_data_segment_but_preserves_new_window() {
        let b = chrome_block("google-chrome", Some("   "), vec!["https://x"]);
        let cmd = build_chromium_like_command(&b).unwrap();
        assert_eq!(
            cmd.args,
            vec!["--new-window".to_string(), "https://x".to_string(),]
        );
    }

    #[test]
    fn build_browser_launch_routes_chromium() {
        let b = chrome_block("chrome", None, vec!["https://z"]);
        let cmd = build_browser_launch(&b).unwrap();
        assert_eq!(cmd.program, "chrome");
        assert!(cmd.args[0] == "--new-window");
        assert_eq!(cmd.args[cmd.args.len() - 1], "https://z");
    }
}

#[cfg(test)]
mod firefox_tests {
    use super::*;
    use crate::profiles::ProfileBrowserBlock;

    fn ff_block(
        exe: &str,
        profile: Option<&str>,
        no_remote: Option<bool>,
        urls: Vec<&str>,
    ) -> ProfileBrowserBlock {
        ProfileBrowserBlock {
            family: BrowserFamily::Firefox,
            executable: exe.into(),
            user_data_dir: None,
            firefox_profile: profile.map(String::from),
            firefox_no_remote: no_remote,
            urls: urls.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    #[test]
    fn firefox_new_window_profile_no_remote_then_urls() {
        let b = ff_block(
            "firefox",
            Some("MaestroSession"),
            Some(true),
            vec!["https://a.example", "https://b.example"],
        );
        let cmd = build_firefox_command(&b).unwrap();
        assert_eq!(cmd.program, "firefox");
        assert_eq!(
            cmd.args,
            vec![
                "-new-window".to_string(),
                "-P".to_string(),
                "MaestroSession".to_string(),
                "-no-remote".to_string(),
                "https://a.example".to_string(),
                "https://b.example".to_string(),
            ]
        );
    }

    #[test]
    fn firefox_omits_profile_and_no_remote_when_not_set() {
        let b = ff_block("firefox", None, None, vec!["https://only"]);
        let cmd = build_firefox_command(&b).unwrap();
        assert_eq!(
            cmd.args,
            vec!["-new-window".to_string(), "https://only".to_string()]
        );
    }

    #[test]
    fn firefox_skips_empty_profile_string() {
        let b = ff_block("firefox", Some("  "), Some(false), vec![]);
        let cmd = build_firefox_command(&b).unwrap();
        assert_eq!(cmd.args, vec!["-new-window".to_string()]);
    }

    #[test]
    fn build_browser_launch_routes_firefox() {
        let b = ff_block("firefox", Some("p"), None, vec!["https://x"]);
        let cmd = build_browser_launch(&b).unwrap();
        assert!(cmd.args.contains(&"-P".to_string()));
        assert!(cmd.args.ends_with(&["https://x".to_string()]));
    }
}
