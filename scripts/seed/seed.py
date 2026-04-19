#!/usr/bin/env python3
"""Seed test data into Qdrant via the index-service + mock-dense.

Reads test_index.json, POSTs to index-service/index, computes dense (mock) and
sparse (index-service) embeddings per chunk, upserts points to Qdrant.
"""
import json
import os
import sys
import urllib.request
import urllib.error


def http_post_json(url: str, payload: dict, headers: dict | None = None) -> dict:
    data = json.dumps(payload).encode("utf-8")
    hdrs = {"Content-Type": "application/json"}
    if headers:
        hdrs.update(headers)
    req = urllib.request.Request(url, data=data, headers=hdrs, method="POST")
    with urllib.request.urlopen(req, timeout=60) as resp:
        return json.loads(resp.read())


def http_put_json(url: str, payload: dict) -> dict:
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(
        url,
        data=data,
        headers={"Content-Type": "application/json"},
        method="PUT",
    )
    with urllib.request.urlopen(req, timeout=60) as resp:
        return json.loads(resp.read())


def main() -> int:
    qdrant_url = os.environ.get("QDRANT_URL", "http://qdrant:6333")
    collection = os.environ.get("COLLECTION", "messages")
    index_url = os.environ.get("INDEX_SERVICE_URL", "http://index-service:8080")
    dense_url = os.environ.get("DENSE_URL", "http://mock-dense:9001/embeddings")
    input_path = os.environ.get("INPUT_JSON", "/scripts/test_index.json")

    with open(input_path, "r", encoding="utf-8") as f:
        index_req = json.load(f)

    print(f"POST {index_url}/index ...", flush=True)
    chunks_resp = http_post_json(f"{index_url}/index", index_req)
    chunks = chunks_resp.get("results", [])
    print(f"received {len(chunks)} chunks", flush=True)

    for i, c in enumerate(chunks):
        dense = http_post_json(dense_url, {"input": c["dense_content"]})
        dense_vec = dense["data"][0]["embedding"]

        sparse = http_post_json(
            f"{index_url}/sparse_embedding",
            {"texts": [c["sparse_content"]]},
        )
        sv = sparse["vectors"][0]

        point = {
            "points": [
                {
                    "id": i,
                    "vector": {
                        "dense": dense_vec,
                        "sparse": {"indices": sv["indices"], "values": sv["values"]},
                    },
                    "payload": {
                        "page_content": c["page_content"],
                        "message_ids": c["message_ids"],
                    },
                }
            ]
        }
        http_put_json(
            f"{qdrant_url}/collections/{collection}/points?wait=true",
            point,
        )
        print(f"  upserted point {i} with {len(c['message_ids'])} message_ids", flush=True)

    print("seeding complete", flush=True)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code} from {e.url}: {e.read().decode('utf-8', 'ignore')}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"error: {e}", file=sys.stderr)
        sys.exit(1)
