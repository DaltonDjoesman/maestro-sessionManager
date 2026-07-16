//! Scored app vs process classification (no hardcoded basename allowlist).

use super::assistant::{CandidateKind, ClassificationConfidence};

pub const SCORE_WINDOW: i32 = 100;
pub const SCORE_DESKTOP_EXEC: i32 = 80;
pub const SCORE_STARTUP_WM_CLASS: i32 = 60;
pub const SCORE_FLATPAK_SNAP: i32 = 20;
pub const SCORE_USER_SESSION: i32 = 10;
pub const SCORE_NO_WINDOW_PENALTY: i32 = -50;
pub const APP_THRESHOLD: i32 = 80;
pub const EXCLUDED_SCORE: i32 = -1000;

#[derive(Debug, Clone, Default)]
pub struct ScoreInput {
    pub excluded: bool,
    pub has_window: bool,
    pub desktop_match: bool,
    pub startup_wm_class_match: bool,
    pub flatpak_snap_hint: bool,
    pub user_session: bool,
    /// True when a window source returned a non-empty mapped list for this session.
    /// When false (e.g. pure Wayland without protocol access), strong `.desktop`
    /// matches are not penalized for lacking a mapped window.
    pub window_list_available: bool,
}

pub fn compute_score(input: &ScoreInput) -> i32 {
    if input.excluded {
        return EXCLUDED_SCORE;
    }
    let mut score = 0;
    if input.has_window {
        score += SCORE_WINDOW;
    }
    if input.desktop_match {
        score += SCORE_DESKTOP_EXEC;
    }
    if input.startup_wm_class_match {
        score += SCORE_STARTUP_WM_CLASS;
    }
    if input.flatpak_snap_hint {
        score += SCORE_FLATPAK_SNAP;
    }
    if input.user_session {
        score += SCORE_USER_SESSION;
    }
    if input.desktop_match && !input.has_window && input.window_list_available {
        score += SCORE_NO_WINDOW_PENALTY;
    }
    score
}

pub fn classify_from_score(
    input: &ScoreInput,
    score: i32,
) -> (CandidateKind, ClassificationConfidence) {
    if score < APP_THRESHOLD {
        return (CandidateKind::Process, ClassificationConfidence::Low);
    }
    let confidence = if input.has_window && input.desktop_match {
        ClassificationConfidence::High
    } else if input.has_window || input.desktop_match || input.startup_wm_class_match {
        ClassificationConfidence::Medium
    } else {
        ClassificationConfidence::Low
    };
    (CandidateKind::App, confidence)
}

pub fn classify(input: &ScoreInput) -> (CandidateKind, ClassificationConfidence, i32) {
    let score = compute_score(input);
    let (kind, confidence) = classify_from_score(input, score);
    (kind, confidence, score)
}

pub fn user_session_active() -> bool {
    std::env::var("DISPLAY").is_ok()
        || std::env::var("WAYLAND_DISPLAY").is_ok()
        || std::env::var("XDG_SESSION_TYPE").is_ok()
}

pub fn flatpak_snap_path_hint(executable: &str) -> bool {
    let el = executable.to_lowercase();
    el.contains("/app/") || el.contains("/snap/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_only_meets_app_threshold() {
        let input = ScoreInput {
            has_window: true,
            user_session: true,
            ..Default::default()
        };
        let (kind, conf, score) = classify(&input);
        assert!(score >= APP_THRESHOLD);
        assert_eq!(kind, CandidateKind::App);
        assert_eq!(conf, ClassificationConfidence::Medium);
    }

    #[test]
    fn desktop_without_window_becomes_process_when_windows_enumerable() {
        let input = ScoreInput {
            desktop_match: true,
            user_session: true,
            window_list_available: true,
            ..Default::default()
        };
        let (kind, _, score) = classify(&input);
        assert!(score < APP_THRESHOLD);
        assert_eq!(kind, CandidateKind::Process);
    }

    #[test]
    fn strong_desktop_match_on_wayland_without_window_list_is_app() {
        let input = ScoreInput {
            desktop_match: true,
            user_session: true,
            window_list_available: false,
            ..Default::default()
        };
        let (kind, _, score) = classify(&input);
        assert!(
            score >= APP_THRESHOLD,
            "expected desktop+session to meet app threshold without no-window penalty, got {score}"
        );
        assert_eq!(kind, CandidateKind::App);
    }

    #[test]
    fn window_and_desktop_high_confidence() {
        let input = ScoreInput {
            has_window: true,
            desktop_match: true,
            user_session: true,
            ..Default::default()
        };
        let (_, conf, _) = classify(&input);
        assert_eq!(conf, ClassificationConfidence::High);
    }

    #[test]
    fn flatpak_path_adds_points_with_window() {
        let input = ScoreInput {
            has_window: true,
            flatpak_snap_hint: true,
            user_session: true,
            ..Default::default()
        };
        let (_, _, score) = classify(&input);
        assert!(score >= APP_THRESHOLD + SCORE_FLATPAK_SNAP);
    }

    #[test]
    fn excluded_always_process() {
        let input = ScoreInput {
            excluded: true,
            has_window: true,
            desktop_match: true,
            ..Default::default()
        };
        let (kind, _, score) = classify(&input);
        assert_eq!(score, EXCLUDED_SCORE);
        assert_eq!(kind, CandidateKind::Process);
    }
}
