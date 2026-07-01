export type BrowserFamily = "chromium_like" | "firefox";

export interface SystemDefaultBrowserHint {
  executable: string | null;
  family: BrowserFamily | null;
  desktopEntry: string | null;
}

export type LogVerbosity = "error" | "warn" | "info" | "debug" | "trace";

export type UiTheme = "system" | "light" | "dark";

export interface ApplicationSettings {
  schema_version: number;
  profiles_root: string;
  default_browser_executable: string | null;
  default_browser_family: BrowserFamily | null;
  logging_verbosity: LogVerbosity;
  theme: UiTheme;
  /** When false, running-apps assistant is hidden and list command returns []. */
  assisted_profile_capture_enabled: boolean;
}
