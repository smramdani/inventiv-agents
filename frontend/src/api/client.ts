const TOKEN_KEY = "inventiv_jwt";

export function apiBase(): string {
  const b = import.meta.env.VITE_API_BASE;
  if (!b) {
    return "http://127.0.0.1:8080";
  }
  return b.replace(/\/$/, "");
}

export function getToken(): string | null {
  return sessionStorage.getItem(TOKEN_KEY);
}

export function setToken(token: string | null): void {
  if (token) {
    sessionStorage.setItem(TOKEN_KEY, token);
  } else {
    sessionStorage.removeItem(TOKEN_KEY);
  }
}

export class ApiError extends Error {
  status: number;
  constructor(status: number, message: string) {
    super(message);
    this.status = status;
    this.name = "ApiError";
  }
}

async function parseErr(res: Response): Promise<string> {
  const t = await res.text();
  try {
    const j = JSON.parse(t) as { message?: string; error?: string };
    return (j.message ?? j.error) ?? (t || res.statusText);
  } catch {
    return t || res.statusText;
  }
}

type FetchInit = Omit<RequestInit, "body"> & { json?: unknown; body?: RequestInit["body"] };

export async function apiFetch<T>(path: string, init: FetchInit = {}): Promise<T> {
  const url = `${apiBase()}${path.startsWith("/") ? path : `/${path}`}`;
  const { json, body, ...rest } = init;
  const headers = new Headers(rest.headers);
  if (json !== undefined) {
    headers.set("Content-Type", "application/json");
  }
  const token = getToken();
  if (token) {
    headers.set("Authorization", `Bearer ${token}`);
  }
  const res = await fetch(url, {
    ...rest,
    headers,
    body: json !== undefined ? JSON.stringify(json) : body,
  });
  if (!res.ok) {
    throw new ApiError(res.status, await parseErr(res));
  }
  if (res.status === 204) {
    return undefined as T;
  }
  const ct = res.headers.get("content-type") ?? "";
  if (ct.includes("application/json")) {
    return (await res.json()) as T;
  }
  return (await res.text()) as T;
}

export async function postTelemetry(
  level: "WARN" | "ERROR",
  message: string,
  context: Record<string, unknown>,
): Promise<void> {
  try {
    await apiFetch("/telemetry/frontend", {
      method: "POST",
      json: [{ level, message, context }],
    });
  } catch {
    /* ignore */
  }
}
