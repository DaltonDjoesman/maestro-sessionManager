import {
  normalizeApplicationEntry,
  normalizeBrowserUrlList,
} from "./browserDetect";
import type { SessionProfile } from "./types/profile";

/** Migrate legacy top-level `browser` into `applications[]` and normalize each entry. */
export function normalizeProfile(p: SessionProfile): SessionProfile {
  const applications = p.applications.map(normalizeApplicationEntry);
  if (p.browser) {
    const legacy = p.browser;
    applications.unshift(
      normalizeApplicationEntry({
        executable: legacy.executable,
        args: [],
        cwd: null,
        skip_if_running: null,
        browser: {
          family: legacy.family,
          user_data_dir: legacy.user_data_dir,
          firefox_profile: legacy.firefox_profile,
          firefox_no_remote: legacy.firefox_no_remote,
          urls: legacy.urls,
        },
      }),
    );
  }
  return { ...p, browser: null, applications };
}

/** Normalize for persist/activate: drop cleanup, trim browser URL lists. */
export function normalizeProfileForRun(p: SessionProfile): SessionProfile {
  const normalized = normalizeProfile(p);
  return {
    ...normalized,
    cleanup: null,
    applications: normalized.applications.map((app) => {
      if (!app.browser) return app;
      return {
        ...app,
        browser: {
          ...app.browser,
          urls: normalizeBrowserUrlList(app.browser.urls ?? []),
        },
      };
    }),
  };
}
