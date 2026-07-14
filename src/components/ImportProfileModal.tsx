import { useState } from "react";
import { pt } from "../i18n/pt";

interface ImportProfileModalProps {
  open: boolean;
  busy: boolean;
  onClose: () => void;
  onImport: (json: string, displayName: string) => Promise<void>;
}

export function ImportProfileModal({ open, busy, onClose, onImport }: ImportProfileModalProps) {
  const [jsonStr, setJsonStr] = useState("");
  const [displayName, setDisplayName] = useState("");
  const [error, setError] = useState<string | null>(null);

  if (!open) return null;

  const handleSubmit = async () => {
    setError(null);
    const trimmedName = displayName.trim();
    if (!trimmedName) {
      setError(pt.hub.emptyName);
      return;
    }
    if (!jsonStr.trim()) {
      setError(pt.import.invalidJson);
      return;
    }
    try {
      JSON.parse(jsonStr);
    } catch {
      setError(pt.import.invalidJson);
      return;
    }
    try {
      await onImport(jsonStr, trimmedName);
      setJsonStr("");
      setDisplayName("");
      onClose();
    } catch (e) {
      setError(String(e));
    }
  };

  const onFilePick = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    e.target.value = "";
    if (!file) return;
    try {
      const text = await file.text();
      setJsonStr(text);
      if (!displayName.trim()) {
        setDisplayName(file.name.replace(/\.json$/i, "") || "Importado");
      }
    } catch (err) {
      setError(String(err));
    }
  };

  return (
    <div className="modal-overlay" role="presentation" onClick={onClose}>
      <div className="modal-card" role="dialog" aria-labelledby="import-modal-title" onClick={(e) => e.stopPropagation()}>
        <header className="modal-header">
          <h2 className="modal-title" id="import-modal-title">
            {pt.import.title}
          </h2>
          <button type="button" className="btn-icon" aria-label={pt.activation.close} onClick={onClose}>
            ✕
          </button>
        </header>
        <div className="modal-body">
          <p className="view-subtitle">{pt.import.hint}</p>
          {error ? (
            <p className="field-error" role="alert">
              {error}
            </p>
          ) : null}
          <label className="form-group">
            <span className="form-label">{pt.import.displayName}</span>
            <input
              type="text"
              className="form-input"
              value={displayName}
              onChange={(e) => setDisplayName(e.target.value)}
              disabled={busy}
            />
          </label>
          <label className="form-group">
            <span className="form-label">{pt.import.jsonLabel}</span>
            <textarea
              className="form-input"
              rows={8}
              value={jsonStr}
              onChange={(e) => setJsonStr(e.target.value)}
              disabled={busy}
              spellCheck={false}
            />
          </label>
          <label className="btn btn-secondary btn-compact">
            {pt.import.pickFile}
            <input type="file" accept=".json,application/json" className="visually-hidden" onChange={(e) => void onFilePick(e)} />
          </label>
        </div>
        <footer className="modal-footer">
          <button type="button" className="btn btn-secondary" disabled={busy} onClick={onClose}>
            {pt.import.cancel}
          </button>
          <button type="button" className="btn btn-primary" disabled={busy} onClick={() => void handleSubmit()}>
            {busy ? pt.hub.working : pt.import.confirm}
          </button>
        </footer>
      </div>
    </div>
  );
}
