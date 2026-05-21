import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ProfileCatalogEntry } from "../types/profile";

interface ProfileCatalogScreenProps {
  onBack: () => void;
  onEdit: (filePath: string) => void;
}

export function ProfileCatalogScreen({ onBack, onEdit }: ProfileCatalogScreenProps) {
  const [rows, setRows] = useState<ProfileCatalogEntry[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

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
      const dup = await invoke<{ filePath: string }>("duplicate_session_profile", { path });
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
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <main className="container">
      <header className="hero row-between">
        <div>
          <h1>Session profiles</h1>
          <p className="tagline">JSON profiles under your configured directory</p>
        </div>
        <div className="header-actions">
          <button type="button" className="btn-secondary" onClick={onBack}>
            Home
          </button>
        </div>
      </header>

      {error ? <p className="cleanup-msg error">{error}</p> : null}

      <div className="catalog-toolbar">
        <button type="button" className="btn-primary" disabled={busy} onClick={() => void handleCreate()}>
          New profile
        </button>
        <button type="button" className="btn-secondary" disabled={busy} onClick={() => void refresh()}>
          Refresh
        </button>
      </div>

      <div className="catalog-table-wrap">
        <table className="catalog-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>File</th>
              <th>Status</th>
              <th style={{ width: "14rem" }}>Actions</th>
            </tr>
          </thead>
          <tbody>
            {rows.length === 0 && !busy ? (
              <tr>
                <td colSpan={4} className="hint">
                  No JSON profiles found. Create one or check Settings → profiles directory.
                </td>
              </tr>
            ) : null}
            {rows.map((r) => {
              const label = r.name ?? r.fileName;
              return (
                <tr key={r.filePath}>
                  <td>{r.name ?? "—"}</td>
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
                  <td className="catalog-actions">
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
                      onClick={() => void handleDuplicate(r.filePath)}
                    >
                      Duplicate
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
