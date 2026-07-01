import { FormEvent, useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ApplicationSettings } from "../types/settings";

interface SettingsScreenProps {
  /** Called after settings are saved (e.g. refresh home meta for profiles path). */
  onReloadSettings?: () => void;
  /** Optional: apply theme to the app shell immediately when the user changes the Theme control (before save). */
  onThemePreview?: (theme: ApplicationSettings["theme"]) => void;
}

export function SettingsScreen({ onReloadSettings, onThemePreview }: SettingsScreenProps) {
  const [form, setForm] = useState<ApplicationSettings | null>(null);
  const [profilesError, setProfilesError] = useState<string | null>(null);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    invoke<ApplicationSettings>("get_settings")
      .then(setForm)
      .catch(() => setSaveError("Could not load settings."));
  }, []);

  const validateProfilesRoot = useCallback(async (path: string) => {
    if (!path.trim()) {
      setProfilesError("Profiles path cannot be empty.");
      return false;
    }
    try {
      await invoke("validate_profiles_root", { path });
      setProfilesError(null);
      return true;
    } catch (e) {
      setProfilesError(String(e));
      return false;
    }
  }, []);

  const handleProfilesBlur = () => {
    if (form) {
      void validateProfilesRoot(form.profiles_root);
    }
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!form) return;

    setSaveError(null);
    setSaved(false);

    const valid = await validateProfilesRoot(form.profiles_root);
    if (!valid) return;

    setSaving(true);
    try {
      const updated = await invoke<ApplicationSettings>("save_settings", {
        settings: form,
      });
      setForm(updated);
      setSaved(true);
      onReloadSettings?.();
    } catch (err) {
      setSaveError(String(err));
    } finally {
      setSaving(false);
    }
  };

  if (!form) {
    return (
      <main className="container">
        <p className="hint">Loading settings…</p>
      </main>
    );
  }

  return (
    <main className="container">
      <header className="hero">
        <h1>Settings</h1>
        <p className="tagline">Global Maestro preferences</p>
      </header>

      <form className="settings-form" onSubmit={handleSubmit} noValidate>
        <fieldset>
          <legend>Storage</legend>
          <label className="field">
            <span>Profiles directory</span>
            <input
              type="text"
              value={form.profiles_root}
              onChange={(e) => {
                setProfilesError(null);
                setSaved(false);
                setForm({ ...form, profiles_root: e.target.value });
              }}
              onBlur={handleProfilesBlur}
              aria-invalid={profilesError ? true : undefined}
              aria-describedby="profiles-root-error"
            />
            {profilesError && (
              <p id="profiles-root-error" className="field-error" role="alert">
                {profilesError}
              </p>
            )}
          </label>
        </fieldset>

        <fieldset>
          <legend>Browser defaults</legend>
          <label className="field">
            <span>Default browser executable (optional)</span>
            <input
              type="text"
              value={form.default_browser_executable ?? ""}
              onChange={(e) =>
                setForm({
                  ...form,
                  default_browser_executable: e.target.value || null,
                })
              }
              placeholder="/usr/bin/firefox"
            />
          </label>
          <label className="field">
            <span>Browser family</span>
            <select
              value={form.default_browser_family ?? ""}
              onChange={(e) =>
                setForm({
                  ...form,
                  default_browser_family:
                    (e.target
                      .value as ApplicationSettings["default_browser_family"]) ||
                    null,
                })
              }
            >
              <option value="">—</option>
              <option value="chromium_like">Chromium-like</option>
              <option value="firefox">Firefox</option>
            </select>
          </label>
        </fieldset>

        <fieldset>
          <legend>Profile editor assistant</legend>
          <p className="hint">
            When enabled, the profile editor can suggest running applications to add as launch rows.
            This is optional; manual editing always works.
          </p>
          <label className="field-inline">
            <input
              type="checkbox"
              checked={form.assisted_profile_capture_enabled}
              onChange={(e) =>
                setForm({
                  ...form,
                  assisted_profile_capture_enabled: e.target.checked,
                })
              }
            />
            <span>Show running-apps assistant in profile editor</span>
          </label>
        </fieldset>

        <fieldset>
          <legend>Application</legend>
          <label className="field">
            <span>Logging verbosity</span>
            <select
              value={form.logging_verbosity}
              onChange={(e) =>
                setForm({
                  ...form,
                  logging_verbosity: e.target
                    .value as ApplicationSettings["logging_verbosity"],
                })
              }
            >
              <option value="error">Error</option>
              <option value="warn">Warn</option>
              <option value="info">Info</option>
              <option value="debug">Debug</option>
              <option value="trace">Trace</option>
            </select>
          </label>
          <label className="field">
            <span>Theme</span>
            <select
              value={form.theme}
              onChange={(e) => {
                const theme = e.target.value as ApplicationSettings["theme"];
                setForm({
                  ...form,
                  theme,
                });
                onThemePreview?.(theme);
              }}
            >
              <option value="system">System</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </label>
        </fieldset>

        {saveError && (
          <p className="form-error" role="alert">
            {saveError}
          </p>
        )}
        {saved && <p className="form-success">Settings saved.</p>}

        <div className="form-actions">
          <button type="submit" className="btn-primary" disabled={saving}>
            {saving ? "Saving…" : "Save settings"}
          </button>
        </div>
      </form>
    </main>
  );
}
