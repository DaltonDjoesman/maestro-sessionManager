import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ProfileCatalogScreen } from "./components/ProfileCatalogScreen";
import { ProfileEditorScreen } from "./components/ProfileEditorScreen";
import { SettingsScreen } from "./components/SettingsScreen";
import type { ApplicationSettings } from "./types/settings";
import type { CleanupDivergenceRow, CleanupTerminateResult } from "./types/cleanup";
import "./App.css";

type View = "home" | "settings" | "profiles" | "profile-editor";

function App() {
  const [view, setView] = useState<View>("home");
  const [platform, setPlatform] = useState("…");
  const [profilesRoot, setProfilesRoot] = useState("…");
  const [editorPath, setEditorPath] = useState<string | null>(null);

  const [cleanupPath, setCleanupPath] = useState("");
  const [divergences, setDivergences] = useState<CleanupDivergenceRow[]>([]);
  const [selectedPids, setSelectedPids] = useState<Set<number>>(new Set());
  const [cleanupBusy, setCleanupBusy] = useState(false);
  const [cleanupError, setCleanupError] = useState<string | null>(null);
  const [cleanupOutcome, setCleanupOutcome] = useState<CleanupTerminateResult[] | null>(null);
  const [forceKill, setForceKill] = useState(false);
  const [forceKillAfterMs, setForceKillAfterMs] = useState(1500);

  useEffect(() => {
    if (view !== "home") return;

    invoke<string>("platform_name")
      .then(setPlatform)
      .catch(() => setPlatform("unavailable"));

    invoke<ApplicationSettings>("get_settings")
      .then((s) => setProfilesRoot(s.profiles_root))
      .catch(() => setProfilesRoot("unavailable"));
  }, [view]);

  useEffect(() => {
    if (view === "profile-editor" && !editorPath) {
      setView("profiles");
    }
  }, [view, editorPath]);

  const scanDivergences = useCallback(async () => {
    setCleanupError(null);
    setCleanupOutcome(null);
    setCleanupBusy(true);
    setSelectedPids(new Set());
    try {
      const rows = await invoke<CleanupDivergenceRow[]>("list_cleanup_divergences", {
        path: cleanupPath.trim(),
      });
      setDivergences(rows);
    } catch (e) {
      setDivergences([]);
      setCleanupError(String(e));
    } finally {
      setCleanupBusy(false);
    }
  }, [cleanupPath]);

  const togglePid = useCallback((pid: number) => {
    setSelectedPids((prev) => {
      const next = new Set(prev);
      if (next.has(pid)) next.delete(pid);
      else next.add(pid);
      return next;
    });
  }, []);

  const runTermination = useCallback(async () => {
    if (selectedPids.size === 0) return;
    const ok = window.confirm(
      `Send SIGTERM to ${selectedPids.size} selected process(es)?${
        forceKill
          ? `\n\nIf a process is still running after ${forceKillAfterMs} ms, SIGKILL will be sent (force kill opt-in).`
          : ""
      }`,
    );
    if (!ok) return;

    setCleanupBusy(true);
    setCleanupError(null);
    setCleanupOutcome(null);
    try {
      const pids = [...selectedPids];
      const results = await invoke<CleanupTerminateResult[]>("cleanup_terminate_processes", {
        pids,
        force_kill_after_ms: forceKill ? forceKillAfterMs : null,
      });
      setCleanupOutcome(results);
      await scanDivergences();
    } catch (e) {
      setCleanupError(String(e));
    } finally {
      setCleanupBusy(false);
    }
  }, [selectedPids, forceKill, forceKillAfterMs, scanDivergences]);

  if (view === "settings") {
    return <SettingsScreen onBack={() => setView("home")} />;
  }

  if (view === "profiles") {
    return (
      <ProfileCatalogScreen
        onBack={() => setView("home")}
        onEdit={(path) => {
          setEditorPath(path);
          setView("profile-editor");
        }}
      />
    );
  }

  if (view === "profile-editor" && editorPath) {
    return (
      <ProfileEditorScreen
        filePath={editorPath}
        onBack={() => {
          setView("profiles");
          setEditorPath(null);
        }}
      />
    );
  }

  return (
    <main className="container">
      <header className="hero row-between">
        <div>
          <h1>Maestro</h1>
          <p className="tagline">Session environment manager for Linux</p>
        </div>
        <div className="header-actions">
          <button type="button" className="btn-secondary" onClick={() => setView("profiles")}>
            Profiles
          </button>
          <button type="button" className="btn-secondary" onClick={() => setView("settings")}>
            Settings
          </button>
        </div>
      </header>
      <section className="status-card">
        <p>
          Platform adapter: <strong>{platform}</strong>
        </p>
        <p>
          Profiles directory: <strong>{profilesRoot}</strong>
        </p>
        <p className="hint">Open Profiles to list, edit, duplicate, delete, and activate sessions.</p>
      </section>

      <section className="cleanup-card">
        <h2>Context cleanup (divergences)</h2>
        <p className="hint" style={{ marginTop: 0 }}>
          Path is relative to your configured profiles directory (same as other profile commands).
          Only processes not allowed by the profile are listed. Termination requires explicit
          confirmation per action.
        </p>
        <div className="cleanup-row">
          <input
            type="text"
            placeholder="e.g. my-session.json"
            value={cleanupPath}
            onChange={(e) => setCleanupPath(e.target.value)}
            aria-label="Profile file path"
          />
          <button
            type="button"
            className="btn-primary"
            disabled={cleanupBusy || !cleanupPath.trim()}
            onClick={() => void scanDivergences()}
          >
            Scan divergences
          </button>
        </div>
        {cleanupError ? <p className="cleanup-msg error">{cleanupError}</p> : null}
        <div className="cleanup-table-wrap">
          <table className="cleanup-table">
            <thead>
              <tr>
                <th style={{ width: "2rem" }} />
                <th>PID</th>
                <th>Exe</th>
                <th>Command</th>
              </tr>
            </thead>
            <tbody>
              {divergences.length === 0 ? (
                <tr>
                  <td colSpan={4} className="cleanup-msg">
                    {cleanupBusy ? "Loading…" : "No rows (scan a profile or none divergent)."}
                  </td>
                </tr>
              ) : (
                divergences.map((row) => (
                  <tr key={row.pid}>
                    <td>
                      <input
                        type="checkbox"
                        checked={selectedPids.has(row.pid)}
                        onChange={() => togglePid(row.pid)}
                        aria-label={`Select PID ${row.pid}`}
                      />
                    </td>
                    <td>{row.pid}</td>
                    <td>
                      <code>{row.executableBasename}</code>
                    </td>
                    <td>
                      <code>{row.cmdPreview}</code>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
        <div className="cleanup-actions">
          <button
            type="button"
            className="btn-primary"
            disabled={cleanupBusy || selectedPids.size === 0}
            onClick={() => void runTermination()}
          >
            Terminate selected…
          </button>
          <label>
            <input
              type="checkbox"
              checked={forceKill}
              onChange={(e) => setForceKill(e.target.checked)}
            />
            Force kill after timeout
          </label>
          <input
            type="number"
            min={0}
            step={100}
            value={forceKillAfterMs}
            disabled={!forceKill}
            onChange={(e) => setForceKillAfterMs(Number(e.target.value) || 0)}
            aria-label="Milliseconds before SIGKILL"
            style={{ maxWidth: "7rem" }}
          />
          <span className="cleanup-msg">ms after SIGTERM</span>
        </div>
        {cleanupOutcome ? (
          <ul className="cleanup-msg" style={{ paddingLeft: "1.1rem" }}>
            {cleanupOutcome.map((r) => (
              <li key={r.pid}>
                <strong>{r.pid}</strong>: {r.ok ? "ok" : "failed"} — {r.message}
              </li>
            ))}
          </ul>
        ) : null}
      </section>
    </main>
  );
}

export default App;
