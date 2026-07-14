import type { ProfileCatalogEntry } from "./types/profile";
import type { SessionProfile } from "./types/profile";
import { pt } from "./i18n/pt";

/** One-line summary for hub cards from catalog metadata. */
export function summaryFromCatalogEntry(entry: ProfileCatalogEntry): string {
  const parts: string[] = [];
  const apps = entry.applicationsCount ?? 0;
  if (apps > 0) {
    parts.push(pt.hub.appsCount(apps));
  }
  if (entry.hasBrowser) {
    parts.push(pt.hub.hasBrowser);
  }
  if (parts.length === 0 && entry.valid) {
    parts.push(pt.hub.emptyContent);
  }
  return parts.join(" · ");
}

/** Richer summary when full profile is loaded (e.g. URL count). */
export function summaryFromProfile(profile: SessionProfile): string {
  const parts: string[] = [];
  const apps = profile.applications.length;
  if (apps > 0) {
    parts.push(pt.hub.appsCount(apps));
  }
  const hasBrowser = profile.applications.some((a) => a.browser != null);
  if (hasBrowser) {
    const urls = profile.applications.reduce(
      (n, a) => n + (a.browser?.urls.filter((u) => u.trim()).length ?? 0),
      0,
    );
    if (urls > 0) {
      parts.push(`${pt.hub.hasBrowser} · ${urls} URL${urls === 1 ? "" : "s"}`);
    } else {
      parts.push(pt.hub.hasBrowser);
    }
  }
  if (parts.length === 0) {
    parts.push(pt.hub.emptyContent);
  }
  return parts.join(" · ");
}
