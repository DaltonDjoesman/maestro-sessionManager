import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SettingsScreen } from "./components/SettingsScreen";
import type { ApplicationSettings } from "./types/settings";
import "./App.css";

type View = "home" | "settings";

function App() {
  const [view, setView] = useState<View>("home");
  const [platform, setPlatform] = useState("…");
  const [profilesRoot, setProfilesRoot] = useState("…");

  useEffect(() => {
    if (view !== "home") return;

    invoke<string>("platform_name")
      .then(setPlatform)
      .catch(() => setPlatform("unavailable"));

    invoke<ApplicationSettings>("get_settings")
      .then((s) => setProfilesRoot(s.profiles_root))
      .catch(() => setProfilesRoot("unavailable"));
  }, [view]);

  if (view === "settings") {
    return <SettingsScreen onBack={() => setView("home")} />;
  }

  return (
    <main className="container">
      <header className="hero row-between">
        <div>
          <h1>Maestro</h1>
          <p className="tagline">Session environment manager for Linux</p>
        </div>
        <button
          type="button"
          className="btn-secondary"
          onClick={() => setView("settings")}
        >
          Settings
        </button>
      </header>
      <section className="status-card">
        <p>
          Platform adapter: <strong>{platform}</strong>
        </p>
        <p>
          Profiles directory: <strong>{profilesRoot}</strong>
        </p>
        <p className="hint">Profile catalog and activation UI arrive in later tasks.</p>
      </section>
    </main>
  );
}

export default App;
