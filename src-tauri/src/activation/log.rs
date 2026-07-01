//! Best-effort activation transcript under the Maestro data directory.

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

/// Opens (truncates) `…/maestro/logs/last_activation.log`. Returns `None` if the data dir is unavailable.
pub(crate) fn try_open_activation_log() -> Option<(PathBuf, File)> {
    let base = crate::settings::maestro_data_dir().ok()?;
    let dir = base.join("logs");
    fs::create_dir_all(&dir).ok()?;
    let path = dir.join("last_activation.log");
    let f = File::create(&path).ok()?;
    Some((path, f))
}

pub(crate) fn log_line(file: &mut File, line: &str) {
    let _ = writeln!(file, "{line}");
}
