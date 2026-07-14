/** Stable short key for localStorage namespacing per profiles root directory. */
export function profilesRootStorageKey(profilesRoot: string): string {
  let h = 0;
  const s = profilesRoot.trim();
  for (let i = 0; i < s.length; i++) {
    h = (Math.imul(31, h) + s.charCodeAt(i)) | 0;
  }
  return `m_${(h >>> 0).toString(16)}`;
}

const pinsKey = (root: string) => `maestro.catalog.pins.${profilesRootStorageKey(root)}`;
const lastKey = (root: string) => `maestro.catalog.lastSession.${profilesRootStorageKey(root)}`;
const activeKey = (root: string) => `maestro.catalog.activeSession.${profilesRootStorageKey(root)}`;

export function loadPinnedPaths(profilesRoot: string): string[] {
  try {
    const raw = localStorage.getItem(pinsKey(profilesRoot));
    if (!raw) return [];
    const parsed = JSON.parse(raw) as unknown;
    if (!Array.isArray(parsed)) return [];
    return parsed.filter((x): x is string => typeof x === "string");
  } catch {
    return [];
  }
}

export function savePinnedPaths(profilesRoot: string, paths: string[]): void {
  localStorage.setItem(pinsKey(profilesRoot), JSON.stringify(paths));
}

export function loadLastSessionPath(profilesRoot: string): string | null {
  try {
    const v = localStorage.getItem(lastKey(profilesRoot));
    return v && v.trim() ? v.trim() : null;
  } catch {
    return null;
  }
}

export function saveLastSessionPath(profilesRoot: string, path: string | null): void {
  if (!path || !path.trim()) {
    localStorage.removeItem(lastKey(profilesRoot));
    return;
  }
  localStorage.setItem(lastKey(profilesRoot), path.trim());
}

export function loadActiveSessionLabel(profilesRoot: string): string | null {
  try {
    const v = localStorage.getItem(activeKey(profilesRoot));
    return v && v.trim() ? v.trim() : null;
  } catch {
    return null;
  }
}

export function saveActiveSessionLabel(profilesRoot: string, label: string | null): void {
  if (!label || !label.trim()) {
    localStorage.removeItem(activeKey(profilesRoot));
    return;
  }
  localStorage.setItem(activeKey(profilesRoot), label.trim());
}
