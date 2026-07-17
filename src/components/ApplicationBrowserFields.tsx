import { useState } from "react";
import { mergeUrlsFromClipboard, parseClipboardUrls, readClipboardText } from "../clipboard";
import { pt } from "../i18n/pt";
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
      setPasteError(pt.editor.pasteClipboardFailed);
      return;
    }
    const lines = parseClipboardUrls(text);
    if (lines.length === 0) {
      setPasteError(pt.editor.pasteClipboardEmpty);
      return;
    }
    onPatch({ urls: mergeUrlsFromClipboard(browser.urls ?? [], lines) });
  };

  return (
    <div className="browser-fields field-full">
      <p className="hint browser-fields-hint">{pt.editor.browserAppHint}</p>
      <div className="app-card-grid browser-fields-grid">
        <label className="field">
          <span>{pt.editor.browserFamily}</span>
          <select
            className="form-select"
            value={browser.family}
            onChange={(e) => onPatch({ family: e.target.value as BrowserFamily })}
          >
            <option value="chromium_like">{pt.editor.browserFamilyChromium}</option>
            <option value="firefox">{pt.editor.browserFamilyFirefox}</option>
          </select>
        </label>
        {browser.family === "chromium_like" ? (
          <label className="field">
            <span>{pt.editor.userDataDir}</span>
            <input
              type="text"
              className="browser-subfield-input"
              value={browser.user_data_dir ?? ""}
              onChange={(e) =>
                onPatch({
                  user_data_dir: e.target.value.trim() ? e.target.value : null,
                })
              }
              placeholder={pt.editor.userDataDirPlaceholder}
            />
          </label>
        ) : (
          <label className="field">
            <span>{pt.editor.firefoxProfile}</span>
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
            <span>{pt.editor.urlsToOpen}</span>
            <div className="browser-urls-toolbar">
              <button type="button" className="btn btn-secondary btn-compact" onClick={addBrowserUrlRow}>
                {pt.editor.addUrl}
              </button>
              <button
                type="button"
                className="btn btn-secondary btn-compact"
                onClick={() => void pasteBrowserUrlsFromClipboard()}
              >
                {pt.editor.pasteFromClipboard}
              </button>
            </div>
          </div>
          <p className="hint browser-urls-hint">{pt.editor.urlsHint}</p>
          {pasteError ? (
            <p className="field-error" role="alert">
              {pasteError}
            </p>
          ) : null}
          {browser.urls.length === 0 ? (
            <p className="hint browser-urls-empty">{pt.editor.urlsEmpty}</p>
          ) : null}
          <ul className="browser-url-list" aria-label={pt.editor.urlsToOpen}>
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
                  aria-label={`${pt.editor.remove} URL ${i + 1}`}
                  onClick={() => onPatch({ urls: (browser.urls ?? []).filter((_, j) => j !== i) })}
                >
                  {pt.editor.remove}
                </button>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}
