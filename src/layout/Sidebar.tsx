import { t } from "../i18n";

export type AppRoute = "hub" | "editor" | "capture" | "settings";

interface SidebarProps {
  route: AppRoute;
  showEditorNav: boolean;
  activeSessionLabel: string | null;
  version: string;
  drawerOpen: boolean;
  onNavigate: (route: AppRoute) => void;
  onCloseDrawer: () => void;
  onClearActiveSession: () => void;
}

function NavIcon({ children }: { children: React.ReactNode }) {
  return (
    <svg className="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2" aria-hidden>
      {children}
    </svg>
  );
}

export function Sidebar({
  route,
  showEditorNav,
  activeSessionLabel,
  version,
  drawerOpen,
  onNavigate,
  onCloseDrawer,
  onClearActiveSession,
}: SidebarProps) {
  const nav = (target: AppRoute) => {
    onNavigate(target);
    onCloseDrawer();
  };

  const linkClass = (active?: boolean) => `nav-item${active ? " active" : ""}`;

  const hubActive = route === "hub";
  const editorActive = route === "editor";
  const captureActive = route === "capture";
  const settingsActive = route === "settings";

  return (
    <>
      {drawerOpen ? (
        <button
          type="button"
          className="sidebar-backdrop"
          aria-label={t.nav.closeMenu}
          onClick={onCloseDrawer}
        />
      ) : null}
      <aside className={`app-sidebar${drawerOpen ? " app-sidebar--open" : ""}`} aria-label="Principal">
        <div className="nav-group">
          <button type="button" className={linkClass(hubActive)} onClick={() => nav("hub")}>
            <NavIcon>
              <path strokeLinecap="round" strokeLinejoin="round" d="M4 6h16M4 12h16M4 18h7" />
            </NavIcon>
            {t.nav.sessions}
          </button>
          {showEditorNav ? (
            <button type="button" className={linkClass(editorActive)} onClick={() => nav("editor")}>
              <NavIcon>
                <path strokeLinecap="round" strokeLinejoin="round" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </NavIcon>
              {t.nav.editor}
            </button>
          ) : null}
          <button type="button" className={linkClass(captureActive)} onClick={() => nav("capture")}>
            <NavIcon>
              <path strokeLinecap="round" strokeLinejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              <path strokeLinecap="round" strokeLinejoin="round" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
            </NavIcon>
            {t.nav.capture}
          </button>
          <button type="button" className={linkClass(settingsActive)} onClick={() => nav("settings")}>
            <NavIcon>
              <path strokeLinecap="round" strokeLinejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path strokeLinecap="round" strokeLinejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </NavIcon>
            {t.nav.settings}
          </button>
        </div>

        <div className="sidebar-footer">
          <div className="current-session-widget">
            <span className="widget-label">{t.shell.activeSession}</span>
            <div className="widget-status">
              <span className={`status-dot${activeSessionLabel ? " active" : ""}`} aria-hidden />
              <span>{activeSessionLabel ?? t.shell.noActiveSession}</span>
            </div>
            {activeSessionLabel ? (
              <button type="button" className="btn btn-compact" onClick={onClearActiveSession}>
                {t.shell.clearIndicator}
              </button>
            ) : null}
          </div>
          <span className="sidebar-version">
            {t.settings.version} {version}
          </span>
        </div>
      </aside>
    </>
  );
}
