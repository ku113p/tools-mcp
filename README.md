# Tools MCP

MCP server (Model Context Protocol) aggregating short-links, landing-pages, and message services.

- **Transport:** Streamable HTTP (`POST /mcp`)
- **Protocol:** MCP 2025-03-26

## Tools

| Tool | Parameters | Description |
|------|-----------|-------------|
| `create_short_link` | `url`, `name?` | Create a short link |
| `list_short_links` | — | List all links with click counts |
| `create_landing_page` | `path`, `html`, `name?` | Create an HTML page |
| `list_landing_pages` | — | List all pages |
| `get_landing_page` | `path` | Get page HTML content |
| `create_topic` | `name`, `tg_api_key?`, `tg_chat_id?` | Create a message topic |
| `list_topics` | — | List all topics |
| `list_messages` | `topic_id` | List messages for a topic |

## Auth

Bearer token via `Authorization` header, checked against `MCP_AUTH_TOKEN`. If `MCP_AUTH_TOKEN` is not set, the endpoint is open.

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `8080` | Bind port |
| `MCP_AUTH_TOKEN` | — | Bearer token for the MCP endpoint |
| `SHORT_LINKS_URL` | `http://short-links-app:3000` | Short-links API base URL |
| `SHORT_LINKS_TOKEN` | — | Auth token for short-links API |
| `LANDING_PAGES_URL` | `http://landing-pages-app:3000` | Landing-pages API base URL |
| `LANDING_PAGES_TOKEN` | — | Auth token for landing-pages API |
| `MESSAGE_URL` | `http://message-app:3000` | Message API base URL |
| `MESSAGE_TOKEN` | — | Auth token for message API |

## Claude Code config

Add to `.mcp.json`:

```json
{
  "mcpServers": {
    "tools": {
      "type": "streamable-http",
      "url": "https://mcp.tools.syncapp.tech/mcp",
      "headers": {
        "Authorization": "Bearer <MCP_AUTH_TOKEN>"
      }
    }
  }
}
```

## Running

```bash
cargo run
```

Docker:

```bash
docker run -e PORT=8080 ghcr.io/ku113p/tools-mcp:latest
```
