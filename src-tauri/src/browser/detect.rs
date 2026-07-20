//! Heuristic browser executable detection for profile normalization and capture.

use crate::platform::executable_basename;
use crate::settings::BrowserFamily;

/// Infer browser family from executable basename/path, when recognizable.
pub fn detect_browser_family(executable: &str) -> Option<BrowserFamily> {
    let base = executable_basename(executable);
    if base.is_empty() {
        return None;
    }

    const FIREFOX_MARKERS: &[&str] = &[
        "firefox", "librewolf", "waterfox", "floorp", "zen-browser",
    ];
    if FIREFOX_MARKERS.iter().any(|m| base.contains(m)) {
        return Some(BrowserFamily::Firefox);
    }

    const CHROMIUM_MARKERS: &[&str] = &[
        "chrome",
        "chromium",
        "vivaldi",
        "brave",
        "edge",
        "opera",
        "microsoft-edge",
        "google-chrome",
        "chromium-browser",
        "brave-browser",
        "vivaldi-bin",
        "vivaldi-stable",
        "google-chrome-stable",
        "microsoft-edge-stable",
        "thorium",
        "ungoogled-chromium",
        "x-www-browser",
    ];
    if CHROMIUM_MARKERS.iter().any(|m| base == *m || base.contains(m)) {
        return Some(BrowserFamily::ChromiumLike);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_vivaldi_as_chromium_like() {
        assert_eq!(
            detect_browser_family("/usr/bin/vivaldi-bin"),
            Some(BrowserFamily::ChromiumLike)
        );
    }

    #[test]
    fn detects_firefox() {
        assert_eq!(
            detect_browser_family("/usr/lib/firefox/firefox"),
            Some(BrowserFamily::Firefox)
        );
    }

    #[test]
    fn ignores_non_browser() {
        assert!(detect_browser_family("/usr/bin/code").is_none());
    }
}
