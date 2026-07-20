import { describe, expect, it } from "vitest";
import { downloadJsonFile, safeDownloadBase } from "./downloadJson";
import { appDisplayLabel, appSummaryHint, cloneProfile } from "./profileEditorHelpers";
import type { ApplicationLaunchEntry, SessionProfile } from "./types/profile";

describe("safeDownloadBase", () => {
  it("sanitizes unsafe characters and falls back", () => {
    expect(safeDownloadBase("My Session!")).toBe("My_Session_");
    expect(safeDownloadBase("")).toBe("session");
  });
});

describe("downloadJsonFile", () => {
  it("creates an anchor download with a .json name", () => {
    const clicks: string[] = [];
    const originalCreate = document.createElement.bind(document);
    document.createElement = ((tag: string) => {
      const el = originalCreate(tag);
      if (tag === "a") {
        Object.defineProperty(el, "click", {
          value: () => {
            clicks.push((el as HTMLAnchorElement).download);
          },
        });
      }
      return el;
    }) as typeof document.createElement;

    try {
      downloadJsonFile('{"ok":true}', "My Profile.json");
      expect(clicks).toEqual(["My_Profile.json"]);
    } finally {
      document.createElement = originalCreate;
    }
  });
});

describe("profileEditorHelpers", () => {
  const app = (executable: string): ApplicationLaunchEntry => ({
    executable,
    args: [],
    cwd: null,
    skip_if_running: null,
    browser: null,
  });

  it("labels apps from basename and clones profiles", () => {
    expect(appDisplayLabel(app("/usr/bin/cursor.AppImage"), 0)).toBe("Cursor");
    expect(appSummaryHint(app(""))).toContain("execut");
    const profile: SessionProfile = {
      schema_version: 1,
      session_id: "id",
      name: "n",
      applications: [app("x")],
      browser: null,
      cleanup: null,
    };
    const cloned = cloneProfile(profile);
    expect(cloned).toEqual(profile);
    expect(cloned).not.toBe(profile);
  });
});
