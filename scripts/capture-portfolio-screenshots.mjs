/**
 * One-off: capture Maestro UI screenshots via Vite + mocked Tauri IPC.
 * Usage: node scripts/capture-portfolio-screenshots.mjs
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import puppeteer from "puppeteer-core";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const outDir = path.resolve(__dirname, "../docs/screenshots");
const baseUrl = process.env.MAESTRO_UI_URL || "http://localhost:1420/";
const chrome =
  process.env.CHROME_PATH ||
  ["/usr/bin/google-chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser"].find((p) =>
    fs.existsSync(p),
  );

if (!chrome) {
  console.error("No Chrome/Chromium found");
  process.exit(1);
}

fs.mkdirSync(outDir, { recursive: true });

const mockSource = `(() => {
  const profilesRoot = '/home/demo/.local/share/maestro/profiles';
  const profiles = {
    'dev-maestro.json': {
      schema_version: 1,
      session_id: '11111111-1111-4111-8111-111111111111',
      name: 'Desenvolvimento Maestro',
      applications: [
        { executable: '/usr/bin/code', args: [], cwd: '/home/demo/maestro', skip_if_running: true, browser: null },
        { executable: '/usr/bin/alacritty', args: [], cwd: null, skip_if_running: null, browser: null },
        {
          executable: '/usr/bin/vivaldi',
          args: [],
          cwd: null,
          skip_if_running: null,
          browser: {
            family: 'chromium_like',
            user_data_dir: null,
            firefox_profile: null,
            firefox_no_remote: null,
            urls: ['https://github.com', 'https://docs.rs'],
          },
        },
      ],
      browser: null,
      cleanup: null,
    },
    'estudo-rust.json': {
      schema_version: 1,
      session_id: '22222222-2222-4222-8222-222222222222',
      name: 'Estudo Rust',
      applications: [
        { executable: '/usr/bin/code', args: [], cwd: '/home/demo/rust', skip_if_running: true, browser: null },
        {
          executable: '/usr/bin/firefox',
          args: [],
          cwd: null,
          skip_if_running: null,
          browser: {
            family: 'firefox',
            user_data_dir: null,
            firefox_profile: 'default',
            firefox_no_remote: true,
            urls: ['https://doc.rust-lang.org'],
          },
        },
      ],
      browser: null,
      cleanup: null,
    },
    'lazer.json': {
      schema_version: 1,
      session_id: '33333333-3333-4333-8333-333333333333',
      name: 'Lazer & Playlist',
      applications: [
        { executable: '/usr/bin/spotify', args: [], cwd: null, skip_if_running: true, browser: null },
        { executable: '/usr/bin/discord', args: [], cwd: null, skip_if_running: true, browser: null },
      ],
      browser: null,
      cleanup: null,
    },
  };

  const catalog = Object.entries(profiles).map(([fileName, p]) => ({
    filePath: profilesRoot + '/' + fileName,
    fileName,
    valid: true,
    sessionId: p.session_id,
    name: p.name,
    error: null,
    applicationsCount: p.applications.length,
    hasBrowser: p.applications.some((a) => !!a.browser),
  }));

  const runningApps = [
    {
      pid: 4201,
      executable: '/usr/bin/code',
      label: 'code',
      cmdPreview: 'code .',
      cwdHint: '/home/demo/maestro',
      kind: 'app',
      displayName: 'VS Code',
      iconName: 'code',
      desktopWorkspace: 0,
      windowTitle: 'maestro — main.rs',
      classificationConfidence: 'high',
    },
    {
      pid: 4202,
      executable: '/usr/bin/alacritty',
      label: 'alacritty',
      cmdPreview: 'alacritty',
      cwdHint: '/home/demo/maestro',
      kind: 'app',
      displayName: 'Alacritty',
      iconName: 'Alacritty',
      desktopWorkspace: 0,
      windowTitle: 'cargo test',
      classificationConfidence: 'high',
    },
    {
      pid: 4203,
      executable: '/usr/bin/vivaldi',
      label: 'vivaldi',
      cmdPreview: 'vivaldi',
      cwdHint: null,
      kind: 'app',
      displayName: 'Vivaldi',
      iconName: 'vivaldi',
      desktopWorkspace: 1,
      windowTitle: 'Docs',
      classificationConfidence: 'medium',
    },
    {
      pid: 4204,
      executable: '/usr/bin/spotify',
      label: 'spotify',
      cmdPreview: 'spotify',
      cwdHint: null,
      kind: 'app',
      displayName: 'Spotify',
      iconName: 'spotify',
      desktopWorkspace: 1,
      windowTitle: 'Spotify Premium',
      classificationConfidence: 'high',
    },
  ];

  let settings = {
    schema_version: 1,
    profiles_root: profilesRoot,
    theme: 'dark',
  };

  window.__TAURI_INTERNALS__ = window.__TAURI_INTERNALS__ || {};
  window.__TAURI_EVENT_PLUGIN_INTERNALS__ = window.__TAURI_EVENT_PLUGIN_INTERNALS__ || {};
  const callbacks = new Map();
  window.__TAURI_INTERNALS__.callbacks = callbacks;
  window.__TAURI_INTERNALS__.transformCallback = (cb, once = false) => {
    const id = crypto.getRandomValues(new Uint32Array(1))[0];
    callbacks.set(id, (data) => {
      if (once) callbacks.delete(id);
      return cb && cb(data);
    });
    return id;
  };
  window.__TAURI_INTERNALS__.unregisterCallback = (id) => callbacks.delete(id);
  window.__TAURI_INTERNALS__.runCallback = (id, data) => {
    const cb = callbacks.get(id);
    if (cb) cb(data);
  };
  window.__TAURI_INTERNALS__.metadata = {
    currentWindow: { label: 'main' },
    currentWebview: { windowLabel: 'main', label: 'main' },
  };

  window.__TAURI_INTERNALS__.invoke = async (cmd, args = {}) => {
    switch (cmd) {
      case 'get_settings':
        return { ...settings };
      case 'save_settings':
        settings = { ...settings, ...(args.settings || {}) };
        return { ...settings };
      case 'validate_profiles_root':
        return true;
      case 'list_session_profiles':
        return catalog.map((c) => ({ ...c }));
      case 'list_assistant_running_apps':
        return runningApps.map((a) => ({ ...a }));
      case 'load_session_profile': {
        const key = (args.path || '').split('/').pop();
        const p = profiles[key];
        if (!p) throw new Error('profile not found');
        return structuredClone(p);
      }
      case 'activate_session_profile': {
        const key = (args.path || '').split('/').pop();
        const p = profiles[key];
        const steps = (p?.applications || []).map((app) => ({
          kind: app.browser ? 'browser' : 'application',
          label: app.executable.split('/').pop(),
          status: 'success',
          detail: null,
          pid: Math.floor(Math.random() * 9000) + 1000,
        }));
        return {
          steps,
          activationLogPath: '/tmp/maestro-activation.log',
        };
      }
      case 'preview_session_activation': {
        const key = (args.path || '').split('/').pop();
        const p = profiles[key];
        return (p?.applications || []).map((app) => ({
          stepType: app.browser ? 'browser' : 'application',
          label: app.executable.split('/').pop(),
          argv: [app.executable, ...(app.args || [])],
          cwd: app.cwd,
          wouldSkip: !!app.skip_if_running,
          skipDetail: app.skip_if_running ? 'already running' : null,
        }));
      }
      case 'detect_system_default_browser':
        return { executable: '/usr/bin/vivaldi', family: 'chromium_like', desktopEntry: 'vivaldi.desktop' };
      case 'create_session_profile':
        return { filePath: profilesRoot + '/nova-sessao.json' };
      case 'save_session_profile':
        return null;
      case 'read_clipboard_text':
        return '';
      default:
        console.warn('unmocked invoke', cmd, args);
        return null;
    }
  };
})();`;

async function waitForHubCards(page) {
  await page.waitForFunction(
    () => !document.body.innerText.includes("Cannot read properties of undefined"),
    { timeout: 10000 },
  );
  await page.waitForFunction(
    () => document.body.innerText.includes("Desenvolvimento Maestro"),
    { timeout: 10000 },
  );
}

const browser = await puppeteer.launch({
  executablePath: chrome,
  headless: "new",
  args: ["--no-sandbox", "--disable-gpu", "--window-size=1440,900"],
  defaultViewport: { width: 1440, height: 900 },
});

const page = await browser.newPage();
await page.evaluateOnNewDocument(mockSource);
await page.goto(baseUrl, { waitUntil: "networkidle0" });
await waitForHubCards(page);
await page.waitForTimeout?.(400);
await new Promise((r) => setTimeout(r, 400));

const hubPath = path.join(outDir, "hub.png");
await page.screenshot({ path: hubPath, type: "png" });
console.log("wrote", hubPath);

async function clickByText(label) {
  const handle = await page.evaluateHandle((text) => {
    const buttons = [...document.querySelectorAll("button")];
    return buttons.find((b) => (b.textContent || "").trim().includes(text)) || null;
  }, label);
  const el = handle.asElement();
  if (!el) throw new Error(`button not found: ${label}`);
  await el.click();
}

await clickByText("Captura");
await page.waitForFunction(() => document.body.innerText.includes("Assistente de captura"), {
  timeout: 8000,
});
await new Promise((r) => setTimeout(r, 500));
const capturePath = path.join(outDir, "capture.png");
await page.screenshot({ path: capturePath, type: "png" });
console.log("wrote", capturePath);

await clickByText("Sessões");
await waitForHubCards(page);
await clickByText("Ativar");
await page.waitForFunction(
  () =>
    document.body.innerText.includes("Terminal Maestro") ||
    document.body.innerText.includes("Activação concluída"),
  { timeout: 8000 },
);
await new Promise((r) => setTimeout(r, 400));
const overlayPath = path.join(outDir, "activation-overlay.png");
await page.screenshot({ path: overlayPath, type: "png" });
console.log("wrote", overlayPath);

await page.keyboard.press("Escape");
await new Promise((r) => setTimeout(r, 300));
await clickByText("Definições");
await page.waitForFunction(() => document.body.innerText.includes("Preferências globais"), {
  timeout: 8000,
});
await new Promise((r) => setTimeout(r, 400));
const settingsPath = path.join(outDir, "settings.png");
await page.screenshot({ path: settingsPath, type: "png" });
console.log("wrote", settingsPath);

await browser.close();
