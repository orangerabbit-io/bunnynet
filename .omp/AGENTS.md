# bunnynet

Rust CLI and client library for the Bunny.net CDN API.

## Build & Test

- `cargo build --workspace` — build all crates
- `cargo test --workspace` — run all tests (unit + integration with mockito)
- `cargo clippy --workspace -- -D warnings` — lint (must be clean)
- `cargo fmt --all --check` — format check
- `cargo doc --workspace --no-deps` — build docs (must be warning-free)
- `nix build` — build via flake (output in `./result/bin/bunnynet`)
- `nix profile install .` — install locally via Nix

## Architecture

Two-crate Cargo workspace. Blocking reqwest (no async). Clap derive for CLI.

### Library: `bunnynet-lib`
- `src/config.rs` — 3-tier config: `--api-key` flag > `BUNNYNET_API_KEY` env > `~/.config/bunnynet/config.toml`
- `src/client.rs` — HTTP client with `AccessKey` header auth
- `src/models/` — API response types + tabled Row types

### CLI: `bunnynet`
- `src/main.rs` — Clap CLI parser, dispatch, error handling
- `src/output.rs` — Table (default, markdown style), JSON (`--json` flag), key-value, and confirmation output
- `src/cmd/` — One file per resource (11 resource modules: api_key, billing, country, dns_zone, pull_zone, purge, region, search, statistics, storage_zone, video_library)
- `tests/` — Integration tests using mockito mock server + assert_cmd

## Testing

- Integration tests use `BUNNYNET_BASE_URL` env var to point at mockito server
- Config tests that mutate env vars must use `#[serial]` from serial_test
- Tests use `assert_cmd` for CLI binary invocation and `predicates` for output assertions

## Gotchas

- reqwest `gzip` feature MUST be enabled — the API responds with gzip-compressed bodies. Without the feature, response parsing fails with "expected value at line 1 column 1"
- PascalCase JSON keys — use `#[serde(rename_all = "PascalCase")]` with explicit `#[serde(rename = "...")]` for acronyms (SSL, DNS, IP, API, DRM, CDN, etc.) since `PascalCase` rename does not handle consecutive uppercase correctly
- Integer enums via `serde_repr` — `DnsRecordType`, `PullZoneOriginType`, and similar use `#[repr(i32)]` with `serde_repr::Serialize_repr`/`Deserialize_repr`
- `AccessKey` header — not Bearer or Basic auth. The client sets `AccessKey: <api-key>` on every request
- Pagination quirk — pull zone, video library, and storage zone list endpoints: `page=0` returns a plain JSON array, `page>=1` returns a `PaginatedList` wrapper object with `Items`, `TotalItems`, `CurrentPage`, `HasMoreItems`. Always default `page` to 1
- `EnviromentalVariables` is NOT a typo — Bunny API spells it that way in the pull zone model
- POST for updates (not PUT) — most update endpoints use POST, but DNS record add uses PUT
- DNS record scan uses `POST /dnszone/scan/trigger` (domain-based) or `POST /dnszone/{id}/scan/trigger` (zone-based)
- Video library binary uploads (watermark, live thumbnail, live watermark) send raw bytes with custom Content-Type, not JSON
- `base64` crate used in CLI for certificate file encoding (pull zone custom certificates)

## Publishing

- License: MIT OR Apache-2.0 (LICENSE-MIT, LICENSE-APACHE)
- Cargo.toml publish metadata: done (description, license, repository, keywords, categories, readme)
- Nix flake: `packages.default` via `rustPlatform.buildRustPackage`

### GitHub
- Repo: orangerabbit-io/bunnynet
- CI: semantic-release on push to main (`.releaserc.json`)
