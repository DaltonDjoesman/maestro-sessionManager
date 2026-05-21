import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ActivationStepSummary } from "../types/activation";
import type {
  ApplicationLaunchEntry,
  ProfileBrowserBlock,
  SessionProfile,
} from "../types/profile";
import type { BrowserFamily } from "../types/settings";

interface ProfileEditorScreenProps {
  filePath: string;
  onBack: () => void;
}

function emptyBrowser(): ProfileBrowserBlock {
  return {
    family: "chromium_like",
    executable: "",
    user_data_dir: null,
    firefox_profile: null,
    firefox_no_remote: null,
    urls: [],
  };
}

function cloneProfile(p: SessionProfile): SessionProfile {
  return JSON.parse(JSON.stringify(p)) as SessionProfile;
}

export function ProfileEditorScreen({ filePath, onBack }: ProfileEditorScreenProps) {
  const [profile, setProfile] = useState<SessionProfile | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [includeBrowser, setIncludeBrowser] = useState(false);
  const [activationSteps, setActivationSteps] = useState<ActivationStepSummary[] | null>(null);
  const [activationError, setActivationError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setLoadError(null);
    setSaveError(null);
    setActivationSteps(null);
    setActivationError(null);
    try {
      const p = await invoke<SessionProfile>("load_session_profile", { path: filePath });
      setProfile(cloneProfile(p));
      setIncludeBrowser(p.browser != null);
    } catch (e) {
      setProfile(null);
      setLoadError(String(e));
    } finally {
      setBusy(false);
    }
  }, [filePath]);

  useEffect(() => {
    void load();
  }, [load]);

  const updateProfile = (patch: Partial<SessionProfile>) => {
    setProfile((prev) => (prev ? { ...prev, ...patch } : prev));
  };

  const setBrowserPatch = (patch: Partial<ProfileBrowserBlock>) => {
    setProfile((prev) => {
      if (!prev) return prev;
      const b = prev.browser ?? emptyBrowser();
      return { ...prev, browser: { ...b, ...patch } };
    });
  };

  const save = async () => {
    if (!profile) return;
    setBusy(true);
    setSaveError(null);
    try {
      const toSave: SessionProfile = {
        ...profile,
        browser: includeBrowser ? (profile.browser ?? emptyBrowser()) : null,
        cleanup:
          profile.cleanup &&
          profile.cleanup.allow_extra_basenames.some((s) => s.trim().length > 0)
            ? profile.cleanup
            : null,
      };
      await invoke("save_session_profile", { path: filePath, profile: toSave });
      setProfile(cloneProfile(toSave));
      setIncludeBrowser(toSave.browser != null);
    } catch (e) {
      setSaveError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const activate = async () => {
    setBusy(true);
    setActivationSteps(null);
    setActivationError(null);
    try {
      const steps = await invoke<ActivationStepSummary[]>("activate_session_profile", {
        path: filePath,
      });
      setActivationSteps(steps);
    } catch (e) {
      setActivationError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const addApp = () => {
    setProfile((prev) => {
      if (!prev) return prev;
      const next: ApplicationLaunchEntry = {
        executable: "",
        args: [],
        cwd: null,
        skip_if_running: null,
      };
      return { ...prev, applications: [...prev.applications, next] };
    });
  };

  const removeApp = (index: number) => {
    setProfile((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        applications: prev.applications.filter((_, i) => i !== index),
      };
    });
  };

  const patchApp = (index: number, patch: Partial<ApplicationLaunchEntry>) => {
    setProfile((prev) => {
      if (!prev) return prev;
      const apps = prev.applications.map((a, i) => (i === index ? { ...a, ...patch } : a));
      return { ...prev, applications: apps };
    });
  };

  const setExtrasFromText = (text: string) => {
    const lines = text
      .split("\n")
      .map((s) => s.trim())
      .filter(Boolean);
    setProfile((prev) => {
      if (!prev) return prev;
      if (lines.length === 0) return { ...prev, cleanup: null };
      return { ...prev, cleanup: { allow_extra_basenames: lines } };
    });
  };

  if (loadError) {
    return (
      <main className="container">
        <header className="hero row-between">
          <h1>Edit profile</h1>
          <button type="button" className="btn-secondary" onClick={onBack}>
            Back to catalog
          </button>
        </header>
        <p className="cleanup-msg error">{loadError}</p>
      </main>
    );
  }

  if (!profile) {
    return (
      <main className="container">
        <p className="hint">{busy ? "Loading…" : "No profile."}</p>
      </main>
    );
  }

  const browser = profile.browser ?? emptyBrowser();
  const urlsText = browser.urls.join("\n");
  const extrasText = (profile.cleanup?.allow_extra_basenames ?? []).join("\n");

  return (
    <main className="container profile-editor">
      <header className="hero row-between">
        <div>
          <h1>Edit session</h1>
          <p className="tagline">
            <code className="file-path">{filePath}</code>
          </p>
        </div>
        <button type="button" className="btn-secondary" onClick={onBack} disabled={busy}>
          Catalog
        </button>
      </header>

      {saveError ? <p className="cleanup-msg error">{saveError}</p> : null}

      <section className="editor-section">
        <h2>Identity</h2>
        <label className="field">
          <span>Name</span>
          <input
            type="text"
            value={profile.name}
            onChange={(e) => updateProfile({ name: e.target.value })}
          />
        </label>
        <label className="field">
          <span>Session id (read-only)</span>
          <input type="text" readOnly value={profile.session_id} />
        </label>
      </section>

      <section className="editor-section">
        <div className="section-head">
          <h2>Applications</h2>
          <button type="button" className="btn-secondary btn-compact" onClick={addApp}>
            Add row
          </button>
        </div>
        {profile.applications.length === 0 ? (
          <p className="hint">No applications yet. Add at least one executable, or rely on browser only.</p>
        ) : null}
        {profile.applications.map((app, i) => (
          <div key={i} className="app-card">
            <div className="app-card-head">
              <span>Application {i + 1}</span>
              <button type="button" className="btn-secondary btn-compact danger" onClick={() => removeApp(i)}>
                Remove
              </button>
            </div>
            <label className="field">
              <span>Executable</span>
              <input
                type="text"
                value={app.executable}
                onChange={(e) => patchApp(i, { executable: e.target.value })}
                placeholder="/usr/bin/code or code"
              />
            </label>
            <label className="field">
              <span>Args (one per line)</span>
              <textarea
                rows={3}
                value={app.args.join("\n")}
                onChange={(e) =>
                  patchApp(i, {
                    args: e.target.value
                      .split("\n")
                      .map((s) => s.trimEnd())
                      .filter((s) => s.length > 0),
                  })
                }
              />
            </label>
            <label className="field">
              <span>Working directory (optional)</span>
              <input
                type="text"
                value={app.cwd ?? ""}
                onChange={(e) =>
                  patchApp(i, { cwd: e.target.value.trim() ? e.target.value : null })
                }
              />
            </label>
            <label className="field-inline">
              <input
                type="checkbox"
                checked={app.skip_if_running === true}
                onChange={(e) =>
                  patchApp(i, { skip_if_running: e.target.checked ? true : null })
                }
              />
              <span>Skip if already running (same executable basename)</span>
            </label>
          </div>
        ))}
      </section>

      <section className="editor-section">
        <label className="field-inline">
          <input
            type="checkbox"
            checked={includeBrowser}
            onChange={(e) => {
              const on = e.target.checked;
              setIncludeBrowser(on);
              setProfile((prev) => {
                if (!prev) return prev;
                return { ...prev, browser: on ? prev.browser ?? emptyBrowser() : null };
              });
            }}
          />
          <span>Include browser launch block</span>
        </label>
        {includeBrowser ? (
          <div className="browser-block">
            <label className="field">
              <span>Family</span>
              <select
                value={browser.family}
                onChange={(e) =>
                  setBrowserPatch({ family: e.target.value as BrowserFamily })
                }
              >
                <option value="chromium_like">Chromium-like</option>
                <option value="firefox">Firefox</option>
              </select>
            </label>
            <label className="field">
              <span>Browser executable</span>
              <input
                type="text"
                value={browser.executable}
                onChange={(e) => setBrowserPatch({ executable: e.target.value })}
              />
            </label>
            {browser.family === "chromium_like" ? (
              <label className="field">
                <span>User data dir (optional)</span>
                <input
                  type="text"
                  value={browser.user_data_dir ?? ""}
                  onChange={(e) =>
                    setBrowserPatch({
                      user_data_dir: e.target.value.trim() ? e.target.value : null,
                    })
                  }
                />
              </label>
            ) : (
              <>
                <label className="field">
                  <span>Firefox profile name (optional)</span>
                  <input
                    type="text"
                    value={browser.firefox_profile ?? ""}
                    onChange={(e) =>
                      setBrowserPatch({
                        firefox_profile: e.target.value.trim() ? e.target.value : null,
                      })
                    }
                  />
                </label>
                <label className="field-inline">
                  <input
                    type="checkbox"
                    checked={browser.firefox_no_remote === true}
                    onChange={(e) =>
                      setBrowserPatch({ firefox_no_remote: e.target.checked ? true : null })
                    }
                  />
                  <span>Add -no-remote</span>
                </label>
              </>
            )}
            <label className="field">
              <span>URLs (one per line)</span>
              <textarea
                rows={4}
                value={urlsText}
                onChange={(e) =>
                  setBrowserPatch({
                    urls: e.target.value
                      .split("\n")
                      .map((s) => s.trim())
                      .filter(Boolean),
                  })
                }
              />
            </label>
          </div>
        ) : null}
      </section>

      <section className="editor-section">
        <h2>Cleanup extras (optional)</h2>
        <p className="hint">Extra executable basenames treated as allowed during divergence checks (one per line).</p>
        <label className="field">
          <span>Allow extra basenames</span>
          <textarea rows={3} value={extrasText} onChange={(e) => setExtrasFromText(e.target.value)} />
        </label>
      </section>

      <div className="editor-actions">
        <button type="button" className="btn-primary" disabled={busy} onClick={() => void save()}>
          Save profile
        </button>
        <button type="button" className="btn-secondary" disabled={busy} onClick={() => void load()}>
          Reload from disk
        </button>
        <button type="button" className="btn-primary" disabled={busy} onClick={() => void activate()}>
          Activate session
        </button>
      </div>

      {activationError ? <p className="cleanup-msg error">{activationError}</p> : null}

      {activationSteps ? (
        <section className="editor-section activation-results">
          <h2>Activation steps</h2>
          {activationSteps.length === 0 ? (
            <p className="hint">No steps returned (empty profile?).</p>
          ) : (
            <div className="catalog-table-wrap">
              <table className="catalog-table">
                <thead>
                  <tr>
                    <th>Kind</th>
                    <th>Label</th>
                    <th>Status</th>
                    <th>PID</th>
                    <th>Detail</th>
                  </tr>
                </thead>
                <tbody>
                  {activationSteps.map((s, idx) => (
                    <tr key={`${s.label}-${idx}`}>
                      <td>{s.kind}</td>
                      <td>
                        <code>{s.label}</code>
                      </td>
                      <td>
                        <span className={`badge badge-status-${s.status}`}>{s.status}</span>
                      </td>
                      <td>{s.pid ?? "—"}</td>
                      <td className="detail-cell">{s.detail ?? "—"}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </section>
      ) : null}
    </main>
  );
}
