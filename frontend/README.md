# InventivAgents Cockpit (M5)

Vite + React + TypeScript SPA : authentification (`/auth/login`, `/auth/whoami`), registre org (`/org/register`), écran **Registry** (Owner/Admin), **Chat** SSE vers `POST /org/agents/:id/complete/stream`, affichage du dernier bloc **usage**.

## Prérequis

- API Rust en marche (`make run` depuis la racine du monorepo).
- CORS : par défaut le backend autorise `http://127.0.0.1:5173` et `http://localhost:5173`. Pour d’autres origines, définir **`INVENTIV_CORS_ORIGINS`** (liste séparée par des virgules) dans la `.env` chargée par l’API.

## Variables

Créer `frontend/.env.local` (non versionné) si besoin :

```bash
# URL de l’API (sans slash final)
VITE_API_BASE=http://127.0.0.1:8080
```

## Commandes

```bash
npm install
npm run dev      # http://127.0.0.1:5173
npm run build
npm run lint     # tsc --noEmit
```

Depuis la racine du repo : `make fe-install`, `make fe-dev`, `make fe-build`, `make fe-lint`.

JWT : stockage **sessionStorage** (`inventiv_jwt`), effacé à la fermetur du onglet.
