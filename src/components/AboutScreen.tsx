/** Static about / credits view (version from build-time package.json via Vite). */
export function AboutScreen() {
  const version = import.meta.env.VITE_APP_VERSION ?? "0.1.0";

  return (
    <main className="container about-screen">
      <header className="hero">
        <h1>About Maestro</h1>
        <p className="tagline">Session environment manager for Linux</p>
      </header>

      <section className="status-card about-card">
        <p>
          <strong>Version</strong> {String(version)}
        </p>
        <p className="hint" style={{ marginTop: "0.75rem" }}>
          Maestro stores session profiles as JSON, launches your applications and optional browser URLs in
          order, and reports activation results. Use the <strong>Sessions</strong> tab to manage profiles.
        </p>
        <p className="hint">
          Built with Tauri 2, React, and Rust. Profiles and settings live on your machine under the
          configured profiles directory.
        </p>
      </section>
    </main>
  );
}
