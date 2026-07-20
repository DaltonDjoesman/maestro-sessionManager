import { useCallback, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { downloadJsonFile, safeDownloadBase } from "../downloadJson";
import { t } from "../i18n";
import type { DuplicateProfileResult, SessionProfile } from "../types/profile";

export type ImportDraft = {
  json: string;
  fileName: string;
  displayName: string;
  error: string | null;
};

export type ExportDraft = {
  path: string;
  displayName: string;
  profile: SessionProfile;
  fileName: string;
};

type UseHubImportExportOptions = {
  setBusy: (busy: boolean) => void;
  setError: (error: string | null) => void;
  refresh: () => Promise<void>;
  onEdit: (filePath: string) => void;
  closeMenu?: () => void;
};

export function useHubImportExport({
  setBusy,
  setError,
  refresh,
  onEdit,
  closeMenu,
}: UseHubImportExportOptions) {
  const [importDraft, setImportDraft] = useState<ImportDraft | null>(null);
  const [exportDraft, setExportDraft] = useState<ExportDraft | null>(null);
  const importFileRef = useRef<HTMLInputElement>(null);

  const openExportModal = useCallback(
    async (path: string, displayName: string) => {
      closeMenu?.();
      setBusy(true);
      setError(null);
      try {
        const profile = await invoke<SessionProfile>("load_session_profile", { path });
        setExportDraft({
          path,
          displayName,
          profile,
          fileName: `${safeDownloadBase(displayName)}.json`,
        });
      } catch (e) {
        setError(String(e));
      } finally {
        setBusy(false);
      }
    },
    [closeMenu, setBusy, setError],
  );

  const confirmExport = useCallback(() => {
    if (!exportDraft) return;
    downloadJsonFile(JSON.stringify(exportDraft.profile, null, 2), exportDraft.fileName);
    setExportDraft(null);
  }, [exportDraft]);

  const handleImportConfirm = useCallback(async () => {
    if (!importDraft) return;
    const trimmed = importDraft.displayName.trim();
    if (!trimmed) {
      setImportDraft({ ...importDraft, error: t.hub.emptyName });
      return;
    }
    setBusy(true);
    setImportDraft({ ...importDraft, error: null });
    try {
      const dup = await invoke<DuplicateProfileResult>("import_session_profile_json", {
        json: importDraft.json,
        displayName: trimmed,
      });
      setImportDraft(null);
      await refresh();
      onEdit(dup.filePath);
    } catch (e) {
      setImportDraft({ ...importDraft, error: String(e) });
    } finally {
      setBusy(false);
    }
  }, [importDraft, onEdit, refresh, setBusy]);

  const onImportFileSelected = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      e.target.value = "";
      if (!file) return;

      setError(null);
      let json: string;
      try {
        json = await file.text();
        JSON.parse(json);
      } catch {
        setError(t.import.invalidJson);
        return;
      }

      const defaultName = file.name.replace(/\.json$/i, "") || "Importado";
      setImportDraft({
        json,
        fileName: file.name,
        displayName: defaultName,
        error: null,
      });
    },
    [setError],
  );

  return {
    importDraft,
    setImportDraft,
    exportDraft,
    setExportDraft,
    importFileRef,
    openExportModal,
    confirmExport,
    handleImportConfirm,
    onImportFileSelected,
  };
}
