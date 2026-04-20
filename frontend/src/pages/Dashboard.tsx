import { Link } from "react-router-dom";
import { canManageRegistry, useAuth } from "../auth/AuthContext";

export default function Dashboard() {
  const { me } = useAuth();
  const admin = canManageRegistry(me?.role);

  return (
    <div className="layout">
      <div className="card">
        <h2>Welcome</h2>
        <p className="muted">
          Signed in as <strong>{me?.user_id}</strong> — org <code>{me?.org_id}</code> — role{" "}
          <strong>{me?.role}</strong>
        </p>
        <ul style={{ margin: "1rem 0", paddingLeft: "1.25rem" }}>
          <li>
            <Link to="/chat">Chat with an agent</Link> (SSE completion)
          </li>
          {admin ? (
            <li>
              <Link to="/registry">Registry</Link> (providers, skills, agents)
            </li>
          ) : (
            <li className="muted">Registry is limited to Owner / Admin.</li>
          )}
        </ul>
      </div>
    </div>
  );
}
