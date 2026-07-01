import type { ApplicationSettings } from "./types/settings";

export type ResolvedTheme = "light" | "dark";

export function resolveTheme(theme: ApplicationSettings["theme"]): ResolvedTheme {
  if (theme === "light" || theme === "dark") return theme;
  if (typeof window !== "undefined" && window.matchMedia) {
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }
  return "dark";
}

export function applyDataThemeToDocument(theme: ApplicationSettings["theme"]): () => void {
  const root = document.documentElement;
  if (theme === "system") {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const sync = () => {
      root.setAttribute("data-theme", resolveTheme("system"));
    };
    sync();
    mq.addEventListener("change", sync);
    return () => mq.removeEventListener("change", sync);
  }
  root.setAttribute("data-theme", resolveTheme(theme));
  return () => {};
}
