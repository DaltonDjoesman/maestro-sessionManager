import { useCallback, useEffect, useState, type ReactNode } from "react";
import { pt } from "../i18n/pt";
import { Sidebar, type AppRoute } from "./Sidebar";

interface AppShellProps {
  route: AppRoute;
  showEditorNav: boolean;
  activeSessionLabel: string | null;
  version: string;
  resolvedTheme: "light" | "dark";
  contentLayout?: "scroll" | "pane";
  onNavigate: (route: AppRoute) => void;
  onThemeToggle: () => void;
  onClearActiveSession: () => void;
  children: ReactNode;
}

export function AppShell({
  route,
  showEditorNav,
  activeSessionLabel,
  version,
  resolvedTheme,
  contentLayout = "scroll",
  onNavigate,
  onThemeToggle,
  onClearActiveSession,
  children,
}: AppShellProps) {
  const [drawerOpen, setDrawerOpen] = useState(false);

  const closeDrawer = useCallback(() => setDrawerOpen(false), []);

  useEffect(() => {
    const mq = window.matchMedia("(min-width: 900px)");
    const onChange = () => {
      if (mq.matches) setDrawerOpen(false);
    };
    mq.addEventListener("change", onChange);
    return () => mq.removeEventListener("change", onChange);
  }, []);

  return (
    <div className="app-shell">
      <div className="app-window">
        <header className="app-header">
          <div className="app-title-wrapper">
            <svg
              className="app-logo"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2.5"
              strokeLinecap="round"
              strokeLinejoin="round"
              aria-hidden
            >
              <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
            </svg>
            <span className="app-title">{pt.appName}</span>
          </div>
          <div className="window-actions">
            <button
              type="button"
              className="btn-icon theme-toggle-btn"
              aria-label={pt.shell.toggleTheme}
              title={pt.shell.toggleTheme}
              onClick={onThemeToggle}
            >
              {resolvedTheme === "dark" ? (
                <svg width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2" aria-hidden>
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                  />
                </svg>
              ) : (
                <svg width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2" aria-hidden>
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                  />
                </svg>
              )}
            </button>
          </div>
        </header>

        <div className="app-body">
          <Sidebar
            route={route}
            showEditorNav={showEditorNav}
            activeSessionLabel={activeSessionLabel}
            version={version}
            drawerOpen={drawerOpen}
            onNavigate={onNavigate}
            onCloseDrawer={closeDrawer}
            onClearActiveSession={onClearActiveSession}
          />
          <div className="app-frame">
            <header className="app-mobile-bar">
              <button
                type="button"
                className="btn-icon"
                aria-label={pt.nav.openMenu}
                onClick={() => setDrawerOpen(true)}
              >
                ☰
              </button>
              <span className="app-mobile-title">{pt.appName}</span>
            </header>
            <main className={`app-content${contentLayout === "pane" ? " app-content--pane" : ""}`}>{children}</main>
          </div>
        </div>
      </div>
    </div>
  );
}

export type { AppRoute };
