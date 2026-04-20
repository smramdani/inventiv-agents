import { NavLink, Outlet } from "react-router-dom";
import { apiBase } from "../api/client";
import { canManageRegistry, useAuth } from "../auth/AuthContext";
import { APP_VERSION } from "../version";

export default function AppShell() {
  const { me, ready, logout } = useAuth();
  const navCls = ({ isActive }: { isActive: boolean }) => (isActive ? "active" : undefined);

  return (
    <div>
      <header className="layout topbar">
        <div className="brand">InventivAgents Cockpit</div>
        <nav className="nav">
          {me ? (
            <>
              <NavLink to="/" end className={navCls}>
                Home
              </NavLink>
              <NavLink to="/chat" className={navCls}>
                Chat
              </NavLink>
              {canManageRegistry(me.role) ? (
                <NavLink to="/registry" className={navCls}>
                  Registry
                </NavLink>
              ) : null}
              <button type="button" className="btn secondary" onClick={logout}>
                Sign out
              </button>
            </>
          ) : ready ? (
            <>
              <NavLink to="/login" className={navCls}>
                Sign in
              </NavLink>
              <NavLink to="/register" className={navCls}>
                Register org
              </NavLink>
            </>
          ) : null}
        </nav>
      </header>
      <Outlet />
      <footer className="layout muted" style={{ paddingBottom: "2rem", fontSize: "0.8rem" }}>
        Cockpit v{APP_VERSION} · API <code>{apiBase()}</code>
      </footer>
    </div>
  );
}
