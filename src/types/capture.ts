/** Row from `list_assistant_running_apps` (serde camelCase). */
export type CandidateKind = "app" | "process";

export type ClassificationConfidence = "high" | "medium" | "low";

export interface RunningAppCandidate {
  pid: number;
  executable: string;
  label: string;
  cmdPreview: string;
  cwdHint?: string | null;
  kind?: CandidateKind;
  displayName: string;
  iconName?: string | null;
  desktopWorkspace?: number | null;
  windowTitle?: string | null;
  classificationConfidence?: ClassificationConfidence | null;
}

export type RunningAppSection =
  | { type: "workspace"; workspace: number; items: RunningAppCandidate[] }
  | { type: "noWorkspace"; items: RunningAppCandidate[] }
  | { type: "flat"; items: RunningAppCandidate[] };

export function displayNameOf(c: RunningAppCandidate): string {
  const name = c.displayName?.trim();
  if (name) return name;
  return c.label;
}

const EDITOR_BASES = new Set(["cursor", "code", "code-oss", "codium", "obsidian"]);

/** Stable row key aligned with backend dedupe (multi-window / workspace). */
export function candidateKey(c: RunningAppCandidate): string {
  const base = c.executable.split("/").pop()?.toLowerCase() ?? "";
  const ws = c.desktopWorkspace != null ? String(c.desktopWorkspace) : "none";
  const title = c.windowTitle?.trim() ?? "";
  if (EDITOR_BASES.has(base)) {
    const folder = c.cwdHint?.trim() ?? "";
    return `${c.executable}\x1f${folder}\x1f${ws}\x1f${title}`;
  }
  return `${c.executable}\x1f${ws}\x1f${title}`;
}

/** Window title for card subtitle when it adds context beyond displayName. */
export function windowSubtitle(c: RunningAppCandidate): string | null {
  const wt = c.windowTitle?.trim();
  if (!wt) return null;
  const dn = displayNameOf(c);
  if (wt.localeCompare(dn, undefined, { sensitivity: "base" }) === 0) return null;
  return wt;
}

export function sortByDisplayName(list: RunningAppCandidate[]): RunningAppCandidate[] {
  return [...list].sort((a, b) => {
    const cmp = displayNameOf(a).localeCompare(displayNameOf(b), undefined, { sensitivity: "base" });
    if (cmp !== 0) return cmp;
    return a.pid - b.pid;
  });
}

export function filterByKind(list: RunningAppCandidate[], showProcesses: boolean): RunningAppCandidate[] {
  if (showProcesses) return list;
  return list.filter((c) => (c.kind ?? "app") === "app");
}

export function groupRunningApps(list: RunningAppCandidate[]): RunningAppSection[] {
  const hasWorkspace = list.some((c) => c.desktopWorkspace != null && c.desktopWorkspace !== undefined);
  if (!hasWorkspace) {
    return [{ type: "flat", items: sortByDisplayName(list) }];
  }
  const byWs = new Map<number, RunningAppCandidate[]>();
  const noWs: RunningAppCandidate[] = [];
  for (const c of list) {
    if (c.desktopWorkspace != null && c.desktopWorkspace !== undefined) {
      const arr = byWs.get(c.desktopWorkspace) ?? [];
      arr.push(c);
      byWs.set(c.desktopWorkspace, arr);
    } else {
      noWs.push(c);
    }
  }
  const sections: RunningAppSection[] = [];
  for (const ws of [...byWs.keys()].sort((a, b) => a - b)) {
    sections.push({ type: "workspace", workspace: ws, items: sortByDisplayName(byWs.get(ws)!) });
  }
  if (noWs.length > 0) {
    sections.push({ type: "noWorkspace", items: sortByDisplayName(noWs) });
  }
  return sections;
}
