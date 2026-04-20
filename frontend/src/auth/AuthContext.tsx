import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import type { WhoAmI } from "../api/types";
import { apiFetch, getToken, postTelemetry, setToken } from "../api/client";

type AuthState = {
  me: WhoAmI | null;
  ready: boolean;
  login: (email: string) => Promise<void>;
  logout: () => void;
  refresh: () => Promise<void>;
};

const AuthContext = createContext<AuthState | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [me, setMe] = useState<WhoAmI | null>(null);
  const [ready, setReady] = useState(false);

  const refresh = useCallback(async () => {
    const t = getToken();
    if (!t) {
      setMe(null);
      setReady(true);
      return;
    }
    try {
      const w = await apiFetch<WhoAmI>("/auth/whoami");
      setMe(w);
    } catch (e) {
      setToken(null);
      setMe(null);
      void postTelemetry("WARN", "whoami_failed", {
        error: String(e),
      });
    } finally {
      setReady(true);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const login = useCallback(async (email: string) => {
    setToken(null);
    const res = await apiFetch<{ token: string }>("/auth/login", {
      method: "POST",
      json: { email },
    });
    setToken(res.token);
    await refresh();
  }, [refresh]);

  const logout = useCallback(() => {
    setToken(null);
    setMe(null);
  }, []);

  const value = useMemo(
    () => ({ me, ready, login, logout, refresh }),
    [me, ready, login, logout, refresh],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthState {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error("useAuth outside AuthProvider");
  }
  return ctx;
}

export function canManageRegistry(role: string | undefined): boolean {
  return role === "Owner" || role === "Admin";
}
