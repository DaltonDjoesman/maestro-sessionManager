import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { DuplicateProfileResult, ProfileCatalogEntry, SessionProfile } from "../types/profile";
import {
  loadPinnedPaths,
  savePinnedPaths,
} from "../sessionCatalogUi";

interface ProfileCatalogScreenProps {
  profilesRoot: string;
  onEdit: (filePath: string) => void;
}

export function ProfileCatalogScreen({ profilesRoot, onEdit }: ProfileCatalogScreenProps) {
  const [rows, setRows] = useState<ProfileCatalogEntry[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [pins, setPins] = useState<string[]>([]);
  const importInputRef = useRef<HTMLInputElement | null>(null);

  const refresh = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const list = await invoke<ProfileCatalogEntry[]>("list_session_profiles");
      setRows(list);
    } catch (e) {
      setRows([]);
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  useEffect(() => {
    if (!profilesRoot.trim()) {
      setPins([]);
      return;
    }
    setPins(loadPinnedPaths(profilesRoot));
  }, [profilesRoot]);

  const persistPins = (next: string[]) => {
    setPins(next);
    if (profilesRoot.trim()) {
      savePinnedPaths(profilesRoot, next);
    }
  };

  const togglePin = (path: string) => {
    const set = new Set(pins);
    if (set.has(path)) set.delete(path);
    else set.add(path);
    persistPins([...set]);
  };

  const filteredRows = useMemo(() => {
    const q = search.trim().toLowerCase();
    let list = rows;
    if (q) {
      list = rows.filter((r) => {
        const name = (r.name ?? "").toLowerCase();
        const fn = r.fileName.toLowerCase();
        return name.includes(q) || fn.includes(q);
      });
    }
    const pinIndex = (path: string) => {
      const i = pins.indexOf(path);
      return i === -1 ? Number.MAX_SAFE_INTEGER : i;
    };
    return [...list].sort((a, b) => {
      const ap = pinIndex(a.filePath);
      const bp = pinIndex(b.filePath);
      if (ap !== bp) return ap - bp;
      return a.fileName.localeCompare(b.fileName);
    });
  }, [rows, search, pins]);

  const handleCreate = async () => {
    setBusy(true);
    setError(null);
    try {
      const created = await invoke<{ filePath: string }>("create_session_profile", {});
      await refresh();
      onEdit(created.filePath);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const handleDuplicate = async (path: string) => {
    setBusy(true);
    setError(null);
    try {
      const dup = await invoke<DuplicateProfileResult>("duplicate_session_profile", { path });
      await refresh();
      onEdit(dup.filePath);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const handleDuplicateNamed = async (path: string, currentName: string) => {
    const name = window.prompt("Nome para a cópia", `${currentName} (copy)`);
    if (name == null) return;
    const trimmed = name.trim();
    if (!trimmed) {
      setError("Nome vazio.");
      return;
    }
    setBusy(true);
    setError(null);
    try {
      const dup = await invoke<DuplicateProfileResult>("duplicate_session_profile_with_name", {
        path,
        displayName: trimmed,
      });
      await refresh();
      onEdit(dup.filePath);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const handleDelete = async (path: string, label: string) => {
    if (!window.confirm(`Delete profile “${label}”? This cannot be undone.`)) return;
    setBusy(true);
    setError(null);
    try {
      await invoke("delete_session_profile", { path });
      persistPins(pins.filter((p) => p !== path));
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const exportProfile = async (path: string, displayName: string) => {
    setBusy(true);
    setError(null);
    try {
      const profile = await invoke<SessionProfile>("load_session_profile", { path });
      const json = JSON.stringify(profile, null, 2);
      const blob = new Blob([json], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      const safe = displayName.replace(/[^\w\-]+/g, "_").slice(0, 80) || "session";
      a.href = url;
      a.download = `${safe}.json`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const onImportPick = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    e.target.value = "";
    if (!file) return;
    const name = window.prompt("Nome do perfil após importar", file.name.replace(/\.json$/i, "") || "Imported");
    if (name == null) return;
    const trimmed = name.trim();
    if (!trimmed) {
      setError("Nome vazio.");
      return;
    }
    setBusy(true);
    setError(null);
    try {
      const text = await file.text();
      const dup = await invoke<DuplicateProfileResult>("import_session_profile_json", {
        json: text,
        displayName: trimmed,
      });
      await refresh();
      onEdit(dup.filePath);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <main className="container">
      <header className="hero">
        <h1>Session profiles</h1>
        <p className="tagline">JSON profiles under your configured directory</p>
      </header>

      {error ? <p className="field-error" role="alert">{error}</p> : null}

      <div className="catalog-toolbar catalog-toolbar--wrap">
        <button type="button" className="btn-primary" disabled={busy} onClick={() => void handleCreate()}>
          New profile
        </button>
        <button type="button" className="btn-secondary" disabled={busy} onClick={() => void refresh()}>
          Refresh
        </button>
        <button
          type="button"
          className="btn-secondary"
          disabled={busy}
          onClick={() => importInputRef.current?.click()}
        >
          Import JSON…
        </button>
        <input
          ref={importInputRef}
          type="file"
          accept=".json,application/json"
          className="visually-hidden"
          aria-hidden
          onChange={(e) => void onImportPick(e)}
        />
        <label className="catalog-search field catalog-search-field">
          <span className="catalog-search-label">Search</span>
          <input
            type="search"
            placeholder="Filter by name or file…"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            disabled={busy}
          />
        </label>
      </div>

      <div className="catalog-table-wrap">
        <table className="catalog-table">
          <thead>
            <tr>
              <th style={{ width: "3rem" }} aria-label="Pin" />
              <th>Name</th>
              <th>File</th>
              <th>Status</th>
              <th style={{ width: "18rem" }}>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredRows.length === 0 && !busy ? (
              <tr>
                <td colSpan={5} className="hint">
                  {rows.length === 0
                    ? "No JSON profiles found. Create one or check Settings → profiles directory."
                    : "No profiles match this search."}
                </td>
              </tr>
            ) : null}
            {filteredRows.map((r) => {
              const label = r.name ?? r.fileName;
              const pinned = pins.includes(r.filePath);
              return (
                <tr key={r.filePath}>
                  <td>
                    <button
                      type="button"
                      className={`btn-pin${pinned ? " btn-pin--on" : ""}`}
                      disabled={busy || !r.valid}
                      title={pinned ? "Unpin" : "Pin to top"}
                      aria-pressed={pinned}
                      onClick={() => togglePin(r.filePath)}
                    >
                      {pinned ? "★" : "☆"}
                    </button>
                  </td>
                  <td>
                    <div className="catalog-name-cell">
                      <span>{r.name ?? "—"}</span>
                      {r.valid ? (
                        <span className="catalog-badges">
                          {r.browserOnly ? (
                            <span className="badge badge-browser-only">Browser only</span>
                          ) : null}
                          {typeof r.applicationsCount === "number" && r.applicationsCount > 0 ? (
                            <span className="badge badge-apps-count">{r.applicationsCount} apps</span>
                          ) : null}
                        </span>
                      ) : null}
                    </div>
                  </td>
                  <td>
                    <code className="file-path">{r.fileName}</code>
                  </td>
                  <td>
                    {r.valid ? (
                      <span className="badge badge-ok">valid</span>
                    ) : (
                      <span className="badge badge-bad" title={r.error ?? ""}>
                        invalid
                      </span>
                    )}
                  </td>
                  <td className="catalog-actions catalog-actions--wrap">
                    <button
                      type="button"
                      className="btn-secondary btn-compact"
                      disabled={busy || !r.valid}
                      onClick={() => onEdit(r.filePath)}
                    >
                      Edit
                    </button>
                    <button
                      type="button"
                      className="btn-secondary btn-compact"
                      disabled={busy || !r.valid}
                      onClick={() => void exportProfile(r.filePath, label)}
                    >
                      Export
                    </button>
                    <button
                      type="button"
                      className="btn-secondary btn-compact"
                      disabled={busy || !r.valid}
                      onClick={() => void handleDuplicate(r.filePath)}
                    >
                      Duplicate
                    </button>
                    <button
                      type="button"
                      className="btn-secondary btn-compact"
                      disabled={busy || !r.valid}
                      onClick={() => void handleDuplicateNamed(r.filePath, label)}
                    >
                      Duplicate…
                    </button>
                    <button
                      type="button"
                      className="btn-secondary btn-compact danger"
                      disabled={busy}
                      onClick={() => void handleDelete(r.filePath, label)}
                    >
                      Delete
                    </button>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
      {busy ? <p className="hint">Working…</p> : null}
    </main>
  );
}
