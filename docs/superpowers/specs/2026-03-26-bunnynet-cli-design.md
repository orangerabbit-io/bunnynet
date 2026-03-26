# bunnynet CLI — Phase 1 Design Spec

Rust CLI and client library for the Bunny.net Core Platform API, following the same architecture as forwardemail, porkbun, and updown.

## Phasing

- **Phase 1 (this spec):** Core API — Pull Zones, Storage Zones, DNS Zones, Video Libraries, Billing, Statistics, Purge, Regions, Search, API Keys (64 paths, 110 schemas)
- **Phase 2:** Shield (WAF/DDoS/rate limiting) + Compute (edge scripts)
- **Phase 3:** Edge Storage file ops, Stream Video, Database, Magic Containers

Each phase adds new cmd + model modules to the existing workspace. No refactoring required between phases.

## Project Structure

Workspace with two crates: `bunnynet-lib` (library) and `bunnynet` (CLI binary).

```
bunnynet/
├── bunnynet/                     # Binary crate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── output.rs
│   │   └── cmd/
│   │       ├── mod.rs
│   │       ├── api_key.rs
│   │       ├── billing.rs
│   │       ├── dns_zone.rs
│   │       ├── pull_zone.rs
│   │       ├── purge.rs
│   │       ├── region.rs
│   │       ├── search.rs
│   │       ├── statistics.rs
│   │       ├── storage_zone.rs
│   │       └── video_library.rs
│   └── tests/
│       ├── common/mod.rs
│       └── fixtures/
├── bunnynet-lib/                 # Library crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── client.rs
│       ├── config.rs
│       └── models/
│           ├── mod.rs
│           ├── api_key.rs
│           ├── billing.rs
│           ├── dns_zone.rs
│           ├── dns_record.rs
│           ├── edge_rule.rs
│           ├── pagination.rs
│           ├── pull_zone.rs
│           ├── region.rs
│           ├── search.rs
│           ├── statistics.rs
│           ├── storage_zone.rs
│           └── video_library.rs
├── Cargo.toml                    # Workspace root
├── Cargo.lock
├── flake.nix
├── .releaserc.json
├── package.json
├── .github/workflows/release.yml
├── LICENSE-APACHE
├── LICENSE-MIT
└── README.md
```

## Configuration

Three-tier priority: CLI flag > environment variable > config file.

**Config file** (`~/.config/bunnynet/config.toml`):
```toml
api_key = "your-account-api-key"
stream_api_key = "your-stream-library-key"
storage_password = "your-storage-zone-password"
```

Only `api_key` is required for Phase 1. `stream_api_key` and `storage_password` are optional fields parsed if present, used in Phase 3.

**Environment variables:**
- `BUNNYNET_API_KEY` — account API key
- `BUNNYNET_STREAM_API_KEY` — stream library key (Phase 3)
- `BUNNYNET_STORAGE_PASSWORD` — storage zone password (Phase 3)
- `BUNNYNET_BASE_URL` — override base URL for testing (default: `https://api.bunny.net`)

**Config struct:**
```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    #[serde(default)]
    pub stream_api_key: Option<String>,
    #[serde(default)]
    pub storage_password: Option<String>,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}
```

`Config::load(api_key_override: Option<&str>) -> Result<Self>` checks flag, then env, then file. Error message on missing key directs user to create the config file or set the env var.

## Client

Single blocking HTTP client for the core API. Phase 2-3 will add separate client types alongside this one (e.g., `StorageClient`).

```rust
pub struct Client {
    http: reqwest::blocking::Client,
    base_url: String,
    api_key: String,
}
```

**Authentication:** `AccessKey` header on every request.

**Methods:**
- `get(path) -> Result<Response>`
- `get_with_params(path, params: &[(&str, &str)]) -> Result<Response>`
- `get_json<T: DeserializeOwned>(path) -> Result<T>`
- `get_json_with_params<T: DeserializeOwned>(path, params) -> Result<T>`
- `post(path, body: &HashMap<String, Value>) -> Result<Response>`
- `put(path, body: &HashMap<String, Value>) -> Result<Response>`
- `delete(path) -> Result<Response>`
- `delete_with_body(path, body) -> Result<Response>`

All methods set `AccessKey` and `Accept-Encoding: gzip` headers.

**Error handling:**
```rust
fn check_status(resp: Response) -> Result<Response> {
    // 400 → "Bad request (HTTP 400): {body}"
    // 401 → "Authentication failed (HTTP 401): {body}"
    // 404 → "Not found (HTTP 404): {body}"
    // 429 → "Rate limited (HTTP 429): {body}"
    // _   → "API error (HTTP {status}): {body}"
}
```

Exit code 2 for config errors, 1 for API errors. `anyhow::Result<T>` throughout.

## Models

Each resource has three components in its model file:

1. **API struct** — deserializes from Bunny's PascalCase JSON
2. **Row struct** — `#[derive(Tabled)]` for table display with human-readable column names
3. **From impl** — converts API struct to Row

**PascalCase handling:** `#[serde(rename_all = "PascalCase")]` on every API struct, since Bunny returns PascalCase keys.

```rust
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorageZone {
    pub id: i64,
    pub name: Option<String>,
    pub storage_used: Option<i64>,
    pub files_stored: Option<i64>,
    pub region: Option<String>,
    pub storage_hostname: Option<String>,
    // ...
}
```

**Large models** (PullZone: 180+ fields, VideoLibrary: 90+ fields): All fields `Option<T>` except `id`. Row types surface only the most useful columns. Full data available via `--json`.

**Integer enums:** Bunny uses integer enums throughout. Mapped to Rust enums:
```rust
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum DnsRecordType {
    A = 0,
    AAAA = 1,
    CNAME = 2,
    TXT = 3,
    MX = 4,
    Redirect = 5,
    Flatten = 6,
    PullZone = 7,
    NS = 8,
    SRV = 9,
    CAA = 10,
    PTR = 11,
    Script = 12,
    NAPTR = 13,
    SSHFP = 14,
    TLSA = 15,
}
```

Custom `Deserialize`/`Serialize` impls to handle integer ↔ enum conversion. `Display` impl for human-readable names in table output.

**Generic pagination wrapper:**
```rust
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaginatedList<T> {
    pub items: Vec<T>,
    pub current_page: i32,
    pub total_items: i32,
    pub has_more_items: bool,
}
```

Used by all list endpoints that return paginated results.

## Commands

### Top-level CLI

```rust
#[derive(Parser)]
#[command(name = "bunnynet", about = "CLI for the Bunny.net API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true)]
    pub json: bool,

    #[arg(long, global = true)]
    pub api_key: Option<String>,
}
```

### Command mapping (10 top-level subcommands)

**Simple resources (flat enum):**

| Command | Subcommands |
|---------|-------------|
| `api-key` | `list` |
| `billing` | `get`, `summary`, `affiliate`, `invoice <ID>`, `payment-requests` |
| `purge` | `url <URL> [--async] [--exact-path]` |
| `region` | `list` |
| `search` | `<QUERY> [--from] [--size]` |
| `statistics` | `[--date-from] [--date-to] [--pull-zone] [--hourly] ...` |

**Storage zones (flat enum with actions):**

```
storage-zone list [--search] [--page] [--per-page] [--include-deleted]
storage-zone get <ID>
storage-zone create <NAME> --region <REGION> [--replication-regions] [--zone-tier]
storage-zone update <ID> [--origin-url] [--custom-404-file-path] [--rewrite-404-to-200]
storage-zone delete <ID> [--delete-linked-pull-zones]
storage-zone reset-password <ID>
storage-zone reset-read-only-password <ID>
storage-zone statistics <ID> [--date-from] [--date-to]
storage-zone check-availability <NAME>
```

**DNS zones (nested — zone ops + record + dnssec subcommands):**

```
dns-zone list [--search] [--page] [--per-page]
dns-zone get <ID>
dns-zone create <DOMAIN>
dns-zone update <ID> [--soa-email] [--logging-enabled] [--nameserver1] [--nameserver2]
dns-zone delete <ID>
dns-zone export <ID>
dns-zone import <ID> --file <PATH>
dns-zone statistics <ID> [--date-from] [--date-to]
dns-zone check-availability <NAME>
dns-zone dnssec enable <ID>
dns-zone dnssec disable <ID>
dns-zone record add <ZONE_ID> --type <TYPE> --name <NAME> --value <VALUE> [--ttl] [--priority] [--weight] [--port]
dns-zone record update <ZONE_ID> <RECORD_ID> [--type] [--name] [--value] [--ttl] ...
dns-zone record delete <ZONE_ID> <RECORD_ID>
dns-zone record scan <ZONE_ID>
dns-zone record scan-results <ZONE_ID>
dns-zone certificate issue <ZONE_ID> [--domain]
```

**Pull zones (deepest — zone ops + hostname + certificate + edge-rule + referrer + blocked-ip + stats):**

```
pull-zone list [--search] [--page] [--per-page] [--include-certificate]
pull-zone get <ID>
pull-zone create <NAME> [--origin-url] [--origin-type] [--storage-zone-id] ...
pull-zone update <ID> [--origin-url] [--cache-control-max-age-override] ...
pull-zone delete <ID>
pull-zone purge-cache <ID> [--cache-tag]
pull-zone check-availability <NAME>

pull-zone hostname add <ID> --hostname <HOST>
pull-zone hostname remove <ID> --hostname <HOST>
pull-zone hostname set-force-ssl <ID> --hostname <HOST> --force-ssl <BOOL>
pull-zone hostname set-private-key-type <ID> --hostname <HOST> --key-type <TYPE>

pull-zone certificate add <ID> --hostname <HOST> --certificate <CERT> --certificate-key <KEY>
pull-zone certificate remove <ID> --hostname <HOST>
pull-zone certificate load-free <HOSTNAME>

pull-zone edge-rule add-or-update <ID> --description <DESC> [--action-type] [--triggers JSON]
pull-zone edge-rule delete <ZONE_ID> <RULE_ID>
pull-zone edge-rule set-enabled <ZONE_ID> <RULE_ID> --enabled <BOOL>

pull-zone referrer add-allowed <ID> --hostname <HOST>
pull-zone referrer remove-allowed <ID> --hostname <HOST>
pull-zone referrer add-blocked <ID> --hostname <HOST>
pull-zone referrer remove-blocked <ID> --hostname <HOST>

pull-zone blocked-ip add <ID> --ip <IP>
pull-zone blocked-ip remove <ID> --ip <IP>

pull-zone reset-security-key <ID>
pull-zone optimizer-statistics <ID> [--date-from] [--date-to] [--hourly]
pull-zone origin-shield-statistics <ID> [--date-from] [--date-to] [--hourly]
pull-zone safehop-statistics <ID> [--date-from] [--date-to] [--hourly]
```

**Video libraries (CRUD + watermark + referrer + api-key ops):**

```
video-library list [--search] [--page] [--per-page]
video-library get <ID>
video-library create <NAME> [--replication-regions] [--player-version]
video-library update <ID> [50+ optional settings flags]
video-library delete <ID>
video-library languages
video-library add-allowed-referrer <ID> --hostname <HOST>
video-library remove-allowed-referrer <ID> --hostname <HOST>
video-library add-blocked-referrer <ID> --hostname <HOST>
video-library remove-blocked-referrer <ID> --hostname <HOST>
video-library reset-api-key <ID>
video-library reset-read-only-api-key <ID>
video-library watermark add <ID> --file <PATH>
video-library watermark delete <ID>
video-library live-thumbnail add <ID> --file <PATH>
video-library live-thumbnail delete <ID>
video-library live-watermark add <ID> --file <PATH>
video-library live-watermark delete <ID>
video-library drm-statistics <ID> [--date-from] [--date-to]
video-library transcribing-statistics <ID> [--date-from] [--date-to]
```

### Nested subcommand implementation

`dns-zone` and `pull-zone` use clap nested subcommands:
```rust
#[derive(Subcommand)]
pub enum DnsZoneAction {
    List { ... },
    Get { id: i64 },
    Create { domain: String },
    // ...
    Record {
        #[command(subcommand)]
        action: DnsRecordAction,
    },
    Dnssec {
        #[command(subcommand)]
        action: DnssecAction,
    },
}

#[derive(Subcommand)]
pub enum DnsRecordAction {
    Add { zone_id: i64, ... },
    Update { zone_id: i64, record_id: i64, ... },
    Delete { zone_id: i64, record_id: i64 },
    Scan { zone_id: i64 },
    ScanResults { zone_id: i64 },
}
```

Each command handler function: `pub fn run(action, &Client, OutputMode) -> Result<()>`.

## Output

Same module as the other tools:

- `OutputMode::Table` (default): markdown tables via `tabled`, key-value pairs for `get`, confirmation messages for mutations
- `OutputMode::Json` (`--json`): `serde_json::to_string_pretty` of the full API response
- `print_table(&[T])`, `print_json(&Value)`, `print_kv(&[(&str, String)])`, `print_confirm(&str)`
- `print_pagination(current_page, total_items, has_more_items)` — derived from `PaginatedList` fields (response body, not headers)

## Testing

### Model unit tests (bunnynet-lib)

Deserialize fixture JSON into structs. Verify PascalCase field mapping. One test per model.

### Integration tests (bunnynet/tests/)

- mockito `Server` for HTTP mocking
- `assert_cmd::Command` for CLI binary invocation
- `predicates` for output assertions
- Fixtures in `tests/fixtures/{resource}_{operation}.json`
- Common helper: `tests/common/mod.rs` with `binary()` and `fixture(name)` functions

Test coverage per resource:
- Table output renders expected columns
- JSON output passes through full response
- `AccessKey` header sent correctly
- Error responses (401, 404) produce correct stderr and exit codes
- Pagination line appears for list commands
- Required parameters enforced

### Live tests (optional)

Gated by `BUNNYNET_LIVE_TEST=1`. Serial execution. Create/read/delete cycle with cleanup. Use disposable resource names to avoid collisions.

## CI/CD

### semantic-release (`.releaserc.json`)

Conventional commits trigger releases:
- `fix:` → patch
- `feat:` → minor
- `BREAKING CHANGE:` → major

Plugins: commit-analyzer, release-notes-generator, changelog, exec (version bump in both Cargo.toml files + flake.nix), git (commit assets), github (create release).

### GitHub Actions (`.github/workflows/release.yml`)

Trigger on push to `main`. Steps: checkout, Node 22, Rust toolchain, npm install, npx semantic-release.

### Nix flake (`flake.nix`)

`rustPlatform.buildRustPackage` for the package. devShell with rustc, cargo, clippy, rustfmt, pkg-config, openssl.

## Dependencies

### bunnynet-lib
- `anyhow = "1"`
- `reqwest = { version = "0.12", features = ["blocking", "json", "gzip"] }`
- `serde = { version = "1", features = ["derive"] }`
- `serde_json = "1"`
- `tabled = "0.17"`
- `toml = "0.8"`
- dev: `serial_test = "3"`

### bunnynet
- `bunnynet-lib = { path = "../bunnynet-lib" }`
- `anyhow = "1"`
- `clap = { version = "4", features = ["derive"] }`
- `serde_json = "1"`
- `tabled = "0.17"`
- dev: `mockito = "1"`, `assert_cmd = "2"`, `predicates = "3"`, `serial_test = "3"`

## License

Dual-licensed: Apache 2.0 OR MIT. `LICENSE-APACHE` + `LICENSE-MIT` files.
