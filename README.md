# bunnynet

Command-line interface and Rust client library for the [Bunny.net](https://bunny.net) CDN API.

## Install

### From source

```sh
cargo install --path bunnynet
```

### With Nix

```sh
# Install to profile
nix profile install .

# Or enter a dev shell with the binary available
nix develop
```

## Configuration

bunnynet looks for an API key in three places (highest priority first):

1. `--api-key` CLI flag
2. `BUNNYNET_API_KEY` environment variable
3. Config file at `~/.config/bunnynet/config.toml`

### Config file format

```toml
api_key = "your-api-key-here"

# Optional: for stream/video library operations
stream_api_key = "your-stream-api-key"

# Optional: for storage zone operations
storage_password = "your-storage-password"
```

## Usage

Default output is a formatted table. Pass `--json` for JSON output.

### Regions & Countries

```sh
bunnynet region list
bunnynet country list
bunnynet country list --json
```

### DNS Zones

```sh
bunnynet dns-zone list
bunnynet dns-zone create example.com
bunnynet dns-zone get 12345
bunnynet dns-zone record add 12345 --type A --name www --value 1.2.3.4 --ttl 300
bunnynet dns-zone dnssec enable 12345
```

### Pull Zones

```sh
bunnynet pull-zone list
bunnynet pull-zone create my-zone --origin-url https://origin.example.com
bunnynet pull-zone get 12345
bunnynet pull-zone purge-cache 12345
bunnynet pull-zone hostname add 12345 --hostname cdn.example.com
```

### Storage Zones

```sh
bunnynet storage-zone list
bunnynet storage-zone create my-storage --region DE
bunnynet storage-zone get 12345
bunnynet storage-zone statistics 12345
```

### Video Libraries

```sh
bunnynet video-library list
bunnynet video-library create my-library
bunnynet video-library get 12345
bunnynet video-library languages
```

### Billing

```sh
bunnynet billing get
bunnynet billing summary
bunnynet billing affiliate
```

### Other Commands

```sh
bunnynet api-key list
bunnynet purge url https://cdn.example.com/image.png
bunnynet search "my query"
bunnynet statistics --date-from 2025-01-01 --date-to 2025-01-31
```

## Output

- **Table** (default): Formatted markdown-style tables for terminal viewing
- **JSON** (`--json`): Raw JSON output, suitable for piping to `jq` or scripting

```sh
# Table output (default)
bunnynet region list

# JSON output
bunnynet region list --json
```

## API Coverage

| Command | Subcommands |
|---------|-------------|
| `region` | `list` |
| `country` | `list` |
| `api-key` | `list` |
| `purge` | `url` |
| `search` | *(direct command)* |
| `statistics` | *(direct command)* |
| `billing` | `get`, `summary`, `affiliate`, `payment-requests`, `download-invoice`, `download-summary` |
| `storage-zone` | `list`, `get`, `create`, `update`, `delete`, `check-availability`, `reset-password`, `reset-read-only-password`, `statistics` |
| `dns-zone` | `list`, `get`, `create`, `update`, `delete`, `export`, `import`, `statistics`, `check-availability`, `record` (add/update/delete/scan/scan-results), `dnssec` (enable/disable), `certificate` (issue) |
| `pull-zone` | `list`, `get`, `create`, `update`, `delete`, `purge-cache`, `check-availability`, `reset-security-key`, `optimizer-statistics`, `origin-shield-statistics`, `safehop-statistics`, `hostname` (add/remove/set-force-ssl), `certificate` (add/remove), `edge-rule` (add-or-update/delete/set-enabled), `referrer` (add-allowed/remove-allowed/add-blocked/remove-blocked), `blocked-ip` (add/remove) |
| `video-library` | `list`, `get`, `create`, `update`, `delete`, `languages`, `add-allowed-referrer`, `remove-allowed-referrer`, `add-blocked-referrer`, `remove-blocked-referrer`, `reset-api-key`, `reset-read-only-api-key`, `drm-statistics`, `transcribing-statistics`, `watermark` (add/delete), `live-thumbnail` (add/delete), `live-watermark` (add/delete) |

## Phase 1 Scope

### Covered

- All account-level API endpoints: regions, countries, API keys, search, statistics, billing, purge
- Full CRUD for storage zones, DNS zones, pull zones, and video libraries
- DNS record management (add, update, delete, scan)
- Pull zone sub-resources: hostnames, certificates, edge rules, referrers, blocked IPs
- Video library sub-resources: watermarks, live thumbnails, live watermarks, referrers
- DNS zone export/import, DNSSEC, certificate issuance
- Paginated list endpoints with proper `PaginatedList` handling

### Coming in Phase 2/3

- Stream video management (collections, videos, upload/encode)
- Edge storage file operations (upload, download, list, delete)
- Shield zones
- Compute (edge scripting)
- Abuse case management
- Interactive prompts and richer TUI output

## Development

```sh
# Enter dev shell (provides Rust toolchain)
nix develop

# Build
cargo build --workspace

# Test
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt --all --check

# Build docs
cargo doc --workspace --no-deps
```

## License

MIT OR Apache-2.0
