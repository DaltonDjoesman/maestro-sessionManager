use super::{PlatformContext, PlatformError};

/// Linux adapter (Pop!_OS reference). Uses `sysinfo` for process enumeration in later tasks.
pub struct LinuxPlatform;

impl LinuxPlatform {
    pub fn new() -> Result<Self, PlatformError> {
        Ok(Self)
    }
}

impl PlatformContext for LinuxPlatform {
    fn platform_name(&self) -> &'static str {
        "linux"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linux_platform_reports_name() {
        let platform = LinuxPlatform::new().expect("linux platform");
        assert_eq!(platform.platform_name(), "linux");
    }
}
