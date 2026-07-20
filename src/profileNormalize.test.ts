import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { normalizeProfile, normalizeProfileForRun } from "./profileNormalize";
import type { SessionProfile } from "./types/profile";

const __dirname = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(
  __dirname,
  "../src-tauri/tests/fixtures/legacy-browser-normalize.json",
);

type NormalizeFixture = {
  input: SessionProfile;
  expected_after_normalize: SessionProfile;
};

describe("normalizeProfile", () => {
  it("matches the shared golden fixture for legacy browser-block migration", () => {
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as NormalizeFixture;
    expect(normalizeProfile(fixture.input)).toEqual(fixture.expected_after_normalize);
  });

  it("drops cleanup and trims browser URLs for run", () => {
    const profile: SessionProfile = {
      schema_version: 1,
      session_id: "id",
      name: "Run",
      applications: [
        {
          executable: "/usr/bin/firefox",
          args: [],
          cwd: null,
          skip_if_running: null,
          browser: {
            family: "firefox",
            user_data_dir: null,
            firefox_profile: null,
            firefox_no_remote: null,
            urls: ["  https://a.com  ", "", "https://b.com"],
          },
        },
      ],
      browser: null,
      cleanup: { allow_extra_basenames: ["helper"] },
    };
    const got = normalizeProfileForRun(profile);
    expect(got.cleanup).toBeNull();
    expect(got.applications[0].browser?.urls).toEqual(["https://a.com", "https://b.com"]);
  });
});
