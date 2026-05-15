import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [platform, setPlatform] = useState("…");

  useEffect(() => {
    invoke<string>("platform_name")
      .then(setPlatform)
      .catch(() => setPlatform("unavailable"));
  }, []);

  return (
    <main className="container">
      <header className="hero">
        <h1>Maestro</h1>
        <p className="tagline">Session environment manager for Linux</p>
      </header>
      <section className="status-card">
        <p>
          Platform adapter: <strong>{platform}</strong>
        </p>
        <p className="hint">Profile catalog and activation UI arrive in later tasks.</p>
      </section>
    </main>
  );
}

export default App;
