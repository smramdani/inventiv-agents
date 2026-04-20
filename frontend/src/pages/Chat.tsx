import { FormEvent, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Navigate } from "react-router-dom";
import { ApiError, apiBase, apiFetch, getToken } from "../api/client";
import type { Agent } from "../api/types";
import { useAuth } from "../auth/AuthContext";
import { consumeSsePost } from "../utils/sse";

type Usage = { input_tokens?: number; output_tokens?: number };

export default function Chat() {
  const { me, ready } = useAuth();
  const [agents, setAgents] = useState<Agent[]>([]);
  const [agentId, setAgentId] = useState("");
  const [model, setModel] = useState("gpt-4o-mini");
  const [message, setMessage] = useState("");
  const [out, setOut] = useState("");
  const [usage, setUsage] = useState<Usage | null>(null);
  const [traceId, setTraceId] = useState<string | null>(null);
  const [err, setErr] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const abortRef = useRef<AbortController | null>(null);

  const loadAgents = useCallback(async () => {
    try {
      const list = await apiFetch<Agent[]>("/org/agents");
      setAgents(list);
      setAgentId((prev) => (prev ? prev : list[0]?.id ?? ""));
    } catch (e) {
      setErr(e instanceof ApiError ? e.message : String(e));
    }
  }, []);

  useEffect(() => {
    if (ready && me) {
      void loadAgents();
    }
  }, [ready, me, loadAgents]);

  const canSend = useMemo(
    () => ready && me && agentId && message.trim() && !busy,
    [ready, me, agentId, message, busy],
  );

  if (ready && !me) {
    return <Navigate to="/login" replace />;
  }

  async function onSubmit(e: FormEvent) {
    e.preventDefault();
    if (!canSend) {
      return;
    }
    setErr(null);
    setOut("");
    setUsage(null);
    setTraceId(null);
    setBusy(true);
    abortRef.current?.abort();
    abortRef.current = new AbortController();
    const token = getToken();
    if (!token) {
      setBusy(false);
      return;
    }
    const url = `${apiBase()}/org/agents/${agentId}/complete/stream`;
    const body = JSON.stringify({
      message: message.trim(),
      model: model.trim(),
      max_tokens: 1024,
    });
    try {
      await consumeSsePost(
        url,
        { Authorization: `Bearer ${token}` },
        body,
        (ev, data) => {
          if (ev === "meta") {
            const m = data as { trace_id?: string };
            setTraceId(m.trace_id ?? null);
          }
          if (ev === "delta") {
            const d = data as { content?: string };
            if (d.content) {
              setOut((o) => o + d.content);
            }
          }
          if (ev === "usage") {
            setUsage(data as Usage);
          }
          if (ev === "error") {
            const er = data as { message?: string };
            setErr(er.message ?? "Stream error");
          }
        },
        abortRef.current.signal,
      );
    } catch (x) {
      if ((x as Error).name !== "AbortError") {
        setErr(String(x));
      }
    } finally {
      setBusy(false);
    }
  }

  function onStop() {
    abortRef.current?.abort();
    setBusy(false);
  }

  if (!ready) {
    return (
      <div className="layout">
        <p className="muted">Loading…</p>
      </div>
    );
  }

  return (
    <div className="layout">
      <div className="card">
        <h2>Agent chat (SSE)</h2>
        <p className="muted">
          Calls <code>POST /org/agents/:id/complete/stream</code>. Requires a configured LLM
          provider on the agent.
        </p>
        {err ? <p className="err">{err}</p> : null}
        <form onSubmit={onSubmit}>
          <div className="row">
            <div className="field">
              <label>Agent</label>
              <select value={agentId} onChange={(e) => setAgentId(e.target.value)} required>
                <option value="" disabled>
                  {agents.length ? "Select…" : "No agents"}
                </option>
                {agents.map((a) => (
                  <option key={a.id} value={a.id}>
                    {a.name}
                  </option>
                ))}
              </select>
            </div>
            <div className="field">
              <label>Model id</label>
              <input value={model} onChange={(e) => setModel(e.target.value)} required />
            </div>
          </div>
          <div className="field" style={{ marginBottom: "0.75rem" }}>
            <label>Message</label>
            <textarea value={message} onChange={(e) => setMessage(e.target.value)} required />
          </div>
          <div className="row">
            <button type="submit" className="btn" disabled={!canSend}>
              {busy ? "Streaming…" : "Send"}
            </button>
            {busy ? (
              <button type="button" className="btn secondary" onClick={onStop}>
                Stop
              </button>
            ) : null}
          </div>
        </form>
        {traceId ? (
          <p className="muted" style={{ marginTop: "0.75rem" }}>
            Trace: <code>{traceId}</code>
          </p>
        ) : null}
        <h3 style={{ marginTop: "1.25rem", fontSize: "0.95rem" }}>Assistant</h3>
        <div className="stream-out">{out || "—"}</div>
        <div className="usage-box">
          <strong>Last usage (SSE)</strong>
          <pre style={{ margin: "0.35rem 0 0", whiteSpace: "pre-wrap" }}>
            {usage
              ? JSON.stringify(usage, null, 2)
              : "No usage event yet for this turn."}
          </pre>
        </div>
      </div>
    </div>
  );
}
