import {json} from "@sveltejs/kit"
import type {RequestHandler} from "./$types"
import {env} from "$env/dynamic/private"

const SEARCH_API_URL = env.PUBLIC_SEARCH_API_URL ?? "http://search-service:8080"

export const GET: RequestHandler = async ({url}) => {
  const query = url.searchParams.get("q") ?? ""

  const payload = {
    question: {
      text: query,
      asker: "",
      asked_on: "",
      variants: [],
      hyde: [],
      keywords: [],
      entities: {people: [], emails: [], documents: [], names: [], links: []},
      date_mentions: [],
      date_range: null,
      search_text: query,
    },
  }

  try {
    const res = await fetch(`${SEARCH_API_URL}/search`, {
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify(payload),
    })
    if (!res.ok) {
      return json({error: `search ${res.status}`, results: []}, {status: res.status})
    }
    const data = await res.json()
    const messageIds: string[] = data?.results?.[0]?.message_ids ?? []
    const items = messageIds.map((id) => ({
      id,
      type: "message",
      title: `Message ${id}`,
      description: "",
      metadata: {},
    }))
    return json(items)
  } catch (err) {
    return json({error: String(err), results: []}, {status: 502})
  }
}
