//! Canonical known-editor executable basenames (activation skip, window discovery, assistant).
//! Keep in sync with `src/types/capture.ts` `EDITOR_BASES`.

/// Lowercased executable basenames treated as code/editors across the stack.
pub const EDITOR_BASENAMES: &[&str] = &["cursor", "code", "code-oss", "codium", "obsidian"];

pub fn is_known_editor_basename(basename_lower: &str) -> bool {
    EDITOR_BASENAMES.contains(&basename_lower)
}
