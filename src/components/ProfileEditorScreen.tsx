import { useCallback, useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openPath } from "@tauri-apps/plugin-opener";
import { activationStepTitle } from "../activationStepLabels";
import type {
  ActivateSessionResult,
  ActivationPreviewStep,
  ActivationStepSummary,
} from "../types/activation";
import type { RunningAppCandidate, RunningAppSection } from "../types/capture";
import {
  candidateKey,
  displayNameOf,
  filterByKind,
  groupRunningApps,
  sortByDisplayName,
  windowSubtitle,
} from "../types/capture";
import type {
  ApplicationLaunchEntry,
  ProfileBrowserBlock,
  SessionProfile,
} from "../types/profile";
import type { ApplicationSettings, BrowserFamily, SystemDefaultBrowserHint } from "../types/settings";

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

function normalizeBrowserUrlList(urls: string[]): string[] {
  return urls.map((u) => u.trim()).filter((u) => u.length > 0);
}

function runningSectionTitle(section: RunningAppSection): string {
  if (section.type === "workspace") {
    return `Workspace ${section.workspace + 1}`;
  }
  if (section.type === "noWorkspace") {
    return "No workspace";
  }
  return "";
}

export function ProfileEditorScreen({ filePath, onBack }: ProfileEditorScreenProps) {
  const [profile, setProfile] = useState<SessionProfile | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [includeBrowser, setIncludeBrowser] = useState(false);
  const [activationResult, setActivationResult] = useState<ActivateSessionResult | null>(null);
  const [activationError, setActivationError] = useState<string | null>(null);
  const [previewOpen, setPreviewOpen] = useState(false);
  const [previewSteps, setPreviewSteps] = useState<ActivationPreviewStep[] | null>(null);
  const [previewBusy, setPreviewBusy] = useState(false);
  const [previewError, setPreviewError] = useState<string | null>(null);
  const [assistedCaptureEnabled, setAssistedCaptureEnabled] = useState(false);
  const [runningApps, setRunningApps] = useState<RunningAppCandidate[]>([]);
  const [runningBusy, setRunningBusy] = useState(false);
  const [runningErr, setRunningErr] = useState<string | null>(null);
  const [selectedPids, setSelectedPids] = useState<Record<string, boolean>>({});
  const [cwdDraftByPid, setCwdDraftByPid] = useState<Record<string, string>>({});
  const [showRunningProcesses, setShowRunningProcesses] = useState(false);
  const [systemBrowserHint, setSystemBrowserHint] = useState<SystemDefaultBrowserHint | null>(null);
  const [settingsBrowserDefaults, setSettingsBrowserDefaults] = useState<{
    default_browser_executable: string | null;
    default_browser_family: BrowserFamily | null;
  } | null>(null);

  const browserExecutablePlaceholder = useMemo(() => {
    const sys = systemBrowserHint?.executable?.trim();
    if (sys) {
      return `System default (this PC): ${sys}`;
    }
    const saved = settingsBrowserDefaults?.default_browser_executable?.trim();
    if (saved) {
      return `From Settings: ${saved}`;
    }
    return "google-chrome-stable, firefox, brave-browser, …";
  }, [systemBrowserHint, settingsBrowserDefaults]);

  const visibleRunningApps = useMemo(
    () => filterByKind(runningApps, showRunningProcesses),
    [runningApps, showRunningProcesses],
  );
  const runningAppSections = useMemo(
    () => groupRunningApps(visibleRunningApps),
    [visibleRunningApps],
  );
  const visibleAppCount = useMemo(
    () => runningApps.filter((c) => (c.kind ?? "app") === "app").length,
    [runningApps],
  );
  const visibleProcessCount = runningApps.length - visibleAppCount;
  const selectedRunningCount = useMemo(
    () => runningApps.reduce((n, c) => n + (selectedPids[candidateKey(c)] ? 1 : 0), 0),
    [runningApps, selectedPids],
  );

  const load = useCallback(async () => {
    setBusy(true);
    setLoadError(null);
    setSaveError(null);
    setActivationResult(null);
    setActivationError(null);
    setPreviewSteps(null);
    setPreviewError(null);
    setPreviewOpen(false);
    try {
      const [p, settings, hint] = await Promise.all([
        invoke<SessionProfile>("load_session_profile", { path: filePath }),
        invoke<ApplicationSettings>("get_settings"),
        invoke<SystemDefaultBrowserHint>("detect_system_default_browser"),
      ]);
      setProfile(cloneProfile(p));
      setIncludeBrowser(p.browser != null);
      setAssistedCaptureEnabled(settings.assisted_profile_capture_enabled ?? false);
      setSystemBrowserHint(hint);
      setSettingsBrowserDefaults({
        default_browser_executable: settings.default_browser_executable,
        default_browser_family: settings.default_browser_family,
      });
    } catch (e) {
      setProfile(null);
      setLoadError(String(e));
      setSystemBrowserHint(null);
      setSettingsBrowserDefaults(null);
    } finally {
      setBusy(false);
    }
  }, [filePath]);

  const refreshRunningApps = useCallback(async () => {
    setRunningBusy(true);
    setRunningErr(null);
    try {
      const list = await invoke<RunningAppCandidate[]>("list_assistant_running_apps");
      setRunningApps(list);
      const cwd: Record<string, string> = {};
      for (const c of list) {
        cwd[candidateKey(c)] = c.cwdHint?.trim() ? c.cwdHint.trim() : "";
      }
      setSelectedPids({});
      setCwdDraftByPid(cwd);
    } catch (e) {
      setRunningErr(String(e));
    } finally {
      setRunningBusy(false);
    }
  }, []);

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
      const rawBrowser = includeBrowser ? (profile.browser ?? emptyBrowser()) : null;
      const browserBlock = rawBrowser
        ? { ...rawBrowser, urls: normalizeBrowserUrlList(rawBrowser.urls) }
        : null;
      const toSave: SessionProfile = {
        ...profile,
        browser: browserBlock,
        cleanup: null,
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
    if (!profile) return;
    setBusy(true);
    setActivationResult(null);
    setActivationError(null);
    setPreviewSteps(null);
    setPreviewError(null);
    try {
      const rawBrowser = includeBrowser ? (profile.browser ?? emptyBrowser()) : null;
      const browserBlock = rawBrowser
        ? { ...rawBrowser, urls: normalizeBrowserUrlList(rawBrowser.urls) }
        : null;
      const toActivate: SessionProfile = {
        ...profile,
        browser: browserBlock,
        cleanup: null,
      };
      const result = await invoke<ActivateSessionResult>("activate_session_profile", {
        path: filePath,
        profile: toActivate,
      });
      setActivationResult(result);
    } catch (e) {
      setActivationError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const runDryRun = async () => {
    if (!profile) return;
    setPreviewBusy(true);
    setPreviewError(null);
    setPreviewSteps(null);
    try {
      const rawBrowser = includeBrowser ? (profile.browser ?? emptyBrowser()) : null;
      const browserBlock = rawBrowser
        ? { ...rawBrowser, urls: normalizeBrowserUrlList(rawBrowser.urls) }
        : null;
      const toActivate: SessionProfile = {
        ...profile,
        browser: browserBlock,
        cleanup: null,
      };
      const steps = await invoke<ActivationPreviewStep[]>("preview_session_activation", {
        path: filePath,
        profile: toActivate,
      });
      setPreviewSteps(steps);
      setPreviewOpen(true);
    } catch (e) {
      setPreviewError(String(e));
      setPreviewOpen(true);
    } finally {
      setPreviewBusy(false);
    }
  };

  const openActivationLog = async () => {
    const p = activationResult?.activationLogPath?.trim();
    if (!p) {
      window.alert("Log file path is not available yet. Activate a session once to generate the log.");
      return;
    }
    try {
      await openPath(p);
    } catch (e) {
      window.alert(String(e));
    }
  };

  const activationTimelineIcon = (s: ActivationStepSummary) => {
    if (s.status === "success") return "✓";
    if (s.status === "failure") return "✗";
    if (s.status === "skipped") return "⊘";
    if (s.status === "warning") return "⚠";
    return "•";
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

  const addSelectedRunningToDraft = () => {
    const chosen = sortByDisplayName(runningApps).filter((c) => selectedPids[candidateKey(c)]);
    if (chosen.length === 0) return;
    setProfile((prev) => {
      if (!prev) return prev;
      const newRows: ApplicationLaunchEntry[] = chosen.map((c) => {
        const key = candidateKey(c);
        const folder =
          (cwdDraftByPid[key] ?? "").trim() || (c.cwdHint ?? "").trim() || "";
        const exeLower = c.executable.toLowerCase();
        const vscodeLike =
          exeLower.includes("/cursor") ||
          exeLower.endsWith("/cursor") ||
          exeLower.includes("code-oss") ||
          exeLower.includes("vscodium") ||
          exeLower.includes("/bin/code") ||
          exeLower.endsWith("/code") ||
          exeLower.includes("/codium") ||
          exeLower.includes("obsidian");
        let args: string[] = [];
        let cwd: string | null = null;
        if (folder.length > 0) {
          cwd = folder;
          if (vscodeLike) {
            args = [folder];
          }
        }
        return {
          executable: c.executable,
          args,
          cwd,
          skip_if_running: null,
        };
      });
      return { ...prev, applications: [...prev.applications, ...newRows] };
    });
  };

  const toggleRunningCardSelection = (key: string) => {
    setSelectedPids((prev) => ({ ...prev, [key]: !prev[key] }));
  };

  const onRunningCardSurfaceClick = (e: React.MouseEvent<HTMLElement>, key: string) => {
    const t = e.target as HTMLElement;
    if (t.closest("input, textarea, label.running-app-card-check, details, summary")) return;
    toggleRunningCardSelection(key);
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

  const addBrowserUrlRow = () => {
    setBrowserPatch({ urls: [...browser.urls, ""] });
  };

  const pasteBrowserUrlsFromClipboard = async () => {
    let text: string;
    try {
      text = await navigator.clipboard.readText();
    } catch {
      return;
    }
    const lines = text
      .split(/\r?\n|,/g)
      .map((s) => s.trim())
      .filter((s) => s.length > 0);
    if (lines.length === 0) return;
    setProfile((prev) => {
      if (!prev) return prev;
      const b = prev.browser ?? emptyBrowser();
      const seen = new Set(b.urls.map((u) => u.trim()).filter(Boolean));
      const merged = [...b.urls];
      for (const line of lines) {
        if (!seen.has(line)) {
          seen.add(line);
          merged.push(line);
        }
      }
      return { ...prev, browser: { ...b, urls: merged } };
    });
  };

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

      {assistedCaptureEnabled ? (
        <section className="editor-section">
          <div className="section-head running-assistant-head">
            <h2>Running apps</h2>
            <div className="running-assistant-toolbar">
              {runningApps.length > 0 ? (
                <span className="running-assistant-meta" aria-live="polite">
                  {visibleAppCount} app{visibleAppCount === 1 ? "" : "s"}
                  {visibleProcessCount > 0
                    ? ` · ${visibleProcessCount} process${visibleProcessCount === 1 ? "" : "es"} hidden`
                    : ""}
                  {selectedRunningCount > 0 ? ` · ${selectedRunningCount} selected` : ""}
                </span>
              ) : null}
              <label className="running-assistant-toggle">
                <input
                  type="checkbox"
                  checked={showRunningProcesses}
                  onChange={(e) => setShowRunningProcesses(e.target.checked)}
                  disabled={busy || runningBusy}
                />
                <span>Show processes</span>
              </label>
              <button
                type="button"
                className="btn-secondary btn-compact"
                disabled={busy || runningBusy}
                onClick={() => void refreshRunningApps()}
              >
                {runningBusy ? "Refreshing…" : "Refresh list"}
              </button>
            </div>
          </div>
          <p className="hint">
            Optional helper: pick open applications to append as draft launch rows. Save the profile to
            persist. For Cursor / VS Code / Obsidian on Linux, each project folder shows as its own row.
            When you add those, the folder is passed as the first CLI argument (and as cwd). Click a card to
            select; expand details for executable and command line.
          </p>
          {runningErr ? <p className="cleanup-msg error">{runningErr}</p> : null}
          {!runningBusy && runningApps.length === 0 ? (
            <p className="hint">No scan yet. Choose Refresh list to load candidates.</p>
          ) : null}
          {!runningBusy && runningApps.length > 0 && visibleRunningApps.length === 0 ? (
            <p className="hint">No GUI apps detected. Enable “Show processes” to see background programs.</p>
          ) : null}
          {visibleRunningApps.length > 0 ? (
            <>
              {runningAppSections.map((section) => (
                <div
                  key={
                    section.type === "workspace"
                      ? `ws-${section.workspace}`
                      : section.type === "noWorkspace"
                        ? "no-ws"
                        : "flat"
                  }
                  className="running-apps-section"
                >
                  {section.type !== "flat" ? (
                    <h3 className="running-apps-section-title">{runningSectionTitle(section)}</h3>
                  ) : null}
                  <div
                    className={`running-apps-grid${section.type === "noWorkspace" ? " running-apps-grid--muted" : ""}`}
                  >
                    {section.items.map((c) => {
                      const rowKey = candidateKey(c);
                      const selected = !!selectedPids[rowKey];
                      const title = displayNameOf(c);
                      const subtitle = windowSubtitle(c);
                      const isProcess = (c.kind ?? "app") === "process";
                      const showLowConfidence =
                        showRunningProcesses && isProcess && c.classificationConfidence === "low";
                      return (
                        <article
                          key={rowKey}
                          className={`running-app-card${selected ? " running-app-card--selected" : ""}${isProcess ? " running-app-card--process" : ""}`}
                          onClick={(e) => onRunningCardSurfaceClick(e, rowKey)}
                        >
                          <div className="running-app-card-head">
                            <label
                              className="running-app-card-check"
                              onClick={(e) => e.stopPropagation()}
                            >
                              <input
                                type="checkbox"
                                checked={selected}
                                onChange={(e) =>
                                  setSelectedPids((prev) => ({
                                    ...prev,
                                    [rowKey]: e.target.checked,
                                  }))
                                }
                                aria-label={`Include ${title} in draft`}
                              />
                              {c.iconName ? (
                                <span
                                  className="running-app-icon"
                                  title={c.iconName}
                                  aria-hidden
                                >
                                  {title.charAt(0).toUpperCase()}
                                </span>
                              ) : (
                                <span className="running-app-icon running-app-icon--placeholder" aria-hidden>
                                  {title.charAt(0).toUpperCase()}
                                </span>
                              )}
                              <span className="running-app-card-titles">
                                <span className="running-app-card-title">{title}</span>
                                {subtitle ? (
                                  <span className="running-app-card-subtitle" title={subtitle}>
                                    {subtitle}
                                  </span>
                                ) : null}
                              </span>
                            </label>
                            {isProcess ? (
                              <span className="running-app-kind-badge">process</span>
                            ) : null}
                            {showLowConfidence ? (
                              <span className="running-app-confidence-badge" title="Low classification confidence">
                                low
                              </span>
                            ) : null}
                          </div>
                          <div className="running-app-card-body">
                            <details className="running-app-details">
                              <summary>Technical details</summary>
                              <dl className="running-app-dl">
                                <div className="running-app-dl-row">
                                  <dt>PID</dt>
                                  <dd>{c.pid}</dd>
                                </div>
                                {c.windowTitle ? (
                                  <div className="running-app-dl-row">
                                    <dt>Window</dt>
                                    <dd>{c.windowTitle}</dd>
                                  </div>
                                ) : null}
                                <div className="running-app-dl-row">
                                  <dt>Executable</dt>
                                  <dd>
                                    <code className="running-app-code">{c.executable}</code>
                                  </dd>
                                </div>
                                <div className="running-app-dl-row">
                                  <dt>Command</dt>
                                  <dd>
                                    <code className="running-app-code running-app-cmd">{c.cmdPreview}</code>
                                  </dd>
                                </div>
                              </dl>
                            </details>
                            <label
                              className="field running-app-cwd-field"
                              onClick={(e) => e.stopPropagation()}
                            >
                              <span>Project folder (optional)</span>
                              <input
                                type="text"
                                value={cwdDraftByPid[rowKey] ?? ""}
                                onChange={(e) =>
                                  setCwdDraftByPid((prev) => ({
                                    ...prev,
                                    [rowKey]: e.target.value,
                                  }))
                                }
                                placeholder={
                                  c.cwdHint
                                    ? `Suggested: ${c.cwdHint}`
                                    : "Leave empty if not needed"
                                }
                                aria-label={`Working directory for ${title}`}
                              />
                            </label>
                          </div>
                        </article>
                      );
                    })}
                  </div>
                </div>
              ))}
              <div className="running-app-draft-actions">
                <button
                  type="button"
                  className="btn-primary btn-add-draft-selected"
                  disabled={busy || selectedRunningCount === 0}
                  onClick={addSelectedRunningToDraft}
                >
                  Add selected to draft
                  {selectedRunningCount > 0 ? ` (${selectedRunningCount})` : ""}
                </button>
              </div>
            </>
          ) : null}
        </section>
      ) : null}

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
            <div className="browser-block-inner">
              {systemBrowserHint?.desktopEntry ? (
                <p className="browser-meta-hint">
                  Desktop default handler: <code>{systemBrowserHint.desktopEntry}</code>
                </p>
              ) : null}
              <label className="field">
                <span>Family</span>
                <select
                  className="browser-family-select"
                  value={browser.family}
                  onChange={(e) =>
                    setBrowserPatch({ family: e.target.value as BrowserFamily })
                  }
                >
                  <option value="chromium_like">Chromium-like (Chrome, Brave, Edge, …)</option>
                  <option value="firefox">Firefox (and derivatives)</option>
                </select>
              </label>
              <div className="browser-executable-row">
                <label className="field browser-executable-field">
                  <span>Browser executable</span>
                  <input
                    type="text"
                    value={browser.executable}
                    onChange={(e) => setBrowserPatch({ executable: e.target.value })}
                    placeholder={browserExecutablePlaceholder}
                    autoComplete="off"
                    spellCheck={false}
                  />
                </label>
                {(systemBrowserHint?.executable || systemBrowserHint?.family) && (
                  <button
                    type="button"
                    className="btn-secondary browser-apply-detected"
                    onClick={() => {
                      const exe = systemBrowserHint?.executable?.trim();
                      const fam = systemBrowserHint?.family;
                      setBrowserPatch({
                        ...(exe ? { executable: exe } : {}),
                        ...(fam ? { family: fam } : {}),
                      });
                    }}
                  >
                    Use detected default
                  </button>
                )}
              </div>
              {browser.family === "chromium_like" ? (
                <label className="field">
                  <span>User data dir (optional)</span>
                  <input
                    type="text"
                    className="browser-subfield-input"
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
                      className="browser-subfield-input"
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
              <div className="field browser-urls-field">
                <div className="browser-urls-label-row">
                  <span>URLs to open</span>
                  <div className="browser-urls-toolbar">
                    <button
                      type="button"
                      className="btn-secondary btn-compact"
                      onClick={addBrowserUrlRow}
                    >
                      Add URL
                    </button>
                    <button
                      type="button"
                      className="btn-secondary btn-compact"
                      onClick={() => void pasteBrowserUrlsFromClipboard()}
                    >
                      Paste from clipboard
                    </button>
                  </div>
                </div>
                <p className="hint browser-urls-hint">
                  One address per row. Long links stay on one line until you focus the field; hover shows the
                  full URL. Empty rows are dropped when you save.
                </p>
                {browser.urls.length === 0 ? (
                  <p className="hint browser-urls-empty">No URLs yet. Add a row or paste several at once.</p>
                ) : null}
                <ul className="browser-url-list" aria-label="Browser URLs">
                  {browser.urls.map((url, i) => (
                    <li key={i} className="browser-url-row">
                      <div className="browser-url-row-main">
                        <span className="browser-url-index">{i + 1}</span>
                        <input
                          type="text"
                          className="browser-url-input"
                          value={url}
                          title={url || undefined}
                          placeholder="https://example.com or file:///…"
                          spellCheck={false}
                          autoComplete="off"
                          aria-label={`URL ${i + 1}`}
                          onChange={(e) => {
                            const next = [...browser.urls];
                            next[i] = e.target.value;
                            setBrowserPatch({ urls: next });
                          }}
                        />
                      </div>
                      <button
                        type="button"
                        className="btn-secondary btn-compact danger browser-url-remove"
                        aria-label={`Remove URL ${i + 1}`}
                        onClick={() =>
                          setBrowserPatch({ urls: browser.urls.filter((_, j) => j !== i) })
                        }
                      >
                        Remove
                      </button>
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          </div>
        ) : null}
      </section>

      <div className="editor-actions">
        <button type="button" className="btn-primary" disabled={busy} onClick={() => void save()}>
          Save profile
        </button>
        <button type="button" className="btn-secondary" disabled={busy} onClick={() => void load()}>
          Reload from disk
        </button>
        <button type="button" className="btn-secondary" disabled={busy || previewBusy} onClick={() => void runDryRun()}>
          {previewBusy ? "Dry run…" : "Dry run (preview argv)"}
        </button>
        <button type="button" className="btn-primary" disabled={busy} onClick={() => void activate()}>
          Activate session
        </button>
      </div>
      <p className="hint activate-hint">
        Activate uses the profile as shown in this editor (including unsaved rows). Save if you want the same JSON on
        disk. Obsidian-style in-sandbox paths (<code>/app/obsidian</code>) are rewritten to{" "}
        <code>flatpak run md.obsidian.Obsidian</code> when you activate; other Flatpak apps may still need an explicit{" "}
        <code>flatpak run …</code> command in the executable field.
      </p>

      {activationError ? <p className="cleanup-msg error">{activationError}</p> : null}

      {previewOpen ? (
        <section className="editor-section activation-preview">
          <div className="section-head row-between">
            <h2>Dry run preview</h2>
            <button type="button" className="btn-secondary btn-compact" onClick={() => setPreviewOpen(false)}>
              Close
            </button>
          </div>
          {previewBusy ? <p className="hint">Loading preview…</p> : null}
          {previewError ? <p className="cleanup-msg error">{previewError}</p> : null}
          {previewSteps && previewSteps.length > 0 ? (
            <ol className="activation-preview-list">
              {previewSteps.map((row, idx) => (
                <li key={`${row.stepType}-${idx}`} className="activation-preview-item">
                  <div className="activation-preview-head">
                    <span className="badge badge-preview-type">{row.stepType}</span>
                    <strong>{row.label}</strong>
                    {row.wouldSkip ? <span className="badge badge-status-skipped">would skip</span> : null}
                  </div>
                  {row.skipDetail ? <p className="hint activation-preview-skip">{row.skipDetail}</p> : null}
                  <pre className="activation-preview-argv">{row.argv.join(" ")}</pre>
                  {row.cwd ? (
                    <p className="hint">
                      cwd: <code>{row.cwd}</code>
                    </p>
                  ) : null}
                </li>
              ))}
            </ol>
          ) : null}
        </section>
      ) : null}

      {activationResult?.steps?.length ? (
        <section className="editor-section activation-results">
          <div className="section-head row-between">
            <h2>Activation results</h2>
            <div className="activation-results-actions">
              <button type="button" className="btn-secondary btn-compact" onClick={() => void openActivationLog()}>
                Abrir log
              </button>
            </div>
          </div>
          <ul className="activation-timeline">
            {activationResult.steps.map((s, idx) => (
              <li
                key={`${s.label}-${idx}`}
                className={`activation-timeline-item activation-timeline-item--${s.status}`}
              >
                <div className="activation-timeline-icon" aria-hidden>
                  {activationTimelineIcon(s)}
                </div>
                <div className="activation-timeline-body">
                  <div className="activation-timeline-title">
                    <span className={`badge badge-kind-${s.kind}`}>{s.kind}</span>
                    <span>{activationStepTitle(s.kind, s.label)}</span>
                    <span className={`badge badge-status-${s.status}`}>{s.status}</span>
                  </div>
                  {s.pid != null ? (
                    <p className="hint activation-timeline-meta">PID {s.pid}</p>
                  ) : null}
                  {s.detail ? <p className="activation-timeline-detail">{s.detail}</p> : null}
                </div>
              </li>
            ))}
          </ul>
        </section>
      ) : null}
    </main>
  );
}
