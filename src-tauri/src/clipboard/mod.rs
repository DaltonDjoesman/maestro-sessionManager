//! Read system clipboard text for the webview (navigator.clipboard is often blocked in Tauri).

pub fn read_text() -> Result<String, String> {
    arboard::Clipboard::new()
        .map_err(|e| format!("clipboard unavailable: {e}"))?
        .get_text()
        .map_err(|e| format!("clipboard read failed: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_text_does_not_panic_when_empty_or_available() {
        let _ = read_text();
    }
}
