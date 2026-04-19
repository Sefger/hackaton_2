#!/bin/sh
set -e

# Entry point: waits for index-service, then runs seed.py.
# Called from docker-compose seed-data service (python:3.12-alpine).

INDEX_SERVICE="${INDEX_SERVICE_URL:-http://index-service:8080}"

echo "waiting for index-service at $INDEX_SERVICE ..."
for i in $(seq 1 60); do
    if wget -q -O - "$INDEX_SERVICE/health" > /dev/null 2>&1; then
        echo "index-service is ready"
        break
    fi
    sleep 1
done

exec python3 /scripts/seed.py
