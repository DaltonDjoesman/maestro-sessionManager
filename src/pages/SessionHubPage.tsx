import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { pt } from "../i18n/pt";
import {
  loadLastSessionPath,
  loadPinnedPaths,
  saveLastSessionPath,
  savePinnedPaths,
} from "../sessionCatalogUi";
import type { ActivateSessionResult } from "../types/activation";
import { normalizeProfileForRun } from "../profileNormalize";
import type {
  DuplicateProfileResult,
  ProfileCatalogEntry,
  SessionProfile,
} from "../types/profile";

interface SessionHubPageProps {
  profilesRoot: string;
  onEdit: (filePath: string) => void;
  onActivated: (label: string) => void;
}

export function SessionHubPage({ profilesRoot, onEdit, onActivated }: SessionHubPageProps) {
  const [rows, setRows] = useState<ProfileCatalogEntry[]>([]);
  const [busy, setBusy] = useState(false);
  const [activatingPath, setActivatingPath] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [pins, setPins] = useState<string[]>([]);
  const [openMenuPath, setOpenMenuPath] = useState<string | null>(null);
  const importFileRef = useRef<HTMLInputElement>(null);
  const menuRef = useRef<HTMLDivElement | null>(null);

  const lastSessionPath = profilesRoot.trim() ? loadLastSessionPath(profilesRoot) : null;

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

  useEffect(() => {
    if (!openMenuPath) return;
    const onDoc = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setOpenMenuPath(null);
      }
    };
    document.addEventListener("mousedown", onDoc);
    return () => document.removeEventListener("mousedown", onDoc);
  }, [openMenuPath]);

  const persistPins = (next: string[]) => {
    setPins(next);
    if (profilesRoot.trim()) savePinnedPaths(profilesRoot, next);
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

  const lastSessionRow = useMemo(() => {
    if (!lastSessionPath) return null;
    return rows.find((r) => r.filePath === lastSessionPath && r.valid) ?? null;
  }, [rows, lastSessionPath]);

  const excludeContinue = (r: ProfileCatalogEntry) =>
    lastSessionRow && !search.trim() && r.filePath === lastSessionPath;

  const pinnedRows = useMemo(
    () => filteredRows.filter((r) => pins.includes(r.filePath) && !excludeContinue(r)),
    [filteredRows, pins, lastSessionRow, lastSessionPath, search],
  );
  const unpinnedRows = useMemo(
    () => filteredRows.filter((r) => !pins.includes(r.filePath) && !excludeContinue(r)),
    [filteredRows, pins, lastSessionRow, lastSessionPath, search],
  );

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

  const activateProfile = async (entry: ProfileCatalogEntry) => {
    const label = entry.name ?? entry.fileName;
    setActivatingPath(entry.filePath);
    setError(null);
    try {
      const profile = await invoke<SessionProfile>("load_session_profile", { path: entry.filePath });
      await invoke<ActivateSessionResult>("activate_session_profile", {
        path: entry.filePath,
        profile: normalizeProfileForRun(profile),
      });
      if (profilesRoot.trim()) saveLastSessionPath(profilesRoot, entry.filePath);
      onActivated(label);
    } catch (e) {
      setError(String(e));
    } finally {
      setActivatingPath(null);
    }
  };

  const handleDuplicateNamed = async (path: string, currentName: string) => {
    setOpenMenuPath(null);
    const name = window.prompt(pt.hub.duplicatePrompt, `${currentName} (cópia)`);
    if (name == null) return;
    const trimmed = name.trim();
    if (!trimmed) {
      setError(pt.hub.emptyName);
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
    setOpenMenuPath(null);
    if (!window.confirm(pt.hub.deleteConfirm(label))) return;
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
    setOpenMenuPath(null);
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

  const handleImport = async (json: string, displayName: string) => {
    setBusy(true);
    setError(null);
    try {
      const dup = await invoke<DuplicateProfileResult>("import_session_profile_json", {
        json,
        displayName,
      });
      await refresh();
      onEdit(dup.filePath);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const onImportFileSelected = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    e.target.value = "";
    if (!file) return;

    setError(null);
    let json: string;
    try {
      json = await file.text();
      JSON.parse(json);
    } catch {
      setError(pt.import.invalidJson);
      return;
    }

    const defaultName = file.name.replace(/\.json$/i, "") || "Importado";
    const name = window.prompt(pt.hub.importPrompt, defaultName);
    if (name == null) return;
    const trimmed = name.trim();
    if (!trimmed) {
      setError(pt.hub.emptyName);
      return;
    }

    await handleImport(json, trimmed);
  };

  const renderCard = (r: ProfileCatalogEntry, opts?: { highlight?: boolean }) => {
    const label = r.name ?? r.fileName;
    const pinned = pins.includes(r.filePath);
    const isActivating = activatingPath === r.filePath;
    const menuOpen = openMenuPath === r.filePath;

    return (
      <article
        key={r.filePath}
        className={`profile-card${!r.valid ? " profile-card--invalid" : ""}${opts?.highlight ? " profile-card--highlight" : ""}`}
      >
        <div
          className={`card-body${r.valid ? " profile-card--clickable" : ""}`}
          role={r.valid ? "button" : undefined}
          tabIndex={r.valid ? 0 : undefined}
          onClick={r.valid ? () => onEdit(r.filePath) : undefined}
          onKeyDown={
            r.valid
              ? (e) => {
                  if (e.key === "Enter" || e.key === " ") {
                    e.preventDefault();
                    onEdit(r.filePath);
                  }
                }
              : undefined
          }
        >
        <div className="card-top">
          <div className="card-title-group">
            <h3 className="card-name">{label}</h3>
          </div>
          <button
            type="button"
            className={`card-btn${pinned ? " card-btn-active" : ""}`}
            disabled={busy || !r.valid}
            title={pinned ? pt.hub.unpin : pt.hub.pin}
            aria-pressed={pinned}
            onClick={(e) => {
              e.stopPropagation();
              togglePin(r.filePath);
            }}
          >
            ★
          </button>
        </div>

        <div className="card-badges">
          {r.valid ? (
            <>
              {(r.applicationsCount ?? 0) > 0 ? (
                <span className="badge badge-gray">{pt.hub.appsCount(r.applicationsCount!)}</span>
              ) : (
                <span className="badge badge-gray">{pt.hub.emptyContent}</span>
              )}
              {r.hasBrowser ? <span className="badge badge-accent">{pt.hub.hasBrowser}</span> : null}
            </>
          ) : (
            <span className="badge badge-danger" title={r.error ?? ""}>
              {pt.hub.invalid}
            </span>
          )}
        </div>
        </div>

        <div className="card-actions" onClick={(e) => e.stopPropagation()}>
          <div className="card-action-btns">
            <button
              type="button"
              className="btn btn-activate btn-compact"
              disabled={busy || !r.valid || isActivating}
              onClick={() => void activateProfile(r)}
            >
              {isActivating ? pt.hub.working : pt.hub.activate}
            </button>
          </div>
          <div className="overflow-menu-wrapper" ref={menuOpen ? menuRef : undefined}>
            <button
              type="button"
              className="card-btn"
              aria-label={pt.hub.more}
              aria-expanded={menuOpen}
              disabled={busy}
              onClick={() => setOpenMenuPath(menuOpen ? null : r.filePath)}
            >
              ···
            </button>
            {menuOpen ? (
              <div className="overflow-menu" role="menu">
                <button type="button" className="overflow-menu-item" role="menuitem" disabled={!r.valid} onClick={() => void exportProfile(r.filePath, label)}>
                  {pt.hub.export}
                </button>
                <button type="button" className="overflow-menu-item" role="menuitem" disabled={!r.valid} onClick={() => void handleDuplicateNamed(r.filePath, label)}>
                  {pt.hub.duplicateNamed}
                </button>
                <button type="button" className="overflow-menu-item danger" role="menuitem" onClick={() => void handleDelete(r.filePath, label)}>
                  {pt.hub.delete}
                </button>
              </div>
            ) : null}
          </div>
        </div>
      </article>
    );
  };

  const renderSection = (title: string, items: ProfileCatalogEntry[], highlightFirst?: boolean) => {
    if (items.length === 0) return null;
    return (
      <section className="hub-section">
        <h2 className="section-title">{title}</h2>
        <div className="profile-grid">{items.map((r, i) => renderCard(r, { highlight: highlightFirst && i === 0 }))}</div>
      </section>
    );
  };

  return (
    <div className="page hub-page">
      <div className="view-title-group">
        <h1 className="view-title">{pt.hub.title}</h1>
        <p className="view-subtitle">{pt.hub.tagline}</p>
      </div>

      {error ? (
        <p className="field-error" role="alert">
          {error}
        </p>
      ) : null}

      <div className="hub-toolbar">
        <div className="search-input-wrapper">
          <svg className="search-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2" aria-hidden>
            <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <input
            type="search"
            className="search-input"
            placeholder={pt.hub.searchPlaceholder}
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            disabled={busy}
            aria-label={pt.hub.search}
          />
        </div>
        <div className="hub-toolbar-actions">
          <input
            ref={importFileRef}
            type="file"
            accept=".json,application/json"
            className="visually-hidden"
            onChange={(e) => void onImportFileSelected(e)}
          />
          <button
            type="button"
            className="btn btn-secondary"
            disabled={busy}
            onClick={() => importFileRef.current?.click()}
          >
            {pt.hub.import}
          </button>
          <button type="button" className="btn btn-primary" disabled={busy} onClick={() => void handleCreate()}>
            {pt.hub.newSession}
          </button>
        </div>
      </div>

      {lastSessionRow && !search.trim() ? renderSection(pt.hub.continue, [lastSessionRow], true) : null}
      {pinnedRows.length > 0 ? renderSection(pt.hub.pinned, pinnedRows) : null}

      <section className="hub-section">
        <h2 className="section-title">
          {pt.hub.all} ({unpinnedRows.length + pinnedRows.length + (lastSessionRow && !search.trim() ? 1 : 0)})
        </h2>
        <div className="profile-grid">
          {unpinnedRows.length === 0 && !busy ? (
            <p className="view-subtitle hub-empty">{rows.length === 0 ? pt.hub.empty : pt.hub.emptySearch}</p>
          ) : null}
          {unpinnedRows.map((r) => renderCard(r))}
        </div>
      </section>

      {busy && !activatingPath ? <p className="view-subtitle">{pt.hub.working}</p> : null}
    </div>
  );
}
