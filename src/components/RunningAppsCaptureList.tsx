import { useMemo } from "react";
import { t } from "../i18n";
import { emptyBrowserSettings, inferBrowserFamily } from "../browserDetect";
import type { ApplicationLaunchEntry } from "../types/profile";
import {
  candidateKey,
  displayNameOf,
  filterByKind,
  groupRunningApps,
  isKnownEditorBasename,
  windowSubtitle,
} from "../types/capture";
import type { RunningAppCandidate, RunningAppSection } from "../types/capture";

function sectionTitle(section: RunningAppSection): string {
  if (section.type === "workspace") return t.capture.workspace(section.workspace + 1);
  if (section.type === "noWorkspace") return t.capture.noWorkspace;
  return "";
}

export interface RunningAppsCaptureListProps {
  apps: RunningAppCandidate[];
  search: string;
  onSearchChange: (value: string) => void;
  selectedKeys: Record<string, boolean>;
  onToggleKey: (key: string, selected?: boolean) => void;
  onBulkSelect?: (patch: Record<string, boolean>) => void;
  busy?: boolean;
  error?: string | null;
  footer?: React.ReactNode;
}

export function RunningAppsCaptureList({
  apps,
  search,
  onSearchChange,
  selectedKeys,
  onToggleKey,
  onBulkSelect,
  busy = false,
  error,
  footer,
}: RunningAppsCaptureListProps) {
  const visibleApps = useMemo(() => filterByKind(apps), [apps]);

  const filteredApps = useMemo(() => {
    const q = search.trim().toLowerCase();
    if (!q) return visibleApps;
    return visibleApps.filter((c) => {
      const title = displayNameOf(c).toLowerCase();
      const sub = windowSubtitle(c)?.toLowerCase() ?? "";
      return title.includes(q) || sub.includes(q) || c.executable.toLowerCase().includes(q);
    });
  }, [visibleApps, search]);

  const sections = useMemo(() => groupRunningApps(filteredApps), [filteredApps]);

  const selectedCount = useMemo(
    () => apps.reduce((n, c) => n + (selectedKeys[candidateKey(c)] ? 1 : 0), 0),
    [apps, selectedKeys],
  );

  const filteredSelectedCount = useMemo(
    () => filteredApps.reduce((n, c) => n + (selectedKeys[candidateKey(c)] ? 1 : 0), 0),
    [filteredApps, selectedKeys],
  );

  const allFilteredSelected =
    filteredApps.length > 0 && filteredSelectedCount === filteredApps.length;

  const toggleSelectAllVisible = () => {
    if (!onBulkSelect || filteredApps.length === 0) return;
    const select = !allFilteredSelected;
    const patch: Record<string, boolean> = {};
    for (const c of filteredApps) {
      patch[candidateKey(c)] = select;
    }
    onBulkSelect(patch);
  };

  const toggleFromRow = (key: string) => onToggleKey(key, !selectedKeys[key]);

  return (
    <div className="capture-window-selector">
      <div className="capture-toolbar">
        <div className="search-input-wrapper">
          <svg className="search-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2" aria-hidden>
            <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <input
            type="search"
            className="search-input"
            placeholder={t.capture.searchPlaceholder}
            value={search}
            onChange={(e) => onSearchChange(e.target.value)}
            disabled={busy}
          />
        </div>
        <div className="capture-toolbar-actions">
          {filteredApps.length > 0 ? (
            <span className="capture-selection-count">{t.capture.selectedCount(selectedCount)}</span>
          ) : null}
          {onBulkSelect && filteredApps.length > 0 ? (
            <button
              type="button"
              className="btn btn-secondary btn-compact"
              disabled={busy}
              onClick={toggleSelectAllVisible}
            >
              {allFilteredSelected ? t.capture.deselectAll : t.capture.selectAll}
            </button>
          ) : null}
        </div>
      </div>

      {error ? (
        <p className="field-error" role="alert">
          {error}
        </p>
      ) : null}

      <div className="capture-list-scroll">
        {sections.length === 0 && !busy ? <p className="view-subtitle capture-empty">{t.capture.empty}</p> : null}
        {sections.map((section) => (
          <section key={sectionTitle(section) || "flat"} className="capture-workspace-group">
            {section.type !== "flat" ? (
              <h3 className="capture-workspace-title">
                <svg width="12" height="12" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2.5" aria-hidden>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                </svg>
                {sectionTitle(section)}
              </h3>
            ) : null}
            {section.items.map((c) => {
              const key = candidateKey(c);
              const selected = !!selectedKeys[key];
              const name = displayNameOf(c);
              const context = windowSubtitle(c);
              return (
                <div
                  key={key}
                  className={`window-item window-item--selectable${selected ? " window-item--selected" : ""}`}
                  onClick={() => toggleFromRow(key)}
                >
                  <div className="window-main">
                    <input
                      type="checkbox"
                      className="window-checkbox"
                      checked={selected}
                      onChange={(e) => onToggleKey(key, e.target.checked)}
                      onClick={(e) => e.stopPropagation()}
                      aria-label={name}
                    />
                    <span className="window-app-icon" aria-hidden>
                      {name.charAt(0).toUpperCase()}
                    </span>
                    <div className="window-details">
                      <span className="window-title">{name}</span>
                      <div className="window-meta">
                        {context ? (
                          <span className="window-meta-chip window-meta-chip--context" title={context}>
                            {context}
                          </span>
                        ) : null}
                      </div>
                    </div>
                  </div>
                </div>
              );
            })}
          </section>
        ))}
      </div>

      {footer}
    </div>
  );
}

export function launchEntriesFromCandidates(chosen: RunningAppCandidate[]): ApplicationLaunchEntry[] {
  return chosen.map((c) => {
    const target = (c.cwdHint ?? "").trim();
    const base =
      c.executable.split(/[/\\]/).pop()?.toLowerCase() ?? c.executable.toLowerCase();
    // Exact basename membership — mirrors Rust `editors::EDITOR_BASENAMES` (no substring heuristics).
    const vscodeLike = isKnownEditorBasename(base);
    let args: string[] = [];
    let cwd: string | null = null;
    if (target.length > 0) {
      if (vscodeLike) {
        args = ["--reuse-window", target];
        cwd = target;
      } else {
        args = [target];
        cwd = target;
      }
    }
    const browserFamily = inferBrowserFamily(c.executable);
    return {
      executable: c.executable,
      args: browserFamily ? [] : args,
      cwd: browserFamily ? null : cwd,
      skip_if_running: null,
      browser: browserFamily ? emptyBrowserSettings(browserFamily) : null,
    };
  });
}
