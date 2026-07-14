import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { pt } from "../i18n/pt";
import {
  emptyBrowserSettings,
  inferBrowserFamily,
  isBrowserApp,
  normalizeApplicationEntry,
  normalizeBrowserUrlList,
  browserSettingsOf,
} from "../browserDetect";
import { ApplicationBrowserFields } from "./ApplicationBrowserFields";
import { RunningAppsCaptureList, launchEntriesFromCandidates } from "./RunningAppsCaptureList";
import { RefreshIconButton } from "./RefreshIconButton";
import type { ActivateSessionResult } from "../types/activation";
import type { RunningAppCandidate } from "../types/capture";
import { candidateKey, sortByDisplayName } from "../types/capture";
import type {
  ApplicationBrowserSettings,
  ApplicationLaunchEntry,
  SessionProfile,
} from "../types/profile";
import type { ApplicationSettings, BrowserFamily, SystemDefaultBrowserHint } from "../types/settings";
import {
  loadLastSessionPath,
  loadPinnedPaths,
  saveLastSessionPath,
  savePinnedPaths,
} from "../sessionCatalogUi";

type EditorTab = "content" | "capture";

interface ProfileEditorScreenProps {
  filePath: string;
  profilesRoot: string;
  onBack: () => void;
  onActivated: (label: string) => void;
  onDeleted?: (label: string) => void;
}

function cloneProfile(p: SessionProfile): SessionProfile {
  return JSON.parse(JSON.stringify(p)) as SessionProfile;
}

function normalizeProfile(p: SessionProfile): SessionProfile {
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

function appDisplayLabel(app: ApplicationLaunchEntry, index: number): string {
  const exe = app.executable.trim();
  if (!exe) return pt.editor.appN(index + 1);
  const base = exe.split(/[/\\]/).pop() ?? exe;
  const cleaned = base.replace(/\.(AppImage|app)$/i, "");
  if (!cleaned) return pt.editor.appN(index + 1);
  return cleaned.charAt(0).toUpperCase() + cleaned.slice(1);
}

export function ProfileEditorScreen({
  filePath,
  profilesRoot,
  onBack,
  onActivated,
  onDeleted,
}: ProfileEditorScreenProps) {
  const [profile, setProfile] = useState<SessionProfile | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [saveToastVisible, setSaveToastVisible] = useState(false);
  const titleInputRef = useRef<HTMLInputElement>(null);
  const titleBeforeEditRef = useRef("");
  const [editingTitle, setEditingTitle] = useState(false);
  const [busy, setBusy] = useState(false);
  const [editorTab, setEditorTab] = useState<EditorTab>("content");
  const [runningApps, setRunningApps] = useState<RunningAppCandidate[]>([]);
  const [runningBusy, setRunningBusy] = useState(false);
  const [runningErr, setRunningErr] = useState<string | null>(null);
  const [selectedPids, setSelectedPids] = useState<Record<string, boolean>>({});
  const [captureSearch, setCaptureSearch] = useState("");
  const [expandedAppIndex, setExpandedAppIndex] = useState<number | null>(null);
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

  const selectedRunningCount = useMemo(
    () => runningApps.reduce((n, c) => n + (selectedPids[candidateKey(c)] ? 1 : 0), 0),
    [runningApps, selectedPids],
  );

  const showSavedFeedback = useCallback(() => {
    setSaveToastVisible(true);
  }, []);

  useEffect(() => {
    if (editingTitle) {
      titleInputRef.current?.focus();
      titleInputRef.current?.select();
    }
  }, [editingTitle]);

  const load = useCallback(async () => {
    setBusy(true);
    setLoadError(null);
    setSaveError(null);
    try {
      const [p, settings, hint] = await Promise.all([
        invoke<SessionProfile>("load_session_profile", { path: filePath }),
        invoke<ApplicationSettings>("get_settings"),
        invoke<SystemDefaultBrowserHint>("detect_system_default_browser"),
      ]);
      setProfile(normalizeProfile(cloneProfile(p)));
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
      setSelectedPids({});
    } catch (e) {
      setRunningErr(String(e));
    } finally {
      setRunningBusy(false);
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  useEffect(() => {
    if (editorTab !== "capture") return;
    void refreshRunningApps();
  }, [editorTab, refreshRunningApps]);

  const updateProfile = (patch: Partial<SessionProfile>) => {
    setProfile((prev) => (prev ? { ...prev, ...patch } : prev));
  };

  const setBrowserPatch = (index: number, patch: Partial<ApplicationBrowserSettings>) => {
    setProfile((prev) => {
      if (!prev) return prev;
      const apps = prev.applications.map((a, i) => {
        if (i !== index) return a;
        const family = inferBrowserFamily(a.executable) ?? "chromium_like";
        const b = a.browser ?? emptyBrowserSettings(family);
        return { ...a, browser: { ...b, ...patch } };
      });
      return { ...prev, applications: apps };
    });
  };

  const profileForPersist = (source: SessionProfile): SessionProfile => {
    const normalized = normalizeProfile(source);
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
  };

  const save = async () => {
    if (!profile) return;
    setBusy(true);
    setSaving(true);
    setSaveError(null);
    setSaveToastVisible(false);
    try {
      const toSave = profileForPersist(profile);
      await invoke("save_session_profile", { path: filePath, profile: toSave });
      setProfile(cloneProfile(toSave));
      showSavedFeedback();
    } catch (e) {
      setSaveError(String(e));
    } finally {
      setBusy(false);
      setSaving(false);
    }
  };

  const sessionLabel = profile?.name?.trim() || filePath.split(/[/\\]/).pop() || filePath;

  const profileForRun = (): SessionProfile | null => {
    if (!profile) return null;
    return profileForPersist(profile);
  };

  const activate = async () => {
    const toActivate = profileForRun();
    if (!toActivate) return;
    setBusy(true);
    setSaveError(null);
    try {
      await invoke<ActivateSessionResult>("activate_session_profile", {
        path: filePath,
        profile: toActivate,
      });
      onActivated(sessionLabel);
    } catch (e) {
      setSaveError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const deleteSession = async () => {
    if (!profile) return;
    if (!window.confirm(pt.hub.deleteConfirm(sessionLabel))) return;
    setBusy(true);
    setSaveError(null);
    try {
      await invoke("delete_session_profile", { path: filePath });
      const root = profilesRoot.trim();
      if (root) {
        savePinnedPaths(
          root,
          loadPinnedPaths(root).filter((p) => p !== filePath),
        );
        if (loadLastSessionPath(root) === filePath) {
          saveLastSessionPath(root, null);
        }
      }
      onDeleted?.(sessionLabel);
      onBack();
    } catch (e) {
      setSaveError(String(e));
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
        browser: null,
      };
      return { ...prev, applications: [...prev.applications, next] };
    });
    setExpandedAppIndex(profile?.applications.length ?? 0);
  };

  const removeApp = (index: number) => {
    setProfile((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        applications: prev.applications.filter((_, i) => i !== index),
      };
    });
    setExpandedAppIndex((prev) => {
      if (prev === null) return null;
      if (prev === index) return null;
      if (prev > index) return prev - 1;
      return prev;
    });
  };

  const toggleAppExpanded = (index: number) => {
    setExpandedAppIndex((prev) => (prev === index ? null : index));
  };

  const patchApp = (index: number, patch: Partial<ApplicationLaunchEntry>) => {
    setProfile((prev) => {
      if (!prev) return prev;
      const apps = prev.applications.map((a, i) => {
        if (i !== index) return a;
        return normalizeApplicationEntry({ ...a, ...patch });
      });
      return { ...prev, applications: apps };
    });
  };

  const addSelectedRunningToDraft = () => {
    const chosen = sortByDisplayName(runningApps).filter((c) => selectedPids[candidateKey(c)]);
    if (chosen.length === 0 || !profile) return;
    const startIndex = profile.applications.length;
    setProfile((prev) => {
      if (!prev) return prev;
      return { ...prev, applications: [...prev.applications, ...launchEntriesFromCandidates(chosen)] };
    });
    setSelectedPids({});
    setEditorTab("content");
    setExpandedAppIndex(startIndex);
  };

  const toggleCaptureKey = (key: string, selected?: boolean) => {
    setSelectedPids((prev) => ({ ...prev, [key]: selected ?? !prev[key] }));
  };

  const mergeCaptureSelection = (patch: Record<string, boolean>) => {
    setSelectedPids((prev) => ({ ...prev, ...patch }));
  };

  const displayTitle = profile?.name?.trim() || pt.editor.title;

  const startEditingTitle = () => {
    if (!profile || busy) return;
    titleBeforeEditRef.current = profile.name;
    setEditingTitle(true);
  };

  const finishEditingTitle = () => {
    setEditingTitle(false);
  };

  const cancelEditingTitle = () => {
    updateProfile({ name: titleBeforeEditRef.current });
    setEditingTitle(false);
  };

  if (loadError) {
    return (
      <div className="page profile-editor-page">
        <header className="editor-sticky-header">
          <button type="button" className="btn-secondary btn-compact" onClick={onBack}>
            ← {pt.editor.back}
          </button>
        </header>
        <p className="cleanup-msg error">{loadError}</p>
      </div>
    );
  }

  if (!profile) {
    return (
      <div className="page profile-editor-page">
        <p className="hint">{busy ? pt.editor.loading : "—"}</p>
      </div>
    );
  }

  const captureTab = (
    <section className="form-section capture-section">
      <div className="capture-section-header">
        <h2 className="form-section-title capture-section-title">{pt.editor.tabCapture}</h2>
        <RefreshIconButton
          onClick={() => void refreshRunningApps()}
          disabled={busy || runningBusy}
          busy={runningBusy}
        />
      </div>
      <p className="view-subtitle">{pt.capture.editorHint}</p>
      <RunningAppsCaptureList
        apps={runningApps}
        search={captureSearch}
        onSearchChange={setCaptureSearch}
        selectedKeys={selectedPids}
        onToggleKey={toggleCaptureKey}
        onBulkSelect={mergeCaptureSelection}
        busy={busy || runningBusy}
        error={runningErr}
        footer={
          <div className="capture-list-footer">
            <button
              type="button"
              className="btn btn-primary btn-compact"
              disabled={busy || runningBusy || selectedRunningCount === 0}
              onClick={addSelectedRunningToDraft}
            >
              {pt.capture.addToDraft}
            </button>
          </div>
        }
      />
    </section>
  );

  return (
    <div className="page profile-editor-page">
      <header className="editor-header">
        <div className="view-title-group" style={{ marginBottom: 0 }}>
          <h1 className="view-title editor-inline-title">
            {editingTitle ? (
              <input
                ref={titleInputRef}
                type="text"
                className="editor-title-input"
                value={profile.name}
                aria-label={pt.editor.name}
                placeholder={pt.editor.namePlaceholder}
                onChange={(e) => updateProfile({ name: e.target.value })}
                onBlur={finishEditingTitle}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    e.preventDefault();
                    (e.currentTarget as HTMLInputElement).blur();
                  }
                  if (e.key === "Escape") {
                    e.preventDefault();
                    cancelEditingTitle();
                  }
                }}
              />
            ) : (
              <button
                type="button"
                className="editor-title-button"
                onClick={startEditingTitle}
                title={pt.editor.renameTitle}
              >
                {displayTitle}
              </button>
            )}
          </h1>
          <p className="view-subtitle">{pt.editor.activateHint}</p>
        </div>
        <div className="editor-sticky-actions">
          <div className="editor-sticky-actions-start">
            {saveToastVisible ? (
              <div className="save-toast" role="status">
                <span className="save-toast-text">{pt.editor.savedToast}</span>
                <button
                  type="button"
                  className="save-toast-close"
                  aria-label={pt.editor.dismissToast}
                  onClick={() => setSaveToastVisible(false)}
                >
                  ×
                </button>
              </div>
            ) : null}
            <button
              type="button"
              className="btn btn-primary btn-compact"
              disabled={busy}
              onClick={() => void save()}
            >
              {saving ? pt.editor.saving : pt.editor.save}
            </button>
            <button type="button" className="btn btn-activate btn-compact" disabled={busy} onClick={() => void activate()}>
              {pt.editor.activate}
            </button>
          </div>
          <div className="editor-sticky-actions-end">
            <button
              type="button"
              className="btn btn-secondary btn-compact danger"
              disabled={busy}
              onClick={() => void deleteSession()}
            >
              {pt.hub.delete}
            </button>
          </div>
        </div>
      </header>

      {saveError ? <p className="field-error">{saveError}</p> : null}

      <nav className="editor-tab-nav" aria-label="Secções do editor">
        <button
          type="button"
          className={`tab-btn${editorTab === "content" ? " active" : ""}`}
          onClick={() => setEditorTab("content")}
        >
          {pt.editor.tabContent}
        </button>
        <button
          type="button"
          className={`tab-btn${editorTab === "capture" ? " active" : ""}`}
          onClick={() => setEditorTab("capture")}
        >
          {pt.editor.tabCapture}
        </button>
      </nav>

      {editorTab === "content" ? (
        <>
      <section className="form-section">
        <div className="section-head">
          <h2 className="form-section-title" style={{ marginBottom: 0, borderBottom: "none", paddingBottom: 0 }}>
            {pt.editor.applications}
          </h2>
          <button type="button" className="btn btn-secondary btn-compact" onClick={addApp}>
            {pt.editor.addApp}
          </button>
        </div>
        {profile.applications.length === 0 ? (
          <p className="hint">{pt.editor.noApps}</p>
        ) : null}
        {profile.applications.map((app, i) => {
          const browserApp = isBrowserApp(app);
          const browserSettings = browserSettingsOf(app);
          const expanded = expandedAppIndex === i;
          const label = appDisplayLabel(app, i);
          return (
          <div
            key={i}
            className={`app-card${browserApp ? " app-card--browser" : ""}${expanded ? " app-card--expanded" : " app-card--collapsed"}`}
          >
            <button
              type="button"
              className="app-card-summary"
              aria-expanded={expanded}
              onClick={() => toggleAppExpanded(i)}
            >
              <span className="app-card-chevron" aria-hidden>
                {expanded ? "▾" : "▸"}
              </span>
              <span className="app-card-name">{label}</span>
              {browserApp ? <span className="badge badge-gray">Browser</span> : null}
            </button>
            {expanded ? (
            <div className="app-card-body">
            <div className="app-card-actions">
              <button type="button" className="btn btn-secondary btn-compact danger" onClick={() => removeApp(i)}>
                {pt.editor.remove}
              </button>
            </div>
            <label className="field">
              <span>{pt.editor.executable}</span>
              <input
                type="text"
                value={app.executable}
                onChange={(e) => patchApp(i, { executable: e.target.value })}
                placeholder={browserApp ? browserExecutablePlaceholder : "/usr/bin/code or code"}
              />
            </label>
            {browserApp && browserSettings ? (
              <ApplicationBrowserFields
                browser={browserSettings}
                onPatch={(patch) => setBrowserPatch(i, patch)}
              />
            ) : (
              <>
                <label className="field">
                  <span>{pt.editor.args}</span>
                  <textarea
                    rows={3}
                    value={(app.args ?? []).join("\n")}
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
                  <span>{pt.editor.cwd}</span>
                  <input
                    type="text"
                    value={app.cwd ?? ""}
                    onChange={(e) =>
                      patchApp(i, { cwd: e.target.value.trim() ? e.target.value : null })
                    }
                  />
                </label>
              </>
            )}
            <label className="field-inline">
              <input
                type="checkbox"
                checked={app.skip_if_running === true}
                onChange={(e) =>
                  patchApp(i, { skip_if_running: e.target.checked ? true : null })
                }
              />
              <span>{pt.editor.skipIfRunning}</span>
            </label>
            </div>
            ) : null}
          </div>
          );
        })}
      </section>
        </>
      ) : null}

      {editorTab === "capture" ? captureTab : null}
    </div>
  );
}
