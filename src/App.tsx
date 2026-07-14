import { useCallback, useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ProfileEditorScreen } from "./components/ProfileEditorScreen";
import { SettingsScreen } from "./components/SettingsScreen";
import { AppShell, type AppRoute } from "./layout/AppShell";
import { CaptureAssistantPage } from "./pages/CaptureAssistantPage";
import { SessionHubPage } from "./pages/SessionHubPage";
import {
  loadActiveSessionLabel,
  saveActiveSessionLabel,
  saveLastSessionPath,
} from "./sessionCatalogUi";
import { applyDataThemeToDocument, resolveTheme } from "./themeDom";
import type { ApplicationSettings } from "./types/settings";
import "./styles/tokens.css";
import "./styles/themes.css";
import "./styles/components.css";
import "./App.css";

function App() {
  const [route, setRoute] = useState<AppRoute>("hub");
  const [appSettings, setAppSettings] = useState<ApplicationSettings | null>(null);
  const [editorPath, setEditorPath] = useState<string | null>(null);
  const [activeSessionLabel, setActiveSessionLabel] = useState<string | null>(null);

  const version = import.meta.env.VITE_APP_VERSION ?? "0.1.0";
  const profilesRoot = appSettings?.profiles_root ?? "";
  const resolvedTheme = useMemo(
    () => resolveTheme(appSettings?.theme ?? "dark"),
    [appSettings?.theme],
  );

  const refreshAppSettings = useCallback(() => {
    invoke<ApplicationSettings>("get_settings")
      .then((s) => {
        setAppSettings(s);
        if (s.profiles_root.trim()) {
          setActiveSessionLabel(loadActiveSessionLabel(s.profiles_root));
        }
      })
      .catch(() => setAppSettings(null));
  }, []);

  useEffect(() => {
    refreshAppSettings();
  }, [refreshAppSettings]);

  useEffect(() => {
    if (!appSettings) return;
    return applyDataThemeToDocument(appSettings.theme);
  }, [appSettings?.theme]);

  const closeEditor = () => {
    setEditorPath(null);
    setRoute("hub");
  };

  const navigate = (next: AppRoute) => {
    if (next === "editor") {
      if (editorPath) setRoute("editor");
      return;
    }
    setEditorPath(null);
    setRoute(next);
  };

  const openEditor = (path: string) => {
    if (profilesRoot.trim()) {
      saveLastSessionPath(profilesRoot, path);
    }
    setEditorPath(path);
    setRoute("editor");
  };

  const handleActivated = (label: string) => {
    if (profilesRoot.trim()) {
      saveActiveSessionLabel(profilesRoot, label);
      setActiveSessionLabel(label);
    }
  };

  const toggleTheme = async () => {
    if (!appSettings) return;
    const nextTheme = resolvedTheme === "dark" ? "light" : "dark";
    try {
      const updated = await invoke<ApplicationSettings>("save_settings", {
        settings: { ...appSettings, theme: nextTheme },
      });
      setAppSettings(updated);
    } catch {
      /* keep current theme */
    }
  };

  const clearActiveSession = () => {
    if (profilesRoot.trim()) saveActiveSessionLabel(profilesRoot, null);
    setActiveSessionLabel(null);
  };

  const renderMain = () => {
    if (route === "settings") {
      return (
        <SettingsScreen
          onReloadSettings={refreshAppSettings}
          onThemePreview={(t) => applyDataThemeToDocument(t)}
        />
      );
    }
    if (route === "capture") {
      return <CaptureAssistantPage onEdit={openEditor} />;
    }
    if (editorPath) {
      return (
        <ProfileEditorScreen
          filePath={editorPath}
          profilesRoot={profilesRoot}
          onBack={closeEditor}
          onActivated={handleActivated}
          onDeleted={(label) => {
            if (activeSessionLabel === label) clearActiveSession();
          }}
        />
      );
    }
    return (
      <SessionHubPage profilesRoot={profilesRoot} onEdit={openEditor} onActivated={handleActivated} />
    );
  };

  return (
    <AppShell
      route={route}
      showEditorNav={!!editorPath}
      activeSessionLabel={activeSessionLabel}
      version={String(version)}
      resolvedTheme={resolvedTheme}
      contentLayout={route === "capture" ? "pane" : "scroll"}
      onNavigate={navigate}
      onThemeToggle={() => void toggleTheme()}
      onClearActiveSession={clearActiveSession}
    >
      <div
        className={`app-main-inner${route === "capture" ? " app-main-inner--pane-scroll" : ""}`}
        key={editorPath ? `editor-${editorPath}` : route}
      >
        {renderMain()}
      </div>
    </AppShell>
  );
}

export default App;
