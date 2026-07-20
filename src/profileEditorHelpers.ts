import { t } from "./i18n";
import type { ApplicationLaunchEntry, SessionProfile } from "./types/profile";

export function cloneProfile(p: SessionProfile): SessionProfile {
  return JSON.parse(JSON.stringify(p)) as SessionProfile;
}

export function appDisplayLabel(app: ApplicationLaunchEntry, index: number): string {
  const exe = app.executable.trim();
  if (!exe) return t.editor.appN(index + 1);
  const base = exe.split(/[/\\]/).pop() ?? exe;
  const cleaned = base.replace(/\.(AppImage|app)$/i, "");
  if (!cleaned) return t.editor.appN(index + 1);
  return cleaned.charAt(0).toUpperCase() + cleaned.slice(1);
}

export function appSummaryHint(app: ApplicationLaunchEntry): string {
  const exe = app.executable.trim();
  return exe || t.editor.noExecutable;
}
