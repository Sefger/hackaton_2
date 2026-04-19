# Hackathon 2 — Indexation & Search

Rust workspace implementing the Samsung ИТ-Академии 2026 "Индексация и поиск по сообщениям" hackathon:

- **index-service** — `/health`, `/index` (per-message chunker with chat/thread metadata), `/sparse_embedding` (local BGE-M3 via fastembed).
- **search-service** — `/health`, `/search` (hybrid dense+sparse retrieval via Qdrant Query API + external reranker).

Both services ship as Docker images with the BGE-M3 sparse model pre-baked, so they run **offline** in the hackathon evaluation environment.

## Local development (docker compose)

Full stack with mocks for `EMBEDDINGS_DENSE_URL` and `RERANKER_URL`, plus seeded sample data.

```
docker compose up --build
```

Endpoints:
- Qdrant — http://localhost:6333
- index-service — http://localhost:8081
- search-service — http://localhost:8082
- mock-dense — http://localhost:9001
- mock-rerank — http://localhost:9002
- frontend — http://localhost:3000

Smoke:
```
curl http://localhost:8081/health
curl http://localhost:8082/health
curl -X POST http://localhost:8082/search \
  -H 'Content-Type: application/json' \
  -d @scripts/sample_search.json
```

Tear down:
```
docker compose down -v
```

## Production build (hackathon registry)

Build `linux/amd64` images for the organizer's registry:

```
docker buildx build --platform linux/amd64 \
  -f index-service/Dockerfile \
  -t <registry>/<team_id>/index-service:latest .

docker buildx build --platform linux/amd64 \
  -f search-service/Dockerfile \
  -t <registry>/<team_id>/search-service:latest .

docker push <registry>/<team_id>/index-service:latest
docker push <registry>/<team_id>/search-service:latest
```

The mocks and docker-compose are strictly local-dev. Production uses only the two service images.

## Tests

```
cargo test --workspace
```

Coverage:
- `shared` — DTO serde (full TZ sample, minimal, roundtrip).
- `index-service` — chunker (per-message, filters, thread context, prefixes), config defaults, /health, /index integration.
- `search-service` — config, /health, dense parser (4 shapes), rerank parser (4 shapes + graceful fallback), Qdrant payload extraction, pipeline orchestration with stub VectorStore.
- `mocks` — deterministic hash_embedding.

External HTTP is mocked via `wiremock`; Qdrant is abstracted behind the `VectorStore` trait and injected with an in-memory stub.

## Project layout

```
shared/              — DTOs (Question, Chat, Message, ...)
index-service/       — chunker + handlers + main
search-service/      — retrieval (dense/sparse/qdrant/rerank) + pipeline + handlers + main
mocks/               — dev-only mock_dense (:9001) + mock_rerank (:9002)
scripts/seed/        — init_qdrant.sh + seed.py (local seeding)
frontend/            — SvelteKit UI, proxies /api/search to search-service
docs/superpowers/    — design spec + implementation plan
```
