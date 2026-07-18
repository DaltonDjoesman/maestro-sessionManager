import type { RunningAppCandidate } from "./types/capture";

/** Soft TTL: show last list instantly when revisiting Captura, then refresh. */
const CAPTURE_CACHE_TTL_MS = 15_000;

let cachedApps: RunningAppCandidate[] | null = null;
let cachedAt = 0;

export function peekCachedRunningApps(): RunningAppCandidate[] | null {
  if (!cachedApps) return null;
  if (Date.now() - cachedAt > CAPTURE_CACHE_TTL_MS) return null;
  return cachedApps;
}

export function rememberRunningApps(apps: RunningAppCandidate[]): void {
  cachedApps = apps;
  cachedAt = Date.now();
}
