/** Sanitize a display name into a safe download basename. */
export function safeDownloadBase(name: string): string {
  return name.replace(/[^\w\-]+/g, "_").slice(0, 80) || "session";
}

/** Trigger a browser download of a JSON string. */
export function downloadJsonFile(json: string, fileName: string): void {
  const blob = new Blob([json], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  const base = safeDownloadBase(fileName.replace(/\.json$/i, ""));
  a.href = url;
  a.download = `${base}.json`;
  a.click();
  URL.revokeObjectURL(url);
}
