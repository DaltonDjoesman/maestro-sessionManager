import { invoke } from "@tauri-apps/api/core";

/** Parse clipboard text into one or more URL strings. */
export function parseClipboardUrls(text: string): string[] {
  const trimmed = text.trim();
  if (!trimmed) return [];
  if (/[\r\n]/.test(trimmed)) {
    return trimmed
      .split(/\r?\n/)
      .map((s) => s.trim())
      .filter((s) => s.length > 0);
  }
  if (trimmed.includes(",")) {
    return trimmed
      .split(",")
      .map((s) => s.trim())
      .filter((s) => s.length > 0);
  }
  return [trimmed];
}

/** Append parsed URLs as new rows, reusing trailing empty rows when present. */
export function mergeUrlsFromClipboard(existing: string[], pasted: string[]): string[] {
  if (pasted.length === 0) return existing;
  const next = [...existing];
  for (const line of pasted) {
    const emptyIdx = next.findIndex((u) => !u.trim());
    if (emptyIdx >= 0) {
      next[emptyIdx] = line;
    } else {
      next.push(line);
    }
  }
  return next;
}

export async function readClipboardText(): Promise<string | null> {
  try {
    const text = await invoke<string>("read_clipboard_text");
    return text;
  } catch {
    try {
      return await navigator.clipboard.readText();
    } catch {
      return null;
    }
  }
}
