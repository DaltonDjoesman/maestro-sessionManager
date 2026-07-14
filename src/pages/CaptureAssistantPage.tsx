import { useCallback, useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { pt } from "../i18n/pt";
import { RunningAppsCaptureList, launchEntriesFromCandidates } from "../components/RunningAppsCaptureList";
import { RefreshIconButton } from "../components/RefreshIconButton";
import { candidateKey, sortByDisplayName } from "../types/capture";
import type { RunningAppCandidate } from "../types/capture";
import type { SessionProfile } from "../types/profile";

interface CaptureAssistantPageProps {
  onEdit: (filePath: string) => void;
}

export function CaptureAssistantPage({ onEdit }: CaptureAssistantPageProps) {
  const [runningApps, setRunningApps] = useState<RunningAppCandidate[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [selected, setSelected] = useState<Record<string, boolean>>({});

  const refresh = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const list = await invoke<RunningAppCandidate[]>("list_assistant_running_apps");
      setRunningApps(list);
    } catch (e) {
      setRunningApps([]);
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const selectedCount = useMemo(
    () => runningApps.reduce((n, c) => n + (selected[candidateKey(c)] ? 1 : 0), 0),
    [runningApps, selected],
  );

  const toggleKey = (key: string, next?: boolean) => {
    setSelected((prev) => ({ ...prev, [key]: next ?? !prev[key] }));
  };

  const mergeSelection = (patch: Record<string, boolean>) => {
    setSelected((prev) => ({ ...prev, ...patch }));
  };

  const createFromCapture = async () => {
    const chosen = sortByDisplayName(runningApps).filter((c) => selected[candidateKey(c)]);
    if (chosen.length === 0) return;
    setBusy(true);
    setError(null);
    try {
      const created = await invoke<{ filePath: string }>("create_session_profile", {});
      const profile = await invoke<SessionProfile>("load_session_profile", { path: created.filePath });
      const apps = launchEntriesFromCandidates(chosen);
      const updated: SessionProfile = {
        ...profile,
        name: pt.capture.defaultProfileName,
        applications: [...profile.applications, ...apps],
      };
      await invoke("save_session_profile", { path: created.filePath, profile: updated });
      onEdit(created.filePath);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="page capture-page">
      <div className="capture-page-header">
        <div className="view-title-group">
          <h1 className="view-title">{pt.capture.title}</h1>
          <p className="view-subtitle">{pt.capture.subtitle}</p>
        </div>
        <RefreshIconButton onClick={() => void refresh()} disabled={busy} busy={busy} />
      </div>

      <RunningAppsCaptureList
        apps={runningApps}
        search={search}
        onSearchChange={setSearch}
        selectedKeys={selected}
        onToggleKey={toggleKey}
        onBulkSelect={mergeSelection}
        busy={busy}
        error={error}
        footer={
          <div className="capture-list-footer">
            <button
              type="button"
              className="btn btn-primary btn-compact"
              disabled={busy || selectedCount === 0}
              onClick={() => void createFromCapture()}
            >
              {pt.capture.createFromSelection}
            </button>
          </div>
        }
      />
    </div>
  );
}
