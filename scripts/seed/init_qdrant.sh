#!/bin/sh
set -e

QDRANT="${QDRANT_URL:-http://qdrant:6333}"
COLLECTION="${COLLECTION:-messages}"

for i in $(seq 1 30); do
    if curl -sf "$QDRANT/readyz" > /dev/null 2>&1; then
        break
    fi
    echo "waiting for qdrant... ($i)"
    sleep 1
done

curl -sf -X DELETE "$QDRANT/collections/$COLLECTION" > /dev/null || true

curl -sf -X PUT "$QDRANT/collections/$COLLECTION" \
    -H "Content-Type: application/json" \
    -d '{
        "vectors": {
            "dense": { "size": 1024, "distance": "Cosine" }
        },
        "sparse_vectors": {
            "sparse": {}
        }
    }'

echo ""
echo "collection $COLLECTION created"
