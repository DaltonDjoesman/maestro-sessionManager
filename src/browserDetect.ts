import type { ApplicationBrowserSettings, ApplicationLaunchEntry } from "./types/profile";
import type { BrowserFamily } from "./types/settings";

const FIREFOX_MARKERS = ["firefox", "librewolf", "waterfox", "floorp", "zen-browser"];

const CHROMIUM_MARKERS = [
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

function executableBasename(executable: string): string {
  const parts = executable.split(/[/\\]/);
  return (parts[parts.length - 1] || executable).toLowerCase();
}

export function inferBrowserFamily(executable: string): BrowserFamily | null {
  const base = executableBasename(executable);
  if (!base) return null;
  if (FIREFOX_MARKERS.some((m) => base.includes(m))) return "firefox";
  if (CHROMIUM_MARKERS.some((m) => base === m || base.includes(m))) return "chromium_like";
  return null;
}

export function emptyBrowserSettings(family: BrowserFamily): ApplicationBrowserSettings {
  return {
    family,
    user_data_dir: null,
    firefox_profile: null,
    firefox_no_remote: null,
    urls: [],
  };
}

export function isBrowserApp(app: ApplicationLaunchEntry): boolean {
  return app.browser != null || inferBrowserFamily(app.executable) != null;
}

export function browserSettingsOf(app: ApplicationLaunchEntry): ApplicationBrowserSettings | null {
  if (app.browser) return app.browser;
  const family = inferBrowserFamily(app.executable);
  return family ? emptyBrowserSettings(family) : null;
}

export function normalizeApplicationEntry(app: ApplicationLaunchEntry): ApplicationLaunchEntry {
  const base: ApplicationLaunchEntry = {
    executable: app.executable ?? "",
    args: app.args ?? [],
    cwd: app.cwd ?? null,
    skip_if_running: app.skip_if_running ?? null,
    browser: app.browser
      ? {
          ...app.browser,
          urls: app.browser.urls ?? [],
        }
      : null,
  };
  const family = inferBrowserFamily(base.executable);
  if (!family) {
    return { ...base, browser: null };
  }
  return {
    ...base,
    browser: base.browser ?? emptyBrowserSettings(family),
    args: [],
    cwd: null,
  };
}

export function normalizeBrowserUrlList(urls: string[]): string[] {
  return urls.map((u) => u.trim()).filter((u) => u.length > 0);
}
