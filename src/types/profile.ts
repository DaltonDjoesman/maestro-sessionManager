import type { BrowserFamily } from "./settings";

/** Catalog row from `list_session_profiles` (camelCase). */
export type ProfileCatalogEntry = {
  filePath: string;
  fileName: string;
  valid: boolean;
  sessionId: string | null;
  name: string | null;
  error?: string | null;
  applicationsCount?: number | null;
  /** At least one application row is a browser. */
  hasBrowser?: boolean | null;
};

export type DuplicateProfileResult = {
  filePath: string;
  profile: SessionProfile;
};

/** Matches session profile JSON (`snake_case` from Rust). */
export type ApplicationBrowserSettings = {
  family: BrowserFamily;
  user_data_dir: string | null;
  firefox_profile: string | null;
  firefox_no_remote: boolean | null;
  urls: string[];
};

export type ApplicationLaunchEntry = {
  executable: string;
  args: string[];
  cwd: string | null;
  skip_if_running: boolean | null;
  /** When set, activation uses browser argv builders instead of plain executable spawn. */
  browser?: ApplicationBrowserSettings | null;
};

/** @deprecated Legacy top-level browser block; migrated into applications on load. */
export type ProfileBrowserBlock = ApplicationBrowserSettings & {
  executable: string;
};

export type ProfileCleanupRules = {
  allow_extra_basenames: string[];
};

export type SessionProfile = {
  schema_version: number;
  session_id: string;
  name: string;
  applications: ApplicationLaunchEntry[];
  browser: ProfileBrowserBlock | null;
  cleanup: ProfileCleanupRules | null;
};
