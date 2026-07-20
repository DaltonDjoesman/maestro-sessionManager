import { pt } from "./pt";
import { en } from "./en";

/** Supported UI locale ids. Default product locale is European Portuguese (pt-PT). */
export type LocaleId = "pt" | "en";

export type Messages = typeof pt;

export const defaultLocale: LocaleId = "pt";

const catalogs: Record<LocaleId, Messages> = {
  pt,
  en,
};

let activeLocale: LocaleId = defaultLocale;

export function getLocale(): LocaleId {
  return activeLocale;
}

/** Switch active locale. Call sites that already use `t` keep working. */
export function setLocale(locale: LocaleId): void {
  activeLocale = locale;
}

/**
 * Shared string accessor for the active locale (default: pt-PT).
 * Prefer `import { t } from "../i18n"` over importing a locale file directly.
 */
export const t: Messages = new Proxy({} as Messages, {
  get(_target, prop, _receiver) {
    const catalog = catalogs[activeLocale];
    const value = catalog[prop as keyof Messages];
    return value;
  },
});

export { pt, en };
