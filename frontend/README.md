# Frontend (M5 — Sovereign Cockpit)

Application web **cockpit** : auth, registre (providers / skills / agents), sessions SSE, usage/cost.

Le dépôt est un **monorepo** : le code front vivra ici (`frontend/`) ; l’API Rust est dans `../backend/`. Les specs et le tooling partagés restent à la **racine** (`specify/`, `scripts/`, `Makefile`, `docker-compose.yml`).

## Prochaines étapes

Choisir une stack (ex. Vite + React, Next.js), ajouter `package.json` / lockfile, et documenter `npm run dev` dans le README racine une fois le squelette en place.
