# Milestone 1 : Socle Technique & Connexion LLM

## Tâches d'Initialisation (Cargo & Rust)

### Tâche 1.1 : Configuration de `Cargo.toml`
- [ ] Ajouter les dépendances : `tokio`, `axum`, `serde`, `serde_json`, `reqwest`, `anyhow`, `thiserror`, `tower-http`.
- [ ] Vérifier la compilation avec `cargo check`.

### Tâche 1.2 : Architecture de Base (Dossiers & Modules)
- [ ] Créer les dossiers de base : `src/api`, `src/core`, `src/infrastructure`.
- [ ] Définir les modules dans `src/main.rs`.

### Tâche 1.3 : Définition du Trait `Agent`
- [ ] Créer le trait `Agent` dans `src/core/agent.rs`.
- [ ] Le trait doit inclure une méthode asynchrone pour traiter un message.

### Tâche 1.4 : Serveur API "Hello World" (Axum)
- [ ] Créer un serveur Axum minimal avec un endpoint de healthcheck.
- [ ] Vérifier le fonctionnement en local.

### Tâche 1.5 : Client LLM de Base (Provider)
- [ ] Créer un client HTTP générique pour appeler une API (ex: OpenAI ou Ollama).
- [ ] Mapper les requêtes et réponses vers des structs Rust.

## Validation de Fin de Milestone
- [ ] Le serveur démarre sans erreur.
- [ ] Une requête HTTP peut être envoyée et l'agent répond avec un message simple.
- [ ] Les tests de base passent.
