import { FormEvent, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { apiBase } from "../api/client";

export default function Register() {
  const nav = useNavigate();
  const [name, setName] = useState("");
  const [adminEmail, setAdminEmail] = useState("");
  const [locale, setLocale] = useState("en_US");
  const [err, setErr] = useState<string | null>(null);
  const [ok, setOk] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function onSubmit(e: FormEvent) {
    e.preventDefault();
    setErr(null);
    setOk(null);
    setBusy(true);
    try {
      const res = await fetch(`${apiBase()}/org/register`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          name: name.trim(),
          admin_email: adminEmail.trim(),
          locale,
        }),
      });
      if (!res.ok) {
        const t = await res.text();
        setErr(t || res.statusText);
        return;
      }
      setOk("Organization created. You can sign in with the owner email.");
      setTimeout(() => nav("/login"), 2000);
    } catch (x) {
      setErr(String(x));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="layout">
      <div className="card" style={{ maxWidth: 480, margin: "3rem auto" }}>
        <h2>Create organization</h2>
        <p className="muted">Registers a new org and an Owner account (no JWT until login).</p>
        <form onSubmit={onSubmit}>
          <div className="field" style={{ marginBottom: "0.75rem" }}>
            <label htmlFor="oname">Organization name</label>
            <input
              id="oname"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
          </div>
          <div className="field" style={{ marginBottom: "0.75rem" }}>
            <label htmlFor="aemail">Owner email</label>
            <input
              id="aemail"
              type="email"
              value={adminEmail}
              onChange={(e) => setAdminEmail(e.target.value)}
              required
            />
          </div>
          <div className="field" style={{ marginBottom: "1rem" }}>
            <label htmlFor="loc">Locale</label>
            <select id="loc" value={locale} onChange={(e) => setLocale(e.target.value)}>
              <option value="en_US">en_US</option>
              <option value="fr_FR">fr_FR</option>
              <option value="ar_SA">ar_SA</option>
            </select>
          </div>
          {err ? <p className="err">{err}</p> : null}
          {ok ? <p style={{ color: "var(--ok)" }}>{ok}</p> : null}
          <button type="submit" className="btn" disabled={busy}>
            {busy ? "Creating…" : "Create"}
          </button>
        </form>
        <p className="muted" style={{ marginTop: "1.25rem" }}>
          <Link to="/login">Back to sign in</Link>
        </p>
      </div>
    </div>
  );
}
