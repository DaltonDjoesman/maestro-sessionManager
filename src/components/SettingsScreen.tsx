import { FormEvent, useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { pt } from "../i18n/pt";
import type { ApplicationSettings } from "../types/settings";

interface SettingsScreenProps {
  onReloadSettings?: () => void;
  onThemePreview?: (theme: ApplicationSettings["theme"]) => void;
}

export function SettingsScreen({ onReloadSettings, onThemePreview }: SettingsScreenProps) {
  const [form, setForm] = useState<ApplicationSettings | null>(null);
  const [profilesError, setProfilesError] = useState<string | null>(null);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const version = import.meta.env.VITE_APP_VERSION ?? "0.1.0";

  useEffect(() => {
    invoke<ApplicationSettings>("get_settings")
      .then(setForm)
      .catch(() => setSaveError(pt.settings.loadError));
  }, []);

  const validateProfilesRoot = useCallback(async (path: string) => {
    if (!path.trim()) {
      setProfilesError(pt.settings.profilesEmpty);
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
    if (form) void validateProfilesRoot(form.profiles_root);
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
      const updated = await invoke<ApplicationSettings>("save_settings", { settings: form });
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
      <div className="page settings-page">
        <p className="hint">{pt.settings.loading}</p>
      </div>
    );
  }

  return (
    <div className="page settings-page">
      <div className="view-title-group">
        <h1 className="view-title">{pt.settings.title}</h1>
        <p className="view-subtitle">{pt.settings.tagline}</p>
      </div>

      <form className="settings-form settings-form--grouped" onSubmit={handleSubmit} noValidate>
        <fieldset className="form-section settings-group">
          <legend className="form-section-title">{pt.settings.general}</legend>
          <label className="field">
            <span>{pt.settings.theme}</span>
            <select
              className="form-select"
              value={form.theme}
              onChange={(e) => {
                const theme = e.target.value as ApplicationSettings["theme"];
                setForm({ ...form, theme });
                onThemePreview?.(theme);
              }}
            >
              <option value="system">{pt.settings.themeSystem}</option>
              <option value="light">{pt.settings.themeLight}</option>
              <option value="dark">{pt.settings.themeDark}</option>
            </select>
          </label>
        </fieldset>

        <fieldset className="form-section settings-group">
          <legend className="form-section-title">{pt.settings.sessions}</legend>
          <label className="field">
            <span>{pt.settings.profilesDir}</span>
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
            {profilesError ? (
              <p id="profiles-root-error" className="field-error" role="alert">
                {profilesError}
              </p>
            ) : null}
          </label>
        </fieldset>

        <fieldset className="form-section settings-group">
          <legend className="form-section-title">{pt.settings.browser}</legend>
          <label className="field">
            <span>{pt.settings.browserExecutable}</span>
            <input
              type="text"
              value={form.default_browser_executable ?? ""}
              onChange={(e) =>
                setForm({ ...form, default_browser_executable: e.target.value || null })
              }
              placeholder="/usr/bin/firefox"
            />
          </label>
          <label className="field">
            <span>{pt.settings.browserFamily}</span>
            <select
              className="form-select"
              value={form.default_browser_family ?? ""}
              onChange={(e) =>
                setForm({
                  ...form,
                  default_browser_family:
                    (e.target.value as ApplicationSettings["default_browser_family"]) || null,
                })
              }
            >
              <option value="">{pt.settings.browserFamilyNone}</option>
              <option value="chromium_like">{pt.settings.chromium}</option>
              <option value="firefox">{pt.settings.firefox}</option>
            </select>
          </label>
        </fieldset>

        <fieldset className="form-section settings-group">
          <legend className="form-section-title">{pt.settings.advanced}</legend>
          <label className="field">
            <span>{pt.settings.logging}</span>
            <select
              className="form-select"
              value={form.logging_verbosity}
              onChange={(e) =>
                setForm({
                  ...form,
                  logging_verbosity: e.target.value as ApplicationSettings["logging_verbosity"],
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
        </fieldset>

        {saveError ? (
          <p className="form-error" role="alert">
            {saveError}
          </p>
        ) : null}
        {saved ? <p className="form-success">{pt.settings.saved}</p> : null}

        <div className="form-actions">
          <button type="submit" className="btn btn-primary" disabled={saving}>
            {saving ? pt.settings.saving : pt.settings.save}
          </button>
        </div>
      </form>

      <section className="settings-about status-card">
        <h2 className="settings-about-title">{pt.settings.aboutTitle}</h2>
        <p>
          <strong>{pt.settings.version}</strong> {String(version)}
        </p>
        <p className="hint">{pt.settings.aboutBody}</p>
        <p className="hint">{pt.settings.aboutTech}</p>
      </section>
    </div>
  );
}
