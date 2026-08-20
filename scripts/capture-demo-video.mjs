/**
 * Capture a README demo video of hub → Ativar → overlay, then desktop apps.
 *
 * Usage (Vite must be running on :1420):
 *   npm run dev
 *   node scripts/capture-demo-video.mjs
 *
 * Writes docs/screenshots/_demo-raw.mp4 then runs scripts/optimize-demo-video.sh
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";
import puppeteer from "puppeteer-core";
import { pathToFileURL } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const outDir = path.join(root, "docs/screenshots");
const framesDir = path.join(outDir, "_demo-frames");
const rawOut = path.join(outDir, "_demo-raw.mp4");
const baseUrl = process.env.MAESTRO_UI_URL || "http://localhost:1420/";
const fps = 12;
const chrome =
  process.env.CHROME_PATH ||
  ["/usr/bin/google-chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser"].find((p) =>
    fs.existsSync(p),
  );
const ffmpeg = fs.existsSync(path.join(root, ".tools/ffmpeg"))
  ? path.join(root, ".tools/ffmpeg")
  : "ffmpeg";

if (!chrome) {
  console.error("No Chrome/Chromium found");
  process.exit(1);
}

fs.rmSync(framesDir, { recursive: true, force: true });
fs.mkdirSync(framesDir, { recursive: true });

// Reuse the same mock catalog as portfolio screenshots (includes Demo README profile).
const mockSource = fs.readFileSync(
  path.join(__dirname, "capture-portfolio-screenshots.mjs"),
  "utf8",
);
// Extract mockSource string from the screenshots script by evaluating a slim duplicate:
const mockIife = `(() => {
  const profilesRoot = '/home/demo/.local/share/maestro/profiles';
  const profiles = {
    'demo-readme.json': {
      schema_version: 1,
      session_id: 'aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee',
      name: 'Demo README',
      applications: [
        { executable: '/usr/bin/gnome-terminal', args: [], cwd: null, skip_if_running: true, browser: null },
        {
          executable: '/usr/bin/firefox',
          args: [],
          cwd: null,
          skip_if_running: true,
          browser: {
            family: 'firefox',
            user_data_dir: null,
            firefox_profile: null,
            firefox_no_remote: null,
            urls: ['https://example.com', 'https://github.com/DaltonDjoesman/maestro-sessionManager'],
          },
        },
      ],
      browser: null,
      cleanup: null,
    },
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
        return [];
      case 'load_session_profile': {
        const key = (args.path || '').split('/').pop();
        const p = profiles[key];
        if (!p) throw new Error('profile not found');
        return structuredClone(p);
      }
      case 'activate_session_profile': {
        const key = (args.path || '').split('/').pop();
        const p = profiles[key];
        await new Promise((r) => setTimeout(r, 600));
        const steps = (p?.applications || []).map((app, i) => ({
          kind: app.browser ? 'browser' : 'application',
          label: app.executable.split('/').pop(),
          status: 'success',
          detail: app.browser ? (app.browser.urls || []).join(', ') : null,
          pid: 3000 + i,
        }));
        return { steps, activationLogPath: '/tmp/maestro-activation.log' };
      }
      case 'preview_session_activation':
        return [];
      case 'detect_system_default_browser':
        return { executable: '/usr/bin/firefox', family: 'firefox', desktopEntry: 'firefox.desktop' };
      case 'create_session_profile':
        return { filePath: profilesRoot + '/nova-sessao.json' };
      case 'save_session_profile':
        return null;
      case 'read_clipboard_text':
        return '';
      default:
        return null;
    }
  };
})();`;

void mockSource; // keep import side-effect free; mockIife is the source of truth for this script

let frameIndex = 0;

async function injectCursor(page) {
  await page.evaluate(() => {
    if (document.getElementById("maestro-demo-cursor")) return;
    const c = document.createElement("div");
    c.id = "maestro-demo-cursor";
    c.style.cssText =
      "position:fixed;z-index:2147483647;width:18px;height:18px;pointer-events:none;" +
      "left:0;top:0;transform:translate(-2px,-2px);" +
      "background:url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='18' height='18'><path d='M1 1 L1 15 L5 12 L8 17 L10 16 L7 11 L13 11 Z' fill='black' stroke='white' stroke-width='1.2'/></svg>\") no-repeat;" +
      "filter:drop-shadow(0 1px 2px rgba(0,0,0,.45));";
    document.documentElement.appendChild(c);
    window.__maestroMoveCursor = (x, y) => {
      c.style.left = `${x}px`;
      c.style.top = `${y}px`;
    };
  });
}

async function moveCursor(page, x, y, steps = 8) {
  const cur = await page.evaluate(() => {
    const el = document.getElementById("maestro-demo-cursor");
    return el
      ? { x: parseFloat(el.style.left) || 0, y: parseFloat(el.style.top) || 0 }
      : { x: 0, y: 0 };
  });
  for (let i = 1; i <= steps; i++) {
    const nx = cur.x + ((x - cur.x) * i) / steps;
    const ny = cur.y + ((y - cur.y) * i) / steps;
    await page.evaluate(
      (px, py) => window.__maestroMoveCursor && window.__maestroMoveCursor(px, py),
      nx,
      ny,
    );
    await page.mouse.move(nx, ny);
    await snap(page);
  }
}

async function snap(page) {
  const file = path.join(framesDir, `f-${String(frameIndex).padStart(5, "0")}.png`);
  frameIndex += 1;
  await page.screenshot({ path: file, type: "png" });
}

async function hold(page, seconds) {
  const n = Math.max(1, Math.round(seconds * fps));
  for (let i = 0; i < n; i++) await snap(page);
}

async function clickText(page, text) {
  const box = await page.evaluate((label) => {
    const buttons = [...document.querySelectorAll("button")];
    const el = buttons.find((b) => (b.textContent || "").trim().includes(label));
    if (!el) return null;
    const r = el.getBoundingClientRect();
    return { x: r.x + r.width / 2, y: r.y + r.height / 2 };
  }, text);
  if (!box) throw new Error(`button not found: ${text}`);
  await moveCursor(page, box.x, box.y);
  await page.mouse.click(box.x, box.y);
  await hold(page, 0.35);
}

async function captureUiFrames() {
  const browser = await puppeteer.launch({
    executablePath: chrome,
    headless: "new",
    args: ["--no-sandbox", "--disable-gpu", "--window-size=1440,900"],
    defaultViewport: { width: 1440, height: 900 },
  });
  const page = await browser.newPage();
  await page.evaluateOnNewDocument(mockIife);
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await page.waitForFunction(() => document.body.innerText.includes("Demo README"), {
    timeout: 15000,
  });
  await injectCursor(page);
  await moveCursor(page, 200, 200, 4);
  await hold(page, 1.2);

  // Prefer Demo README card's Ativar — click the first Ativar near Demo README
  const activateBox = await page.evaluate(() => {
    const cards = [...document.querySelectorAll("article, [class*='card'], li, div")];
    const demo = cards.find((el) => (el.textContent || "").includes("Demo README"));
    const scope = demo || document.body;
    const btn = [...scope.querySelectorAll("button")].find((b) =>
      (b.textContent || "").trim().includes("Ativar"),
    );
    if (!btn) return null;
    const r = btn.getBoundingClientRect();
    return { x: r.x + r.width / 2, y: r.y + r.height / 2 };
  });
  if (!activateBox) throw new Error("Ativar on Demo README not found");
  await moveCursor(page, activateBox.x, activateBox.y, 14);
  await hold(page, 0.4);
  await page.mouse.click(activateBox.x, activateBox.y);

  await page.waitForFunction(
    () =>
      document.body.innerText.includes("Terminal Maestro") ||
      document.body.innerText.includes("Activação concluída") ||
      document.body.innerText.includes("gnome-terminal") ||
      document.body.innerText.includes("firefox"),
    { timeout: 10000 },
  );
  await hold(page, 2.0);

  // Close overlay
  const closeBox = await page.evaluate(() => {
    const buttons = [...document.querySelectorAll("button")];
    const el =
      buttons.find((b) => (b.textContent || "").trim() === "Fechar") ||
      buttons.find((b) => (b.textContent || "").trim().includes("Fechar"));
    if (!el) return null;
    const r = el.getBoundingClientRect();
    return { x: r.x + r.width / 2, y: r.y + r.height / 2 };
  });
  if (closeBox) {
    await moveCursor(page, closeBox.x, closeBox.y, 10);
    await page.mouse.click(closeBox.x, closeBox.y);
  } else {
    await page.keyboard.press("Escape");
  }
  await hold(page, 1.0);
  await browser.close();
}

async function captureDesktopApps() {
  // Privacy-safe full-screen mock: Cosmic-like chrome + terminal + Firefox (example.com).
  // For a live OBS re-record, see docs/screenshots/README.md.
  const stageUrl = pathToFileURL(path.join(root, "scripts/demo-desktop-stage.html")).href;
  const browser = await puppeteer.launch({
    executablePath: chrome,
    headless: "new",
    args: ["--no-sandbox", "--disable-gpu", "--window-size=1440,900"],
    defaultViewport: { width: 1440, height: 900 },
  });
  const page = await browser.newPage();
  await page.goto(stageUrl, { waitUntil: "networkidle0" });
  await page.evaluate(() => {
    window.__maestroMoveCursor = (x, y) => {
      const c = document.getElementById("cursor");
      if (c) {
        c.style.left = `${x}px`;
        c.style.top = `${y}px`;
      }
    };
  });
  await hold(page, 0.8);
  await moveCursor(page, 280, 220, 10);
  await hold(page, 1.0);
  await moveCursor(page, 980, 360, 12);
  await hold(page, 1.6);
  await browser.close();
}

function encodeRaw() {
  const list = fs.readdirSync(framesDir).filter((f) => /^f-\d+\.png$/.test(f));
  if (list.length === 0) throw new Error("no frames captured");
  console.log(`Encoding ${list.length} frames → ${rawOut}`);
  const r = spawnSync(
    ffmpeg,
    [
      "-y",
      "-framerate",
      String(fps),
      "-i",
      path.join(framesDir, "f-%05d.png"),
      "-vf",
      "scale=1280:-2:flags=lanczos",
      "-c:v",
      "libx264",
      "-pix_fmt",
      "yuv420p",
      "-crf",
      "23",
      "-an",
      rawOut,
    ],
    { encoding: "utf8" },
  );
  if (r.status !== 0) {
    console.error(r.stderr);
    throw new Error("ffmpeg encode failed");
  }
}

async function main() {
  // Ensure Vite is up
  try {
    const res = await fetch(baseUrl);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
  } catch (e) {
    console.error(`UI not reachable at ${baseUrl}. Start with: npm run dev`);
    console.error(e.message || e);
    process.exit(1);
  }

  console.log("Capturing UI frames…");
  await captureUiFrames();
  console.log("Capturing desktop apps…");
  await captureDesktopApps();
  encodeRaw();
  console.log("Optimizing…");
  const opt = spawnSync("bash", [path.join(root, "scripts/optimize-demo-video.sh"), rawOut], {
    encoding: "utf8",
    env: { ...process.env, START: "0", END: "" },
  });
  process.stdout.write(opt.stdout || "");
  process.stderr.write(opt.stderr || "");
  if (opt.status !== 0) process.exit(opt.status || 1);

  // Cleanup temp frames (keep raw for re-optimize)
  fs.rmSync(framesDir, { recursive: true, force: true });
  console.log("Done.");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
