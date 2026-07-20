import { useState } from "react";
import { mergeUrlsFromClipboard, parseClipboardUrls, readClipboardText } from "../clipboard";
import { t } from "../i18n";
import type { ApplicationBrowserSettings } from "../types/profile";
import type { BrowserFamily } from "../types/settings";

export interface ApplicationBrowserFieldsProps {
  browser: ApplicationBrowserSettings;
  onPatch: (patch: Partial<ApplicationBrowserSettings>) => void;
}

export function ApplicationBrowserFields({ browser, onPatch }: ApplicationBrowserFieldsProps) {
  const [pasteError, setPasteError] = useState<string | null>(null);

  const addBrowserUrlRow = () => {
    setPasteError(null);
    onPatch({ urls: [...(browser.urls ?? []), ""] });
  };

  const pasteBrowserUrlsFromClipboard = async () => {
    setPasteError(null);
    const text = await readClipboardText();
    if (text == null) {
      setPasteError(t.editor.pasteClipboardFailed);
      return;
    }
    const lines = parseClipboardUrls(text);
    if (lines.length === 0) {
      setPasteError(t.editor.pasteClipboardEmpty);
      return;
    }
    onPatch({ urls: mergeUrlsFromClipboard(browser.urls ?? [], lines) });
  };

  return (
    <div className="browser-fields field-full">
      <p className="hint browser-fields-hint">{t.editor.browserAppHint}</p>
      <div className="app-card-grid browser-fields-grid">
        <label className="field">
          <span>{t.editor.browserFamily}</span>
          <select
            className="form-select"
            value={browser.family}
            onChange={(e) => onPatch({ family: e.target.value as BrowserFamily })}
          >
            <option value="chromium_like">{t.editor.browserFamilyChromium}</option>
            <option value="firefox">{t.editor.browserFamilyFirefox}</option>
          </select>
        </label>
        {browser.family === "chromium_like" ? (
          <label className="field">
            <span>{t.editor.userDataDir}</span>
            <input
              type="text"
              className="browser-subfield-input"
              value={browser.user_data_dir ?? ""}
              onChange={(e) =>
                onPatch({
                  user_data_dir: e.target.value.trim() ? e.target.value : null,
                })
              }
              placeholder={t.editor.userDataDirPlaceholder}
            />
          </label>
        ) : (
          <label className="field">
            <span>{t.editor.firefoxProfile}</span>
            <input
              type="text"
              className="browser-subfield-input"
              value={browser.firefox_profile ?? ""}
              onChange={(e) =>
                onPatch({
                  firefox_profile: e.target.value.trim() ? e.target.value : null,
                })
              }
            />
          </label>
        )}
        <div className="field field-full browser-urls-field">
          <div className="browser-urls-label-row">
            <span>{t.editor.urlsToOpen}</span>
            <div className="browser-urls-toolbar">
              <button type="button" className="btn btn-secondary btn-compact" onClick={addBrowserUrlRow}>
                {t.editor.addUrl}
              </button>
              <button
                type="button"
                className="btn btn-secondary btn-compact"
                onClick={() => void pasteBrowserUrlsFromClipboard()}
              >
                {t.editor.pasteFromClipboard}
              </button>
            </div>
          </div>
          <p className="hint browser-urls-hint">{t.editor.urlsHint}</p>
          {pasteError ? (
            <p className="field-error" role="alert">
              {pasteError}
            </p>
          ) : null}
          {browser.urls.length === 0 ? (
            <p className="hint browser-urls-empty">{t.editor.urlsEmpty}</p>
          ) : null}
          <ul className="browser-url-list" aria-label={t.editor.urlsToOpen}>
            {(browser.urls ?? []).map((url, i) => (
              <li key={i} className="browser-url-row">
                <div className="browser-url-row-main">
                  <span className="browser-url-index">{i + 1}</span>
                  <input
                    type="text"
                    className="browser-url-input"
                    value={url}
                    title={url || undefined}
                    placeholder="https://example.com or file:///…"
                    spellCheck={false}
                    autoComplete="off"
                    aria-label={`URL ${i + 1}`}
                    onChange={(e) => {
                      const next = [...(browser.urls ?? [])];
                      next[i] = e.target.value;
                      onPatch({ urls: next });
                    }}
                  />
                </div>
                <button
                  type="button"
                  className="btn btn-secondary btn-compact danger browser-url-remove"
                  aria-label={`${t.editor.remove} URL ${i + 1}`}
                  onClick={() => onPatch({ urls: (browser.urls ?? []).filter((_, j) => j !== i) })}
                >
                  {t.editor.remove}
                </button>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}
