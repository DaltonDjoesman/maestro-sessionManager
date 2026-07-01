import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AboutScreen } from "./components/AboutScreen";
import { ProfileCatalogScreen } from "./components/ProfileCatalogScreen";
import { ProfileEditorScreen } from "./components/ProfileEditorScreen";
import { SettingsScreen } from "./components/SettingsScreen";
import type { ProfileCatalogEntry } from "./types/profile";
import type { ApplicationSettings } from "./types/settings";
import { applyDataThemeToDocument } from "./themeDom";
import { loadLastSessionPath, saveLastSessionPath } from "./sessionCatalogUi";
import "./App.css";

type View = "home" | "profiles" | "profile-editor" | "settings" | "about";

function App() {
  const [view, setView] = useState<View>("home");
  const [platform, setPlatform] = useState("…");
  const [appSettings, setAppSettings] = useState<ApplicationSettings | null>(null);
  const [editorPath, setEditorPath] = useState<string | null>(null);

  const [lastSessionOk, setLastSessionOk] = useState(false);

  const refreshAppSettings = useCallback(() => {
    invoke<string>("platform_name")
      .then(setPlatform)
      .catch(() => setPlatform("unavailable"));

    invoke<ApplicationSettings>("get_settings")
      .then(setAppSettings)
      .catch(() => setAppSettings(null));
  }, []);

  useEffect(() => {
    refreshAppSettings();
  }, [refreshAppSettings]);

  const profilesRoot = appSettings?.profiles_root ?? "";
  const lastSessionPath = profilesRoot.trim() ? loadLastSessionPath(profilesRoot) : null;

  useEffect(() => {
    if (!lastSessionPath || !profilesRoot.trim()) {
      setLastSessionOk(false);
      return;
    }
    let cancelled = false;
    invoke<ProfileCatalogEntry[]>("list_session_profiles")
      .then((rows) => {
        if (cancelled) return;
        setLastSessionOk(rows.some((r) => r.filePath === lastSessionPath && r.valid));
      })
      .catch(() => {
        if (!cancelled) setLastSessionOk(false);
      });
    return () => {
      cancelled = true;
    };
  }, [lastSessionPath, profilesRoot, view]);

  useEffect(() => {
    if (!appSettings) return;
    return applyDataThemeToDocument(appSettings.theme);
  }, [appSettings?.theme]);

  useEffect(() => {
    if (view === "profile-editor" && !editorPath) {
      setView("profiles");
    }
  }, [view, editorPath]);

  const navigate = (next: View) => {
    if (next === "profile-editor") return;
    setEditorPath(null);
    setView(next);
  };

  const openEditor = (path: string) => {
    if (profilesRoot.trim()) {
      saveLastSessionPath(profilesRoot, path);
    }
    setEditorPath(path);
    setView("profile-editor");
  };

  const navLinkClass = (target: View) => {
    const active = view === target || (target === "profiles" && view === "profile-editor");
    return active ? "app-nav-link app-nav-link--active" : "app-nav-link";
  };

  const renderMain = () => {
    if (view === "settings") {
      return (
        <SettingsScreen
          onReloadSettings={refreshAppSettings}
          onThemePreview={(t) => {
            applyDataThemeToDocument(t);
          }}
        />
      );
    }
    if (view === "about") {
      return <AboutScreen />;
    }
    if (view === "profiles") {
      return (
        <ProfileCatalogScreen
          profilesRoot={profilesRoot}
          onEdit={openEditor}
        />
      );
    }
    if (view === "profile-editor" && editorPath) {
      return (
        <ProfileEditorScreen
          filePath={editorPath}
          onBack={() => {
            setView("profiles");
            setEditorPath(null);
          }}
        />
      );
    }
    return (
      <main className="container home-screen">
        <header className="hero">
          <h1>Welcome</h1>
          <p className="tagline">Pick a tab above to manage sessions or preferences.</p>
        </header>
        <section className="status-card home-status-card">
          <p>
            Platform: <strong>{platform}</strong>
          </p>
          <p>
            Profiles directory: <strong>{profilesRoot || "…"}</strong>
          </p>
          {lastSessionPath && lastSessionOk ? (
            <div className="home-continue-block">
              <button
                type="button"
                className="btn-primary home-continue-btn"
                onClick={() => openEditor(lastSessionPath)}
              >
                Continuar última sessão
              </button>
              <p className="hint home-continue-hint">
                Opens the profile you last edited from this catalog. If the file was removed, use Sessions to pick
                another.
              </p>
            </div>
          ) : null}
          <p className="hint" style={{ marginTop: "0.75rem" }}>
            Open <strong>Sessions</strong> to list, create, edit, duplicate, delete, and activate session profiles.
          </p>
        </section>
      </main>
    );
  };

  return (
    <div className="app-shell">
      <header className="app-top-bar">
        <div className="app-brand">
          <button type="button" className="app-brand-btn" onClick={() => navigate("home")}>
            Maestro
          </button>
          <span className="app-brand-sub">Linux sessions</span>
        </div>
        <nav className="app-nav" aria-label="Main">
          <button type="button" className={navLinkClass("home")} onClick={() => navigate("home")}>
            Home
          </button>
          <button type="button" className={navLinkClass("profiles")} onClick={() => navigate("profiles")}>
            Sessions
          </button>
          <button type="button" className={navLinkClass("settings")} onClick={() => navigate("settings")}>
            Settings
          </button>
          <button type="button" className={navLinkClass("about")} onClick={() => navigate("about")}>
            About
          </button>
        </nav>
      </header>

      <div className="app-main" key={view === "profile-editor" ? `editor-${editorPath}` : view}>
        {renderMain()}
      </div>
    </div>
  );
}

export default App;
