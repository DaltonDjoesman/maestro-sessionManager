/**
 * In-app copy of `docs/examples/smoke-session.profile.json` for “create from example”.
 * Keep in sync when the docs example changes.
 */
export const SMOKE_SESSION_PROFILE_JSON = JSON.stringify(
  {
    schema_version: 1,
    session_id: "00000000-0000-4000-8000-000000000001",
    name: "Smoke test (mock browser)",
    applications: [
      {
        executable: "/bin/true",
        args: [],
        cwd: null,
        skip_if_running: null,
      },
      {
        executable: "/bin/true",
        args: [],
        cwd: null,
        skip_if_running: null,
        browser: {
          family: "chromium_like",
          user_data_dir: "/tmp/maestro-smoke-user-data",
          firefox_profile: null,
          firefox_no_remote: null,
          urls: ["https://example.com"],
        },
      },
    ],
    cleanup: null,
  },
  null,
  2,
);

export const SMOKE_SESSION_EXAMPLE_NAME = "Exemplo smoke test";
