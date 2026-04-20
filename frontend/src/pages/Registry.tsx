import { FormEvent, useCallback, useEffect, useState } from "react";
import { Navigate } from "react-router-dom";
import { ApiError, apiFetch } from "../api/client";
import type { Agent, LlmProvider, Skill, SkillType } from "../api/types";
import { canManageRegistry, useAuth } from "../auth/AuthContext";

export default function Registry() {
  const { me, ready } = useAuth();
  const [providers, setProviders] = useState<LlmProvider[]>([]);
  const [skills, setSkills] = useState<Skill[]>([]);
  const [agents, setAgents] = useState<Agent[]>([]);
  const [err, setErr] = useState<string | null>(null);

  const load = useCallback(async () => {
    setErr(null);
    try {
      const [p, s, a] = await Promise.all([
        apiFetch<LlmProvider[]>("/org/providers"),
        apiFetch<Skill[]>("/org/skills"),
        apiFetch<Agent[]>("/org/agents"),
      ]);
      setProviders(p);
      setSkills(s);
      setAgents(a);
    } catch (e) {
      setErr(e instanceof ApiError ? e.message : String(e));
    }
  }, []);

  useEffect(() => {
    if (ready && me && canManageRegistry(me.role)) {
      void load();
    }
  }, [ready, me, load]);

  if (ready && (!me || !canManageRegistry(me.role))) {
    return <Navigate to="/" replace />;
  }

  if (!ready || !me) {
    return (
      <div className="layout">
        <p className="muted">Loading…</p>
      </div>
    );
  }

  return (
    <div className="layout">
      {err ? <p className="err">{err}</p> : null}

      <div className="card">
        <h2>LLM providers</h2>
        <ProviderForm onCreated={load} />
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Base URL</th>
              <th>Active</th>
            </tr>
          </thead>
          <tbody>
            {providers.map((p) => (
              <tr key={p.id}>
                <td>{p.name}</td>
                <td>
                  <code>{p.base_url}</code>
                </td>
                <td>{p.is_active ? "yes" : "no"}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="card">
        <h2>Skills</h2>
        <SkillForm onCreated={load} />
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Type</th>
              <th>Endpoint</th>
            </tr>
          </thead>
          <tbody>
            {skills.map((s) => (
              <tr key={s.id}>
                <td>{s.name}</td>
                <td>{s.skill_type}</td>
                <td>
                  <code>{s.endpoint_url ?? "—"}</code>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="card">
        <h2>Agents</h2>
        <AgentForm providers={providers} onCreated={load} />
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Mission</th>
              <th>LLM provider</th>
            </tr>
          </thead>
          <tbody>
            {agents.map((a) => (
              <tr key={a.id}>
                <td>{a.name}</td>
                <td>{a.mission}</td>
                <td>
                  <code>{a.llm_provider_id ?? "—"}</code>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

function ProviderForm({ onCreated }: { onCreated: () => void }) {
  const [name, setName] = useState("");
  const [baseUrl, setBaseUrl] = useState("https://api.openai.com");
  const [apiKey, setApiKey] = useState("");
  const [busy, setBusy] = useState(false);
  const [localErr, setLocalErr] = useState<string | null>(null);

  async function onSubmit(e: FormEvent) {
    e.preventDefault();
    setLocalErr(null);
    setBusy(true);
    try {
      await apiFetch<{ id: string }>("/org/providers", {
        method: "POST",
        json: {
          name: name.trim(),
          base_url: baseUrl.trim(),
          api_key: apiKey.trim() || null,
        },
      });
      setName("");
      setApiKey("");
      onCreated();
    } catch (x) {
      setLocalErr(x instanceof ApiError ? x.message : String(x));
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={onSubmit} className="row">
      <div className="field">
        <label>Name</label>
        <input value={name} onChange={(e) => setName(e.target.value)} required />
      </div>
      <div className="field">
        <label>Base URL</label>
        <input value={baseUrl} onChange={(e) => setBaseUrl(e.target.value)} required />
      </div>
      <div className="field">
        <label>API key</label>
        <input
          type="password"
          autoComplete="off"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
          placeholder="optional for local mock"
        />
      </div>
      <button type="submit" className="btn" disabled={busy}>
        Add provider
      </button>
      {localErr ? <p className="err">{localErr}</p> : null}
    </form>
  );
}

function SkillForm({ onCreated }: { onCreated: () => void }) {
  const [name, setName] = useState("");
  const [skillType, setSkillType] = useState<SkillType>("Native");
  const [endpointUrl, setEndpointUrl] = useState("");
  const [busy, setBusy] = useState(false);
  const [localErr, setLocalErr] = useState<string | null>(null);

  async function onSubmit(e: FormEvent) {
    e.preventDefault();
    setLocalErr(null);
    setBusy(true);
    try {
      await apiFetch("/org/skills", {
        method: "POST",
        json: {
          name: name.trim(),
          skill_type: skillType,
          endpoint_url: skillType === "MCP" ? endpointUrl.trim() || null : null,
        },
      });
      setName("");
      setEndpointUrl("");
      onCreated();
    } catch (x) {
      setLocalErr(x instanceof ApiError ? x.message : String(x));
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={onSubmit} className="row">
      <div className="field">
        <label>Name</label>
        <input value={name} onChange={(e) => setName(e.target.value)} required />
      </div>
      <div className="field">
        <label>Type</label>
        <select value={skillType} onChange={(e) => setSkillType(e.target.value as SkillType)}>
          <option value="Native">Native</option>
          <option value="MCP">MCP</option>
        </select>
      </div>
      {skillType === "MCP" ? (
        <div className="field">
          <label>MCP endpoint URL</label>
          <input
            value={endpointUrl}
            onChange={(e) => setEndpointUrl(e.target.value)}
            placeholder="https://…"
            required
          />
        </div>
      ) : null}
      <button type="submit" className="btn" disabled={busy}>
        Add skill
      </button>
      {localErr ? <p className="err">{localErr}</p> : null}
    </form>
  );
}

function AgentForm({
  providers,
  onCreated,
}: {
  providers: LlmProvider[];
  onCreated: () => void;
}) {
  const [name, setName] = useState("");
  const [mission, setMission] = useState("");
  const [llmPid, setLlmPid] = useState("");
  const [busy, setBusy] = useState(false);
  const [localErr, setLocalErr] = useState<string | null>(null);

  async function onSubmit(e: FormEvent) {
    e.preventDefault();
    setLocalErr(null);
    setBusy(true);
    try {
      await apiFetch("/org/agents", {
        method: "POST",
        json: {
          name: name.trim(),
          mission: mission.trim(),
          llm_provider_id: llmPid ? llmPid : null,
        },
      });
      setName("");
      setMission("");
      onCreated();
    } catch (x) {
      setLocalErr(x instanceof ApiError ? x.message : String(x));
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={onSubmit} className="row">
      <div className="field">
        <label>Name</label>
        <input value={name} onChange={(e) => setName(e.target.value)} required />
      </div>
      <div className="field">
        <label>Mission</label>
        <input value={mission} onChange={(e) => setMission(e.target.value)} required />
      </div>
      <div className="field">
        <label>LLM provider</label>
        <select value={llmPid} onChange={(e) => setLlmPid(e.target.value)}>
          <option value="">—</option>
          {providers.map((p) => (
            <option key={p.id} value={p.id}>
              {p.name}
            </option>
          ))}
        </select>
      </div>
      <button type="submit" className="btn" disabled={busy}>
        Add agent
      </button>
      {localErr ? <p className="err">{localErr}</p> : null}
    </form>
  );
}
