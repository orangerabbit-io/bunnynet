# bunnynet Phase 1 Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust CLI and client library for the Bunny.net Core Platform API with full Phase 1 endpoint coverage.

**Architecture:** Two-crate Cargo workspace (`bunnynet-lib` + `bunnynet`). Blocking reqwest with `AccessKey` header auth. Clap derive for CLI. Table (default) or JSON (`--json`) output. Three-tier config: CLI flag > env var > config file.

**Tech Stack:** Rust (2021 edition), reqwest 0.12 (blocking), clap 4 (derive), serde + serde_repr, tabled 0.17, anyhow, mockito + assert_cmd for tests, semantic-release for CI/CD.

**Spec:** `docs/superpowers/specs/2026-03-26-bunnynet-cli-design.md`

**OpenAPI source of truth:** `openapi-specs/bunnynet-api.json` — all model fields, enum values, and endpoint parameters are defined here. When the plan says "derive fields from OpenAPI spec schema `XxxModel`", read that schema from the spec file.

**Reference project:** The `updown` sibling project at `/home/alindsay/projects/orangerabbit-io/updown/` uses the identical architecture. When in doubt about patterns, check that project.

**Key API conventions:**
- Bunny.net uses **POST for updates**, not PUT. Every `update` command maps to `POST /resource/{id}`.
- All integer enums use `serde_repr` for serialization. When deriving models from the OpenAPI spec, check for all `enum` types and create corresponding Rust enums with `Deserialize_repr`/`Serialize_repr`.
- Hostname/referrer removal endpoints use **DELETE with a JSON body** (`delete_with_body`), not a bodyless DELETE.
- Several `list` endpoints (pull zones, video libraries, storage zones) return a plain array when `page=0` but `PaginatedList<T>` when `page>=1`. **Always default `--page` to 1.** DNS zones and API keys default to `page=1` natively, so no workaround needed for those.

---

## Chunk 1: Project Scaffold & Foundation

### Task 1: Workspace scaffold

Create all project infrastructure files. No Rust code yet — just the workspace and build/CI configuration.

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `bunnynet-lib/Cargo.toml`
- Create: `bunnynet/Cargo.toml`
- Create: `flake.nix`
- Create: `.releaserc.json`
- Create: `package.json`
- Create: `.github/workflows/release.yml`
- Create: `LICENSE-APACHE`
- Create: `LICENSE-MIT`
- Create: `.gitignore`

- [ ] **Step 1: Create workspace root Cargo.toml**

```toml
[workspace]
members = ["bunnynet-lib", "bunnynet"]
resolver = "2"
```

- [ ] **Step 2: Create bunnynet-lib/Cargo.toml**

```toml
[package]
name = "bunnynet-lib"
version = "0.1.0"
edition = "2021"
description = "Rust client library for the Bunny.net API"
license = "MIT OR Apache-2.0"
repository = "https://github.com/orangerabbit-io/bunnynet"
homepage = "https://github.com/orangerabbit-io/bunnynet"
readme = "../README.md"
keywords = ["bunnynet", "bunny", "cdn", "api", "client"]
categories = ["api-bindings"]

[dependencies]
anyhow = "1"
reqwest = { version = "0.12", features = ["blocking", "json", "gzip"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_repr = "0.1"
tabled = "0.17"
toml = "0.8"

[dev-dependencies]
serial_test = "3"
```

- [ ] **Step 3: Create bunnynet/Cargo.toml**

```toml
[package]
name = "bunnynet"
version = "0.1.0"
edition = "2021"
description = "Command-line interface for the Bunny.net API"
license = "MIT OR Apache-2.0"
repository = "https://github.com/orangerabbit-io/bunnynet"
homepage = "https://github.com/orangerabbit-io/bunnynet"
readme = "../README.md"
keywords = ["bunnynet", "bunny", "cdn", "cli"]
categories = ["command-line-utilities"]

[dependencies]
bunnynet-lib = { path = "../bunnynet-lib" }
anyhow = "1"
clap = { version = "4", features = ["derive"] }
serde_json = "1"
tabled = "0.17"

[dev-dependencies]
mockito = "1"
assert_cmd = "2"
predicates = "3"
serial_test = "3"
```

- [ ] **Step 4: Create flake.nix**

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "bunnynet";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
            pkg-config
            openssl
          ];
        };
      });
}
```

- [ ] **Step 5: Create .releaserc.json**

```json
{
  "branches": ["main"],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    ["@semantic-release/changelog", {
      "changelogFile": "CHANGELOG.md"
    }],
    ["@semantic-release/exec", {
      "prepareCmd": "sed -i 's/^version = .*/version = \"${nextRelease.version}\"/' bunnynet/Cargo.toml bunnynet-lib/Cargo.toml && sed -i 's/version = \"[0-9]*\\.[0-9]*\\.[0-9]*\";/version = \"${nextRelease.version}\";/' flake.nix && cargo generate-lockfile"
    }],
    ["@semantic-release/git", {
      "assets": ["CHANGELOG.md", "Cargo.lock", "flake.nix", "bunnynet/Cargo.toml", "bunnynet-lib/Cargo.toml"],
      "message": "chore(release): ${nextRelease.version}\n\n${nextRelease.notes}"
    }],
    "@semantic-release/github"
  ]
}
```

- [ ] **Step 6: Create package.json**

```json
{
  "private": true,
  "devDependencies": {
    "semantic-release": "^24",
    "@semantic-release/changelog": "^6",
    "@semantic-release/exec": "^7",
    "@semantic-release/git": "^10",
    "@semantic-release/github": "^11"
  }
}
```

- [ ] **Step 7: Create .github/workflows/release.yml**

```yaml
name: Release

on:
  push:
    branches: [main]

permissions:
  contents: write
  issues: write
  pull-requests: write

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: actions/setup-node@v4
        with:
          node-version: 22
      - uses: dtolnay/rust-toolchain@stable
      - run: npm install
      - run: npx semantic-release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

- [ ] **Step 8: Create LICENSE-APACHE and LICENSE-MIT**

Copy the standard Apache 2.0 and MIT license texts. Use "orangerabbit-io" as the copyright holder and 2026 as the year.

- [ ] **Step 9: Create .gitignore**

```
/target
node_modules
.direnv
.env
```

- [ ] **Step 10: Commit**

```bash
git add Cargo.toml bunnynet-lib/Cargo.toml bunnynet/Cargo.toml flake.nix .releaserc.json package.json .github/ LICENSE-APACHE LICENSE-MIT .gitignore
git commit -m "chore: scaffold workspace with build and CI configuration"
```

---

### Task 2: Config module

**Files:**
- Create: `bunnynet-lib/src/lib.rs`
- Create: `bunnynet-lib/src/config.rs`

- [ ] **Step 1: Create lib.rs**

```rust
pub mod config;
```

- [ ] **Step 2: Write config tests**

```rust
// bunnynet-lib/src/config.rs

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

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

fn default_base_url() -> String {
    "https://api.bunny.net".to_string()
}

impl Config {
    pub fn load(api_key_override: Option<&str>) -> Result<Self> {
        let base_url =
            std::env::var("BUNNYNET_BASE_URL").unwrap_or_else(|_| default_base_url());

        if let Some(key) = api_key_override {
            return Ok(Config {
                api_key: key.to_string(),
                stream_api_key: None,
                storage_password: None,
                base_url,
            });
        }

        if let Ok(key) = std::env::var("BUNNYNET_API_KEY") {
            return Ok(Config {
                api_key: key,
                stream_api_key: std::env::var("BUNNYNET_STREAM_API_KEY").ok(),
                storage_password: std::env::var("BUNNYNET_STORAGE_PASSWORD").ok(),
                base_url,
            });
        }

        let path = Self::config_path()?;
        let contents = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "No API key found. Create a config file at {} with:\n\n  api_key = \"your-api-key\"\n\nOr set BUNNYNET_API_KEY environment variable.",
                path.display()
            )
        })?;

        let mut config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file at {}", path.display()))?;

        if std::env::var("BUNNYNET_BASE_URL").is_ok() {
            config.base_url = base_url;
        }

        Ok(config)
    }

    fn config_path() -> Result<PathBuf> {
        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        Ok(PathBuf::from(home).join(".config/bunnynet/config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_cli_flag_override_takes_priority() {
        let config = Config::load(Some("flag-key")).unwrap();
        assert_eq!(config.api_key, "flag-key");
    }

    #[test]
    #[serial]
    fn test_env_var_override() {
        std::env::set_var("BUNNYNET_API_KEY", "env-key");
        let config = Config::load(None).unwrap();
        assert_eq!(config.api_key, "env-key");
        std::env::remove_var("BUNNYNET_API_KEY");
    }

    #[test]
    #[serial]
    fn test_base_url_env_override() {
        std::env::set_var("BUNNYNET_BASE_URL", "http://localhost:9999");
        let config = Config::load(Some("key")).unwrap();
        assert_eq!(config.base_url, "http://localhost:9999");
        std::env::remove_var("BUNNYNET_BASE_URL");
    }

    #[test]
    #[serial]
    fn test_default_base_url() {
        std::env::remove_var("BUNNYNET_BASE_URL");
        let config = Config::load(Some("key")).unwrap();
        assert_eq!(config.base_url, "https://api.bunny.net");
    }

    #[test]
    #[serial]
    fn test_missing_api_key_errors() {
        std::env::remove_var("BUNNYNET_API_KEY");
        let result = Config::load(None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("No API key found") || err.contains("config"));
    }
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p bunnynet-lib`
Expected: All 5 tests pass.

- [ ] **Step 4: Commit**

```bash
git add bunnynet-lib/src/
git commit -m "feat: add config module with three-tier API key resolution"
```

---

### Task 3: Client module

**Files:**
- Modify: `bunnynet-lib/src/lib.rs` — add `pub mod client;`
- Create: `bunnynet-lib/src/client.rs`

- [ ] **Step 1: Create client.rs**

Follow the updown client pattern exactly, but:
- Use `AccessKey` header instead of `X-API-KEY`
- Add `post_no_body`, `post_with_params`, `get_bytes`, `put_file` methods
- Add `400 → "Bad request"` to `check_status`

```rust
// bunnynet-lib/src/client.rs

use anyhow::{bail, Context, Result};
use reqwest::blocking::{Client as HttpClient, Response};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub struct Client {
    http: HttpClient,
    base_url: String,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String, base_url: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));

        let http = HttpClient::builder()
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Client {
            http,
            base_url,
            api_key,
        })
    }

    pub fn get(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .header("AccessKey", &self.api_key)
            .send()
            .with_context(|| format!("Request failed: GET {}", url))?;
        Self::check_status(resp)
    }

    pub fn get_with_params(&self, path: &str, params: &[(&str, &str)]) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .header("AccessKey", &self.api_key)
            .query(params)
            .send()
            .with_context(|| format!("Request failed: GET {}", url))?;
        Self::check_status(resp)
    }

    #[allow(dead_code)]
    pub fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self.get(path)?;
        resp.json::<T>().context("Failed to parse JSON response")
    }

    #[allow(dead_code)]
    pub fn get_json_with_params<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        let resp = self.get_with_params(path, params)?;
        resp.json::<T>().context("Failed to parse JSON response")
    }

    pub fn get_bytes(&self, path: &str) -> Result<Vec<u8>> {
        let resp = self.get(path)?;
        let bytes = resp.bytes().context("Failed to read response bytes")?;
        Ok(bytes.to_vec())
    }

    pub fn post(&self, path: &str, body: &HashMap<String, serde_json::Value>) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("AccessKey", &self.api_key)
            .json(body)
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;
        Self::check_status(resp)
    }

    pub fn post_no_body(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("AccessKey", &self.api_key)
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;
        Self::check_status(resp)
    }

    pub fn post_text(&self, path: &str, body: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("AccessKey", &self.api_key)
            .header("Content-Type", "text/plain")
            .body(body.to_string())
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;
        Self::check_status(resp)
    }

    pub fn post_with_params(&self, path: &str, params: &[(&str, &str)]) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("AccessKey", &self.api_key)
            .query(params)
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;
        Self::check_status(resp)
    }

    pub fn put(&self, path: &str, body: &HashMap<String, serde_json::Value>) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .put(&url)
            .header("AccessKey", &self.api_key)
            .json(body)
            .send()
            .with_context(|| format!("Request failed: PUT {}", url))?;
        Self::check_status(resp)
    }

    pub fn put_file(&self, path: &str, data: Vec<u8>, content_type: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .put(&url)
            .header("AccessKey", &self.api_key)
            .header("Content-Type", content_type)
            .body(data)
            .send()
            .with_context(|| format!("Request failed: PUT {}", url))?;
        Self::check_status(resp)
    }

    pub fn delete(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .delete(&url)
            .header("AccessKey", &self.api_key)
            .send()
            .with_context(|| format!("Request failed: DELETE {}", url))?;
        Self::check_status(resp)
    }

    pub fn delete_with_body(
        &self,
        path: &str,
        body: &HashMap<String, serde_json::Value>,
    ) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .delete(&url)
            .header("AccessKey", &self.api_key)
            .json(body)
            .send()
            .with_context(|| format!("Request failed: DELETE {}", url))?;
        Self::check_status(resp)
    }

    fn check_status(resp: Response) -> Result<Response> {
        let status = resp.status();
        if status.is_success() {
            return Ok(resp);
        }
        let url = resp.url().to_string();
        let body = resp.text().unwrap_or_default();
        match status.as_u16() {
            400 => bail!("Bad request (HTTP {}): {}", status, body),
            401 | 403 => bail!("Authentication failed (HTTP {}): {}", status, body),
            404 => bail!("Not found (HTTP {}): {}", status, body),
            422 => bail!("Validation error (HTTP {}): {}", status, body),
            429 => bail!("Rate limited (HTTP {}): {}", status, body),
            _ => bail!("API error (HTTP {}) for {}: {}", status, url, body),
        }
    }
}
```

- [ ] **Step 2: Update lib.rs**

```rust
pub mod client;
pub mod config;
```

- [ ] **Step 3: Verify it compiles**

Run: `cargo build -p bunnynet-lib`
Expected: Compiles successfully.

- [ ] **Step 4: Commit**

```bash
git add bunnynet-lib/src/
git commit -m "feat: add HTTP client with AccessKey auth and all method variants"
```

---

### Task 4: Output module

**Files:**
- Create: `bunnynet/src/output.rs`

- [ ] **Step 1: Create output.rs**

```rust
// bunnynet/src/output.rs

use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputMode {
    Table,
    Json,
}

impl OutputMode {
    pub fn from_json_flag(json: bool) -> Self {
        if json {
            OutputMode::Json
        } else {
            OutputMode::Table
        }
    }
}

pub fn print_json(value: &serde_json::Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
    );
}

pub fn print_table<T: Tabled>(items: &[T]) {
    if items.is_empty() {
        println!("No results.");
        return;
    }
    let mut table = Table::new(items);
    table.with(Style::markdown());
    println!("{}", table);
}

pub fn print_kv(pairs: &[(&str, String)]) {
    let max_key_len = pairs.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    for (key, value) in pairs {
        println!("{:>width$}:  {}", key, value, width = max_key_len);
    }
}

pub fn print_confirm(message: &str) {
    println!("{}", message);
}

pub fn print_pagination(current_page: i32, total_items: i32, has_more_items: bool) {
    let suffix = if has_more_items {
        ", more available"
    } else {
        ""
    };
    println!("Page {} ({} total items{})", current_page, total_items, suffix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_mode_from_flag() {
        assert_eq!(OutputMode::from_json_flag(true), OutputMode::Json);
        assert_eq!(OutputMode::from_json_flag(false), OutputMode::Table);
    }
}
```

- [ ] **Step 2: This file will compile as part of Task 5. Move on.**

---

### Task 5: Main.rs skeleton + first compile

**Files:**
- Create: `bunnynet/src/main.rs`
- Create: `bunnynet/src/cmd/mod.rs`
- Create: `bunnynet-lib/src/models/mod.rs`
- Create: `bunnynet/tests/common/mod.rs`

- [ ] **Step 1: Create models/mod.rs (empty for now)**

```rust
// bunnynet-lib/src/models/mod.rs
```

- [ ] **Step 2: Update lib.rs to add models**

```rust
pub mod client;
pub mod config;
pub mod models;
```

- [ ] **Step 3: Create cmd/mod.rs (empty for now)**

```rust
// bunnynet/src/cmd/mod.rs
```

- [ ] **Step 4: Create main.rs with minimal skeleton**

Start with zero commands — just the parser shell that compiles. Commands will be added as resources are implemented.

```rust
// bunnynet/src/main.rs

mod cmd;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process;
use bunnynet_lib::client::Client;
use bunnynet_lib::config::Config;

#[derive(Parser)]
#[command(name = "bunnynet", about = "CLI for the Bunny.net API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Force JSON output
    #[arg(long, global = true)]
    pub json: bool,

    /// API key (overrides config file and env var)
    #[arg(long, global = true)]
    pub api_key: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {:#}", e);

        let exit_code = if format!("{:#}", e).contains("No API key found")
            || format!("{:#}", e).contains("Failed to parse config")
            || format!("{:#}", e).contains("HOME environment variable")
        {
            2
        } else {
            1
        };
        process::exit(exit_code);
    }
}

fn run(cli: Cli) -> Result<()> {
    let config = Config::load(cli.api_key.as_deref())?;
    let client = Client::new(config.api_key, config.base_url)?;
    let mode = output::OutputMode::from_json_flag(cli.json);
    let _ = (&client, mode); // suppress unused warnings until commands are added

    match cli.command {}
}
```

- [ ] **Step 5: Create test helper**

```rust
// bunnynet/tests/common/mod.rs

use std::path::PathBuf;

#[allow(dead_code)]
pub fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Missing fixture: {}", path.display()))
}

pub fn binary() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("bunnynet").unwrap()
}
```

- [ ] **Step 6: Create tests/fixtures/ directory**

```bash
mkdir -p bunnynet/tests/fixtures
```

- [ ] **Step 7: Build the full workspace**

Run: `cargo build --workspace`
Expected: Compiles successfully.

- [ ] **Step 8: Run all tests**

Run: `cargo test --workspace`
Expected: All tests pass (config tests + output test).

- [ ] **Step 9: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: No warnings.

- [ ] **Step 10: Commit**

```bash
git add -A
git commit -m "feat: add main CLI skeleton with output module and test infrastructure"
```

---

## Chunk 2: Simple Resources

Each resource task follows this pattern:
1. Create the model in `bunnynet-lib/src/models/` with unit test
2. Create the command handler in `bunnynet/src/cmd/`
3. Wire into `main.rs` (add Commands variant + dispatch)
4. Create test fixture + integration test
5. Run tests, commit

### Task 6: Pagination model

**Files:**
- Create: `bunnynet-lib/src/models/pagination.rs`
- Modify: `bunnynet-lib/src/models/mod.rs`

- [ ] **Step 1: Create pagination.rs**

```rust
// bunnynet-lib/src/models/pagination.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaginatedList<T> {
    pub items: Vec<T>,
    pub current_page: i32,
    pub total_items: i32,
    pub has_more_items: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_paginated_list() {
        let json = r#"{
            "Items": [1, 2, 3],
            "CurrentPage": 0,
            "TotalItems": 10,
            "HasMoreItems": true
        }"#;
        let list: PaginatedList<i32> = serde_json::from_str(json).unwrap();
        assert_eq!(list.items, vec![1, 2, 3]);
        assert_eq!(list.current_page, 0);
        assert_eq!(list.total_items, 10);
        assert!(list.has_more_items);
    }
}
```

- [ ] **Step 2: Update models/mod.rs**

```rust
pub mod pagination;
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p bunnynet-lib`
Expected: Pass.

- [ ] **Step 4: Commit**

```bash
git add bunnynet-lib/src/models/
git commit -m "feat: add generic PaginatedList model"
```

---

### Task 7: Region resource (canonical template)

This is the simplest resource: one GET endpoint, no pagination, no mutations. It establishes the canonical pattern that all subsequent resources follow.

**Files:**
- Create: `bunnynet-lib/src/models/region.rs`
- Create: `bunnynet/src/cmd/region.rs`
- Create: `bunnynet/tests/fixtures/region_list.json`
- Create: `bunnynet/tests/region_test.rs`
- Modify: `bunnynet-lib/src/models/mod.rs`
- Modify: `bunnynet/src/cmd/mod.rs`
- Modify: `bunnynet/src/main.rs`

- [ ] **Step 1: Create region model with test**

Derive fields from OpenAPI spec schema `RegionModel`: Id (int64), Name (string nullable), PricePerGigabyte (double), RegionCode (string nullable), ContinentCode (string nullable), CountryCode (string nullable), Latitude (double), Longitude (double), AllowLatencyRouting (boolean).

```rust
// bunnynet-lib/src/models/region.rs

use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Region {
    pub id: i64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub price_per_gigabyte: Option<f64>,
    #[serde(default)]
    pub region_code: Option<String>,
    #[serde(default)]
    pub continent_code: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub allow_latency_routing: Option<bool>,
}

#[derive(Debug, Tabled)]
pub struct RegionRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "CODE")]
    pub region_code: String,
    #[tabled(rename = "CONTINENT")]
    pub continent_code: String,
    #[tabled(rename = "PRICE/GB")]
    pub price_per_gigabyte: String,
    #[tabled(rename = "LATENCY ROUTING")]
    pub allow_latency_routing: String,
}

impl From<&Region> for RegionRow {
    fn from(r: &Region) -> Self {
        RegionRow {
            id: r.id.to_string(),
            name: r.name.clone().unwrap_or("-".to_string()),
            region_code: r.region_code.clone().unwrap_or("-".to_string()),
            continent_code: r.continent_code.clone().unwrap_or("-".to_string()),
            price_per_gigabyte: r
                .price_per_gigabyte
                .map(|p| format!("{:.4}", p))
                .unwrap_or("-".to_string()),
            allow_latency_routing: r
                .allow_latency_routing
                .map(|b| b.to_string())
                .unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_region() {
        let json = r#"{
            "Id": 1,
            "Name": "EU: Falkenstein",
            "PricePerGigabyte": 0.01,
            "RegionCode": "DE",
            "ContinentCode": "EU",
            "CountryCode": "DE",
            "Latitude": 50.47,
            "Longitude": 12.37,
            "AllowLatencyRouting": true
        }"#;
        let region: Region = serde_json::from_str(json).unwrap();
        assert_eq!(region.id, 1);
        assert_eq!(region.name, Some("EU: Falkenstein".to_string()));
        assert_eq!(region.region_code, Some("DE".to_string()));
    }
}
```

- [ ] **Step 2: Update models/mod.rs**

```rust
pub mod pagination;
pub mod region;
```

- [ ] **Step 3: Run model test**

Run: `cargo test -p bunnynet-lib models::region`
Expected: Pass.

- [ ] **Step 4: Create region command handler**

```rust
// bunnynet/src/cmd/region.rs

use anyhow::Result;
use clap::Subcommand;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::region::{Region, RegionRow};

#[derive(Subcommand)]
pub enum RegionAction {
    /// List available CDN regions
    List,
}

pub fn run(action: RegionAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        RegionAction::List => list(client, mode),
    }
}

fn list(client: &Client, mode: OutputMode) -> Result<()> {
    let resp = client.get("/region")?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let regions: Vec<Region> = resp.json()?;
            let rows: Vec<RegionRow> = regions.iter().map(RegionRow::from).collect();
            output::print_table(&rows);
        }
    }

    Ok(())
}
```

- [ ] **Step 5: Update cmd/mod.rs**

```rust
pub mod region;
```

- [ ] **Step 6: Wire region into main.rs**

Add to the `Commands` enum:

```rust
/// List CDN regions
Region {
    #[command(subcommand)]
    action: cmd::region::RegionAction,
},
```

Add to the `match cli.command` block:

```rust
Commands::Region { action } => cmd::region::run(action, &client, mode),
```

Remove the `let _ = (&client, mode);` line now that there's a real command.

- [ ] **Step 7: Create test fixture**

```json
// bunnynet/tests/fixtures/region_list.json
[
    {
        "Id": 1,
        "Name": "EU: Falkenstein",
        "PricePerGigabyte": 0.01,
        "RegionCode": "DE",
        "ContinentCode": "EU",
        "CountryCode": "DE",
        "Latitude": 50.47,
        "Longitude": 12.37,
        "AllowLatencyRouting": true
    },
    {
        "Id": 2,
        "Name": "US: New York",
        "PricePerGigabyte": 0.01,
        "RegionCode": "NY",
        "ContinentCode": "NA",
        "CountryCode": "US",
        "Latitude": 40.71,
        "Longitude": -74.01,
        "AllowLatencyRouting": true
    }
]
```

- [ ] **Step 8: Create integration test**

```rust
// bunnynet/tests/region_test.rs

mod common;

use predicates::prelude::*;

#[test]
fn test_region_list_table() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/region")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("region_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "region", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Falkenstein"))
        .stdout(predicate::str::contains("New York"));

    mock.assert();
}

#[test]
fn test_region_list_json() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/region")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("region_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--json", "--api-key", "test-key", "region", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Id\": 1"));

    mock.assert();
}

#[test]
fn test_region_list_auth_error() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/region")
        .with_status(401)
        .with_body(r#"{"Message":"Invalid API key"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad-key", "region", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}
```

- [ ] **Step 9: Run all tests**

Run: `cargo test --workspace`
Expected: All pass.

- [ ] **Step 10: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: No warnings.

- [ ] **Step 11: Commit**

```bash
git add bunnynet-lib/src/models/ bunnynet/src/cmd/ bunnynet/src/main.rs bunnynet/tests/
git commit -m "feat: add region list command"
```

---

### Task 8: Country resource

Follows the Region pattern exactly. One GET endpoint, no pagination.

**Files:**
- Create: `bunnynet-lib/src/models/country.rs`
- Create: `bunnynet/src/cmd/country.rs`
- Create: `bunnynet/tests/fixtures/country_list.json`
- Create: `bunnynet/tests/country_test.rs`
- Modify: `bunnynet-lib/src/models/mod.rs`, `bunnynet/src/cmd/mod.rs`, `bunnynet/src/main.rs`

**OpenAPI schema:** `CountryModel` — properties: Name (string), IsoCode (string), IsEU (boolean), TaxRate (double), TaxPrefix (string), FlagUrl (string), PopList (array of string).

**Row columns:** ISO CODE, NAME, EU, TAX RATE

**Command:** `bunnynet country list` → `GET /country`

**Fixture:** 2-3 sample countries with realistic data.

**Tests:** Table output, JSON output, auth error. Same pattern as Task 7.

- [ ] **Step 1: Create country model with test** — derive from `CountryModel` schema
- [ ] **Step 2: Update models/mod.rs** — add `pub mod country;`
- [ ] **Step 3: Run model test** — `cargo test -p bunnynet-lib models::country`
- [ ] **Step 4: Create country command handler** — `GET /country`, same pattern as region
- [ ] **Step 5: Update cmd/mod.rs** — add `pub mod country;`
- [ ] **Step 6: Wire into main.rs** — add `Country` variant + dispatch
- [ ] **Step 7: Create test fixture** — `country_list.json`
- [ ] **Step 8: Create integration test** — `country_test.rs`
- [ ] **Step 9: Run all tests** — `cargo test --workspace`
- [ ] **Step 10: Commit** — `feat: add country list command`

---

### Task 9: API Key resource

First resource with pagination. Uses `PaginatedList<ApiKeyModel>`.

**Files:**
- Create: `bunnynet-lib/src/models/api_key.rs`
- Create: `bunnynet/src/cmd/api_key.rs`
- Create: `bunnynet/tests/fixtures/api_key_list.json`
- Create: `bunnynet/tests/api_key_test.rs`
- Modify: `bunnynet-lib/src/models/mod.rs`, `bunnynet/src/cmd/mod.rs`, `bunnynet/src/main.rs`

**OpenAPI schema:** `ApiKeyModel` — properties: Id (int64), Key (string nullable), Roles (array of string nullable).

**Row columns:** ID, KEY, ROLES

**Command:**
```
bunnynet api-key list [--page <N>] [--per-page <N>]
```
Maps to `GET /apikey?page=N&perPage=N`. Response is `PaginatedList<ApiKeyModel>`.

In table mode, print the table then call `output::print_pagination(...)`.

**Fixture:** Wrap 2 sample keys in `{"Items":[...],"CurrentPage":0,"TotalItems":2,"HasMoreItems":false}`.

**Tests:** Table output with pagination line, JSON output, auth error.

- [ ] **Step 1: Create api_key model with test**
- [ ] **Step 2: Update models/mod.rs**
- [ ] **Step 3: Run model test**
- [ ] **Step 4: Create api_key command handler** — handle pagination params and print_pagination
- [ ] **Step 5: Update cmd/mod.rs**
- [ ] **Step 6: Wire into main.rs** — use `#[command(name = "api-key")]` for kebab-case
- [ ] **Step 7: Create test fixture**
- [ ] **Step 8: Create integration test** — verify pagination line appears
- [ ] **Step 9: Run all tests**
- [ ] **Step 10: Commit** — `feat: add api-key list command with pagination`

---

### Task 10: Purge resource

First POST command. Uses query parameters, not a JSON body.

**Files:**
- Create: `bunnynet/src/cmd/purge.rs`
- Create: `bunnynet/tests/fixtures/purge_url.json`
- Create: `bunnynet/tests/purge_test.rs`
- Modify: `bunnynet/src/cmd/mod.rs`, `bunnynet/src/main.rs`

No model needed — purge returns 200 on success with minimal or empty body.

**Command:**
```
bunnynet purge url <URL> [--async] [--exact-path]
```
Maps to `POST /purge?url=<URL>&async=<bool>&exactPath=<bool>`. Uses `post_with_params`.

**Important:** The response has no body (204). Do NOT call `.json()` on the response — it will fail. Just check for success status (which `check_status` already handles), then print confirmation.

In table mode, print confirmation: `"Purge queued for <URL>"`. In JSON mode, construct and print `{"status":"purged","url":"<URL>"}`.

**Tests:** Success (204 response), auth error.

- [ ] **Step 1: Create purge command handler** — use `post_with_params`
- [ ] **Step 2: Update cmd/mod.rs**
- [ ] **Step 3: Wire into main.rs**
- [ ] **Step 4: Create integration test**
- [ ] **Step 5: Run all tests**
- [ ] **Step 6: Commit** — `feat: add purge url command`

---

### Task 11: Search resource

GET with query parameters, returns a wrapper object.

**Files:**
- Create: `bunnynet-lib/src/models/search.rs`
- Create: `bunnynet/src/cmd/search.rs`
- Create: `bunnynet/tests/fixtures/search_results.json`
- Create: `bunnynet/tests/search_test.rs`
- Modify: `bunnynet-lib/src/models/mod.rs`, `bunnynet/src/cmd/mod.rs`, `bunnynet/src/main.rs`

**OpenAPI schemas:** `SearchResultsModel` (Query, Total, From, Size, SearchResults: array of SearchResultItemModel). `SearchResultItemModel` (Type: **string** (not int — it's a resource type name like "pullzone"), Id: int64, Name: string nullable).

**Row columns:** TYPE, ID, NAME

**Command:**
```
bunnynet search <QUERY> [--from <N>] [--size <N>]
```
Maps to `GET /search?search=<QUERY>&from=N&size=N`.

- [ ] **Step 1: Create search model with test**
- [ ] **Step 2: Update models/mod.rs**
- [ ] **Step 3: Run model test**
- [ ] **Step 4: Create search command handler**
- [ ] **Step 5: Update cmd/mod.rs**
- [ ] **Step 6: Wire into main.rs**
- [ ] **Step 7: Create test fixture**
- [ ] **Step 8: Create integration test**
- [ ] **Step 9: Run all tests**
- [ ] **Step 10: Commit** — `feat: add global search command`

---

## Chunk 3: Billing & Statistics

### Task 12: Statistics resource

GET with many boolean toggle flags. No model needed — response is a complex stats object best served as JSON in table mode too (key-value pairs for summaries, raw data for charts).

**Files:**
- Create: `bunnynet-lib/src/models/statistics.rs`
- Create: `bunnynet/src/cmd/statistics.rs`
- Create: `bunnynet/tests/fixtures/statistics_get.json`
- Create: `bunnynet/tests/statistics_test.rs`
- Modify: `bunnynet-lib/src/models/mod.rs`, `bunnynet/src/cmd/mod.rs`, `bunnynet/src/main.rs`

**OpenAPI schema:** `StatisticsModel` — TotalBandwidthUsed (int64), TotalOriginTraffic (int64), AverageOriginResponseTime (**int32**, not int64), TotalRequestsServed (int64), CacheHitRate (double), plus chart data (maps/arrays). Derive ALL field types from the OpenAPI spec — do not assume int64 for all integers.

**Row: key-value pairs** for summary fields (total bandwidth, requests served, cache hit rate, origin response time). Chart data only shown in `--json` mode.

**Command:**
```
bunnynet statistics [--date-from <DATE>] [--date-to <DATE>] [--pull-zone <ID>]
    [--server-zone-id <ID>] [--hourly]
    [--load-errors] [--load-origin-response-times] [--load-requests-served]
    [--load-bandwidth-used] [--load-origin-traffic] [--load-origin-shield-bandwidth]
    [--load-geographic-traffic-distribution] [--load-user-balance-history]
```
Maps to `GET /statistics` with all params as query string. Note: `statistics` has no subcommands — it's a direct command with flags.

- [ ] **Step 1: Create statistics model with test**
- [ ] **Step 2: Update models/mod.rs**
- [ ] **Step 3: Create statistics command handler** — build params vec from all Option flags
- [ ] **Step 4: Update cmd/mod.rs + main.rs**
- [ ] **Step 5: Create fixture + integration test**
- [ ] **Step 6: Run all tests**
- [ ] **Step 7: Commit** — `feat: add statistics command`

---

### Task 13: Billing resource

Multiple subcommands including PDF downloads.

**Files:**
- Create: `bunnynet-lib/src/models/billing.rs`
- Create: `bunnynet/src/cmd/billing.rs`
- Create: `bunnynet/tests/fixtures/billing_get.json`, `billing_summary.json`, `billing_affiliate.json`, `billing_payment_requests.json`
- Create: `bunnynet/tests/billing_test.rs`
- Modify: `bunnynet-lib/src/models/mod.rs`, `bunnynet/src/cmd/mod.rs`, `bunnynet/src/main.rs`

**OpenAPI schemas:**
- `BillingModel` — massive model with 60+ fields for balance, charges, usage metrics. Derive all from spec.
- `BillingSummaryItem` — PullZoneId, MonthlyUsage, MonthlyBandwidthUsed.
- `BillingAffiliateDetailsModel` — AffiliateBalance, AffiliateUrl, etc.
- `PaymentRequestModel` — Id, Amount, DateGenerated, DateDue, Description, Paid, etc.

**Commands:**
```
billing get                                    → GET /billing
billing summary                                → GET /billing/summary
billing affiliate                              → GET /billing/affiliate
billing payment-requests                       → GET /billing/payment-requests
billing download-invoice <ID> --output <FILE>  → GET /billing/payment-request-invoice/{id}/pdf
billing download-summary <ID> --output <FILE>  → GET /billing/summary/{id}/pdf
```

For `download-invoice` and `download-summary`:
```rust
BillingAction::DownloadInvoice { id, output } => {
    let bytes = client.get_bytes(&format!("/billing/payment-request-invoice/{}/pdf", id))?;
    std::fs::write(&output, &bytes)
        .with_context(|| format!("Failed to write to {}", output))?;
    output::print_confirm(&format!("Invoice saved to {}", output));
    Ok(())
}
```

Table mode for `billing get`: key-value pairs showing Balance, ThisMonthCharges, MonthlyBandwidthUsed.

- [ ] **Step 1: Create billing models with test**
- [ ] **Step 2: Update models/mod.rs**
- [ ] **Step 3: Create billing command handler** — all 6 subcommands
- [ ] **Step 4: Update cmd/mod.rs + main.rs**
- [ ] **Step 5: Create fixtures + integration tests** — test each subcommand
- [ ] **Step 6: Run all tests**
- [ ] **Step 7: Commit** — `feat: add billing commands with PDF download`

---

## Chunk 4: Storage Zones

### Task 14: Storage Zone CRUD

**Files:**
- Create: `bunnynet-lib/src/models/storage_zone.rs`
- Create: `bunnynet/src/cmd/storage_zone.rs`
- Create: `bunnynet/tests/fixtures/storage_zone_list.json`, `storage_zone_get.json`
- Create: `bunnynet/tests/storage_zone_test.rs`
- Modify: `bunnynet-lib/src/models/mod.rs`, `bunnynet/src/cmd/mod.rs`, `bunnynet/src/main.rs`

**OpenAPI schemas:**
- `StorageZoneModel` — Id, UserId, Name, Password, DateModified, Deleted, StorageUsed, FilesStored, Region, ReplicationRegions, PullZones, ReadOnlyPassword, Rewrite404To200, Custom404FilePath, StorageHostname, ZoneTier, ReplicationChangeInProgress, PriceOverride, Discount, StorageZoneType.
- `StorageZoneModelAdd` (required: Name, Region) — Name, Region, ReplicationRegions, ZoneTier, StorageZoneType.
- `StorageZoneSettingsModel` — ReplicationZones, OriginUrl, Custom404FilePath, Rewrite404To200.
- Enums: `StorageZoneTier` (0=Standard, 1=Edge), `StorageZoneS3Type` — derive values from OpenAPI spec (do not assume names; check the spec's enum definition).

**Row columns:** ID, NAME, REGION, STORAGE USED, FILES, HOSTNAME, TIER

**Commands (CRUD only in this task):**
```
storage-zone list [--search] [--page] [--per-page] [--include-deleted]  → GET /storagezone
storage-zone get <ID>                                                    → GET /storagezone/{id}
storage-zone create <NAME> --region <REGION> [--replication-regions] [--zone-tier] [--storage-zone-type]  → POST /storagezone
storage-zone update <ID> [--origin-url] [--custom-404-file-path] [--rewrite-404-to-200] [--replication-zones]  → POST /storagezone/{id}
storage-zone delete <ID> [--delete-linked-pull-zones]                    → DELETE /storagezone/{id}?deleteLinkedPullZones=bool
```

**Important: dual response format.** Same as pull zones — the storage zone `list` endpoint returns a plain array when `page` is not provided or `page=0`, but a paginated object when `page>=1`. **Always send `page=1` as the minimum.** Default `--page` to 1.

Note: `delete` uses a query parameter for `deleteLinkedPullZones`. Since the client doesn't have a `delete_with_params` method, construct the URL manually: `/storagezone/{id}?deleteLinkedPullZones=true` when the flag is set, otherwise `/storagezone/{id}`.

- [ ] **Step 1: Create storage_zone model with enums and test**
- [ ] **Step 2: Update models/mod.rs**
- [ ] **Step 3: Create storage_zone command handler** — CRUD commands
- [ ] **Step 4: Update cmd/mod.rs + main.rs** — use `#[command(name = "storage-zone")]`
- [ ] **Step 5: Create fixtures + integration tests**
- [ ] **Step 6: Run all tests**
- [ ] **Step 7: Commit** — `feat: add storage zone CRUD commands`

---

### Task 15: Storage Zone actions

**Additional commands:**
```
storage-zone check-availability <NAME>          → POST /storagezone/checkavailability (body: {"Name":"..."})
storage-zone reset-password <ID>                → POST /storagezone/{id}/resetPassword (no body)
storage-zone reset-read-only-password --id <ID> → POST /storagezone/resetReadOnlyPassword?id=<ID> (query param!)
storage-zone statistics <ID> [--date-from] [--date-to]  → GET /storagezone/{id}/statistics
```

**OpenAPI schema:** `StorageZoneStatisticsModel` — StorageUsedChart, FileCountChart (both maps).

Note `reset-password` uses `post_no_body`. `reset-read-only-password` uses `post_with_params` because the API takes `id` as a query parameter (not a path parameter — this is an API inconsistency).

- [ ] **Step 1: Add StatisticsModel to storage_zone model file**
- [ ] **Step 2: Add new action variants to StorageZoneAction enum**
- [ ] **Step 3: Implement handlers** — check-availability, reset-password, reset-read-only-password, statistics
- [ ] **Step 4: Add fixtures + integration tests for new actions**
- [ ] **Step 5: Run all tests**
- [ ] **Step 6: Commit** — `feat: add storage zone actions (reset password, statistics, availability)`

---

## Chunk 5: DNS Zones

### Task 16: DNS Zone CRUD

**Files:**
- Create: `bunnynet-lib/src/models/dns_zone.rs`
- Create: `bunnynet/src/cmd/dns_zone.rs`
- Create: fixtures + test files
- Modify: models/mod.rs, cmd/mod.rs, main.rs

**OpenAPI schemas:**
- `DnsZoneModel` — Id, Domain, Records (array of DnsRecordModel), DateModified, DateCreated, NameserversDetected, CustomNameserversEnabled, Nameserver1, Nameserver2, SoaEmail, NameserversNextCheck, LoggingEnabled, LoggingIPAnonymizationEnabled, LogAnonymizationType, DnsSecEnabled, CertificateKeyType.
- `DnsZoneAddModel` (required: Domain) — Domain, Records.
- `UpdateDnsZoneModel` — CustomNameserversEnabled, Nameserver1, Nameserver2, SoaEmail, LoggingEnabled, LogAnonymizationType, CertificateKeyType, LoggingIPAnonymizationEnabled.
- `DnsSecDsRecordModel` — Enabled, DsRecord, Digest, etc.
- `DnsZoneStatisticsModel` — TotalQueriesServed, QueriesServedChart, etc.

**Row columns:** ID, DOMAIN, NAMESERVERS DETECTED, DNSSEC, LOGGING, DATE CREATED

**Commands (CRUD):**
```
dns-zone list [--search] [--page] [--per-page]  → GET /dnszone (paginated)
dns-zone get <ID>                                → GET /dnszone/{id}
dns-zone create <DOMAIN>                         → POST /dnszone (body: {"Domain":"..."})
dns-zone update <ID> [flags]                     → POST /dnszone/{id}
dns-zone delete <ID>                             → DELETE /dnszone/{id}
```

The `list` endpoint returns `PaginatedList<DnsZoneModel>`.

This task implements the `DnsZoneAction` enum with CRUD variants. Record, Dnssec, and action subcommands come in Tasks 17-18.

- [ ] **Step 1: Create dns_zone model with test** — include DnsZoneModel, enums (LogAnonymizationType, etc.)
- [ ] **Step 2: Update models/mod.rs**
- [ ] **Step 3: Create dns_zone command handler** — CRUD + nested subcommand stubs for Record/Dnssec
- [ ] **Step 4: Update cmd/mod.rs + main.rs** — use `#[command(name = "dns-zone")]`
- [ ] **Step 5: Create fixtures + integration tests**
- [ ] **Step 6: Run all tests**
- [ ] **Step 7: Commit** — `feat: add dns zone CRUD commands`

---

### Task 17: DNS Records

**Files:**
- Create: `bunnynet-lib/src/models/dns_record.rs`
- Add record subcommands to `bunnynet/src/cmd/dns_zone.rs`
- Create: additional fixtures + tests

**OpenAPI schemas:**
- `DnsRecordModel` — Id, Type (DnsRecordTypes enum), Ttl, Value, Name, Weight, Priority, Port, Flags, Tag, Accelerated, AcceleratedPullZoneId, LinkName, MonitorStatus, MonitorType, GeolocationLatitude, GeolocationLongitude, LatencyZone, SmartRoutingType, Disabled, EnviromentalVariables, Comment, AutoSslIssuance, AccelerationStatus.
- `AddDnsRecordModel` — same fields as update (Type, Ttl, Value, Name, Weight, Priority, Flags, Tag, Port, PullZoneId, ScriptId, Accelerated, MonitorType, GeolocationLatitude, GeolocationLongitude, LatencyZone, SmartRoutingType, Disabled, EnviromentalVariables, Comment, AutoSslIssuance).
- `DnsRecordTypes` enum: A=0, AAAA=1, CNAME=2, TXT=3, MX=4, Redirect=5, Flatten=6, PullZone=7, NS=8, SRV=9, CAA=10, PTR=11, Script=12, NAPTR=13, SSHFP=14, TLSA=15.
- Additional enums: DnsSmartRoutingType, DnsMonitoringType, DnsMonitoringStatus, AcceleratedStatus.

**Row columns:** ID, TYPE, NAME, VALUE, TTL, PRIORITY, DISABLED

**Commands:**
```
dns-zone record add <ZONE_ID> --type <TYPE> --name <NAME> --value <VALUE> [--ttl] [--priority] [--weight] [--port] [--comment] [--disabled]
    → PUT /dnszone/{zoneId}/records

dns-zone record update <ZONE_ID> <RECORD_ID> [--type] [--name] [--value] [--ttl] [--priority] [--weight] [--port] [--comment] [--disabled]
    → POST /dnszone/{zoneId}/records/{id}

dns-zone record delete <ZONE_ID> <RECORD_ID>
    → DELETE /dnszone/{zoneId}/records/{id}
```

The `--type` flag should accept human-readable names (A, AAAA, CNAME, etc.) and convert to the integer value for the API. Use clap `value_parser` with the enum's `FromStr` impl.

- [ ] **Step 1: Create dns_record model with enums (DnsRecordType etc.) and test**
- [ ] **Step 2: Update models/mod.rs**
- [ ] **Step 3: Add DnsRecordAction enum to dns_zone command handler**
- [ ] **Step 4: Implement record add/update/delete handlers**
- [ ] **Step 5: Create fixtures + integration tests**
- [ ] **Step 6: Run all tests**
- [ ] **Step 7: Commit** — `feat: add dns record add, update, delete commands`

---

### Task 18: DNS Zone actions

Remaining DNS zone subcommands: export, import, statistics, check-availability, DNSSEC, certificate, record scan.

**Commands:**
```
dns-zone export <ID>                    → GET /dnszone/{id}/export (returns text)
dns-zone import <ID> --file <PATH>      → POST /dnszone/{zoneId}/import (body: file contents as text)
dns-zone statistics <ID> [--date-from] [--date-to]  → GET /dnszone/{id}/statistics
dns-zone check-availability <NAME>      → POST /dnszone/checkavailability (body: {"Name":"..."})
dns-zone dnssec enable <ID>             → POST /dnszone/{id}/dnssec
dns-zone dnssec disable <ID>            → DELETE /dnszone/{id}/dnssec
dns-zone certificate issue <ZONE_ID> [--domain <DOMAIN>]  → POST /dnszone/{zoneId}/certificate/issue
dns-zone record scan [--zone-id <ID>] [--domain <DOMAIN>] → POST /dnszone/records/scan
    Note: exactly one of --zone-id or --domain must be provided. Use clap `#[group(required = true, multiple = false, id = "target")]` on both args to enforce this.
dns-zone record scan-results <ZONE_ID>  → GET /dnszone/{zoneId}/records/scan
```

**OpenAPI schemas:** `DnsZoneStatisticsModel`, `DnsZoneImportResultModel` (RecordsSuccessful, RecordsFailed, RecordsSkipped), `DnsZoneRecordScanTriggerResponse`, `DnsZoneRecordScanJobResponse`, `DnsZoneDiscoveredRecordModel`, `IssueWildcardCertificateRequestModel`.

**Enums (use serde_repr):** `DnsZoneScanJobStatus` (Pending=0, InProgress=1, Completed=2, Failed=3), `DnsZoneDiscoveredRecordType` (13 values — derive from OpenAPI spec).

Note: `export` returns the zone file as `application/octet-stream` — use `get_bytes` then write to stdout or a file. `import` sends the file contents as a raw text POST body — use `post_text(path, &contents)`. Read the file with `std::fs::read_to_string`, then call `client.post_text()`.

- [ ] **Step 1: Add remaining action variants to DnsZoneAction and DnsRecordAction enums**
- [ ] **Step 2: Implement handlers for each action**
- [ ] **Step 3: Create fixtures + integration tests**
- [ ] **Step 4: Run all tests**
- [ ] **Step 5: Commit** — `feat: add dns zone actions (export, import, dnssec, statistics, scan, certificate)`

---

## Chunk 6: Pull Zones

### Task 19: Pull Zone CRUD

The largest model in the API. PullZoneModel has 180+ fields.

**Files:**
- Create: `bunnynet-lib/src/models/pull_zone.rs`
- Create: `bunnynet/src/cmd/pull_zone.rs`
- Create: fixtures + test files
- Modify: models/mod.rs, cmd/mod.rs, main.rs

**OpenAPI schemas:**
- `PullZoneModel` — 180+ fields. Derive ALL from the OpenAPI spec. Every field except `Id` should be `Option<T>`.
- `PullZoneAddModel` (required: Name) — 130+ optional settings fields. Same structure as PullZoneSettingsModel.
- `PullZoneSettingsModel` — 130+ fields for update.
- Enums: `PullZoneType`, `PullZoneOriginType`, `PullZoneLogFormat`, `PullZoneLogForwarderProtocolType`, `OptimizerWatermarkPosition`, `PermaCacheType`, `PreloadingScreenTheme`, `StickySessionType`, `ShieldDDosProtectionType`.
- Nested models in pull_zone.rs: `HostnameModel`, `EdgeRuleV2Model`, `EdgeRuleV2ActionModel`, `Trigger`, `OptimizerClassModel`, `BunnyAiImageBlueprintModel`.

**Row columns:** ID, NAME, ORIGIN URL, ORIGIN TYPE, ENABLED, MONTHLY BANDWIDTH, HOSTNAMES (count)

**Important: dual response format.** The pull zone `list` endpoint returns a plain JSON array when `page=0` (the default), but `PaginatedList<PullZoneModel>` when `page>=1`. To keep the handler simple, **always send `page=1` as the minimum value.** Default `--page` to 1, not 0.

**Commands (CRUD):**
```
pull-zone list [--search] [--page] [--per-page] [--include-certificate]  → GET /pullzone (paginated, always send page>=1)
pull-zone get <ID> [--include-certificate]                                → GET /pullzone/{id}
pull-zone create <NAME> [--origin-url <URL>] [...many optional flags]     → POST /pullzone
pull-zone update <ID> [...many optional flags]                            → POST /pullzone/{id}
pull-zone delete <ID>                                                     → DELETE /pullzone/{id}
pull-zone purge-cache <ID> [--cache-tag <TAG>]                           → POST /pullzone/{id}/purgeCache
pull-zone check-availability <NAME>                                       → POST /pullzone/checkavailability
```

For `create` and `update`: only include commonly-used flags as named CLI arguments. The full 130+ fields are accessible via `--json-body <FILE>` flag that reads a JSON file with the complete settings. This avoids 130 clap arguments.

Implementation note: Add a `--json-body <PATH>` flag to both `Create` and `Update` variants that reads a JSON file and sends it as the request body. Named flags take precedence over json-body fields.

**Shared helper for --json-body:** Create a helper function in `cmd/pull_zone.rs` (and reuse in `cmd/video_library.rs`):
```rust
fn load_json_body(path: &str) -> Result<HashMap<String, serde_json::Value>> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))?;
    let value: serde_json::Value = serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse JSON from {}", path))?;
    match value {
        serde_json::Value::Object(map) => Ok(map.into_iter().collect()),
        _ => bail!("Expected JSON object in {}", path),
    }
}
```

- [ ] **Step 1: Create pull_zone model file** — PullZoneModel with ALL fields from OpenAPI, plus enums, nested models (HostnameModel, EdgeRuleV2Model, Trigger, etc.)
- [ ] **Step 2: Add Row type** — select ~7 most useful columns
- [ ] **Step 3: Add unit test**
- [ ] **Step 4: Update models/mod.rs**
- [ ] **Step 5: Create pull_zone command handler** — CRUD + purge-cache + check-availability. Stub sub-enums for hostname/certificate/edge-rule/referrer/blocked-ip
- [ ] **Step 6: Update cmd/mod.rs + main.rs** — use `#[command(name = "pull-zone")]`
- [ ] **Step 7: Create fixtures + integration tests**
- [ ] **Step 8: Run all tests**
- [ ] **Step 9: Commit** — `feat: add pull zone CRUD commands`

---

### Task 20: Pull Zone hostname + certificate + edge rules

**Commands:**
```
pull-zone hostname add <ID> --hostname <HOST>                → POST /pullzone/{id}/addHostname
pull-zone hostname remove <ID> --hostname <HOST>             → DELETE /pullzone/{id}/removeHostname (uses delete_with_body!)
pull-zone hostname set-force-ssl <ID> --hostname <HOST> --force-ssl <BOOL>  → POST /pullzone/{id}/setForceSSL
pull-zone hostname set-private-key-type <ID> --hostname <HOST> --key-type <TYPE>  → POST /pullzone/{id}/updatePrivateKeyType

pull-zone certificate add <ID> --hostname <HOST> --certificate <FILE> --certificate-key <FILE>  → POST /pullzone/{id}/addCertificate
    Note: --certificate and --certificate-key are FILE PATHS. Read the file, base64-encode the contents, then send as the Certificate/CertificateKey JSON fields.
pull-zone certificate remove <ID> --hostname <HOST>          → DELETE /pullzone/{id}/removeCertificate (uses delete_with_body!)
pull-zone certificate load-free <HOSTNAME> [--use-only-http01]  → GET /pullzone/loadFreeCertificate?hostname=X

pull-zone edge-rule add-or-update <ID> --json-body <FILE>   → POST /pullzone/{pullZoneId}/edgerules/addOrUpdate
pull-zone edge-rule delete <ZONE_ID> <RULE_ID>              → DELETE /pullzone/{pullZoneId}/edgerules/{edgeRuleId}
pull-zone edge-rule set-enabled <ZONE_ID> <RULE_ID> --enabled <BOOL>  → POST /pullzone/{pullZoneId}/edgerules/{edgeRuleId}/setEdgeRuleEnabled
```

Edge rules have complex nested JSON (triggers, actions). Use `--json-body <FILE>` for `add-or-update` rather than trying to express the full edge rule structure as CLI flags.

**OpenAPI schemas:** `AddHostnameRequestModel`, `RemoveHostnameRequestModel`, `ForceSSLRequestModel`, `PrivateKeyUpdateModel`, `AddCertificateRequestModel`, `RemoveCertificateRequestModel`, `EdgeRuleV2Model`, `ToggleRequestModel`.

- [ ] **Step 1: Add PullZoneHostnameAction, PullZoneCertificateAction, PullZoneEdgeRuleAction enums**
- [ ] **Step 2: Implement handlers for all hostname/certificate/edge-rule actions**
- [ ] **Step 3: Create fixtures + integration tests**
- [ ] **Step 4: Run all tests**
- [ ] **Step 5: Commit** — `feat: add pull zone hostname, certificate, and edge rule commands`

---

### Task 21: Pull Zone referrer + blocked-ip + stats

**Commands:**
```
pull-zone referrer add-allowed <ID> --hostname <HOST>        → POST /pullzone/{id}/addAllowedReferrer
pull-zone referrer remove-allowed <ID> --hostname <HOST>     → POST /pullzone/{id}/removeAllowedReferrer
pull-zone referrer add-blocked <ID> --hostname <HOST>        → POST /pullzone/{id}/addBlockedReferrer
pull-zone referrer remove-blocked <ID> --hostname <HOST>     → POST /pullzone/{id}/removeBlockedReferrer

pull-zone blocked-ip add <ID> --ip <IP>                      → POST /pullzone/{id}/addBlockedIp
pull-zone blocked-ip remove <ID> --ip <IP>                   → POST /pullzone/{id}/removeBlockedIp

pull-zone reset-security-key <ID> [--security-key <KEY>]      → POST /pullzone/{id}/resetSecurityKey (optional body with custom key 8-36 chars; omit for auto-generated)

pull-zone optimizer-statistics <ID> [--date-from] [--date-to] [--hourly]       → GET /pullzone/{pullZoneId}/optimizer/statistics
pull-zone origin-shield-statistics <ID> [--date-from] [--date-to] [--hourly]   → GET /pullzone/{pullZoneId}/originshield/queuestatistics
pull-zone safehop-statistics <ID> [--date-from] [--date-to] [--hourly]         → GET /pullzone/{pullZoneId}/safehop/statistics
```

All referrer/blocked-ip commands follow the same pattern: POST with a single-field JSON body.

**OpenAPI schemas:** `AddAllowedReferrerRequestModel`, `RemoveAllowedReferrerRequestModel`, `AddBlockedReferrerRequestModel`, `RemoveBlockedReferrerRequestModel`, `AddBlockedIpRequestModel`, `RemoveBlockedIpRequestModel`, `ResetPullZoneSecurityKeyModel`, `OptimizerStatisticsModel`, `OriginShieldConcurrencyStatisticsModel`, `SafeHopStatisticsModel`.

- [ ] **Step 1: Add PullZoneReferrerAction, PullZoneBlockedIpAction enums + remaining top-level actions**
- [ ] **Step 2: Implement all handlers**
- [ ] **Step 3: Create fixtures + integration tests**
- [ ] **Step 4: Run all tests**
- [ ] **Step 5: Commit** — `feat: add pull zone referrer, blocked-ip, and statistics commands`

---

## Chunk 7: Video Libraries

### Task 22: Video Library CRUD + languages

**Files:**
- Create: `bunnynet-lib/src/models/video_library.rs`
- Create: `bunnynet/src/cmd/video_library.rs`
- Create: fixtures + test files
- Modify: models/mod.rs, cmd/mod.rs, main.rs

**OpenAPI schemas:**
- `VideoLibraryModel` — 90+ fields. Derive all from spec. Includes player settings, encoding settings, DRM config, watermark settings, etc.
- `VideoLibraryCreateModel` (required: Name) — Name, ReplicationRegions, PlayerVersion.
- `VideoLibraryUpdateModel` — 50+ optional settings.
- Nested: `AppleFairPlayDrm`, `GoogleWidevineDrm`, `AppleFairPlayDrmUpdateModel`, `GoogleWidevineDrmUpdateModel`.
- Enums: `DrmVersion`, `EncodingTier`, `ExecutionPhase`, `WidevineMinClientSecurityLevel`.

**Row columns:** ID, NAME, API KEY, PULL ZONE ID, STORAGE ZONE ID, DRM ENABLED

**Important: dual response format.** Same as pull zones — the `list` endpoint returns a plain JSON array when `page=0` but `PaginatedList<VideoLibraryModel>` when `page>=1`. **Always send `page=1` as the minimum value.**

**Commands:**
```
video-library list [--search] [--page] [--per-page]  → GET /videolibrary (paginated, always send page>=1)
video-library get <ID>                                → GET /videolibrary/{id}
video-library create <NAME> [--replication-regions] [--player-version]  → POST /videolibrary
video-library update <ID> [--json-body <FILE>]        → POST /videolibrary/{id}
video-library delete <ID>                             → DELETE /videolibrary/{id}
video-library languages                               → GET /videolibrary/languages
```

Same `--json-body` pattern as pull zones for update (50+ fields).

- [ ] **Step 1: Create video_library model with all fields, enums, nested types, and test**
- [ ] **Step 2: Update models/mod.rs**
- [ ] **Step 3: Create video_library command handler** — CRUD + languages. Stub sub-enums for watermark/live-thumbnail/live-watermark
- [ ] **Step 4: Update cmd/mod.rs + main.rs** — use `#[command(name = "video-library")]`
- [ ] **Step 5: Create fixtures + integration tests**
- [ ] **Step 6: Run all tests**
- [ ] **Step 7: Commit** — `feat: add video library CRUD and languages commands`

---

### Task 23: Video Library actions

**Commands:**
```
video-library add-allowed-referrer <ID> --hostname <HOST>     → POST /videolibrary/{id}/addAllowedReferrer
video-library remove-allowed-referrer <ID> --hostname <HOST>  → POST /videolibrary/{id}/removeAllowedReferrer
video-library add-blocked-referrer <ID> --hostname <HOST>     → POST /videolibrary/{id}/addBlockedReferrer
video-library remove-blocked-referrer <ID> --hostname <HOST>  → POST /videolibrary/{id}/removeBlockedReferrer
video-library reset-api-key <ID>                              → POST /videolibrary/{id}/resetApiKey (no body)
video-library reset-read-only-api-key <ID>                    → POST /videolibrary/{id}/resetReadOnlyApiKey (no body)
video-library watermark add <ID> --file <PATH>                → PUT /videolibrary/{id}/watermark (binary upload)
video-library watermark delete <ID>                           → DELETE /videolibrary/{id}/watermark
video-library live-thumbnail add <ID> --file <PATH>           → PUT /videolibrary/{id}/live/thumbnail (binary upload)
video-library live-thumbnail delete <ID>                      → DELETE /videolibrary/{id}/live/thumbnail
video-library live-watermark add <ID> --file <PATH>           → PUT /videolibrary/{id}/live/watermark (binary upload)
video-library live-watermark delete <ID>                      → DELETE /videolibrary/{id}/live/watermark
video-library drm-statistics <ID> [--date-from] [--date-to]              → GET /videolibrary/{id}/drm/statistics
video-library transcribing-statistics <ID> [--date-from] [--date-to]     → GET /videolibrary/{id}/transcribing/statistics
```

For watermark/thumbnail file uploads:
```rust
VideoLibraryAction::Watermark { action: WatermarkAction::Add { id, file } } => {
    let data = std::fs::read(&file)
        .with_context(|| format!("Failed to read file: {}", file))?;
    let content_type = if file.ends_with(".png") { "image/png" } else { "image/jpeg" };
    client.put_file(&format!("/videolibrary/{}/watermark", id), data, content_type)?;
    output::print_confirm("Watermark uploaded.");
    Ok(())
}
```

**OpenAPI schemas:** `VideoLibraryDrmStatisticsModel`, `VideoLibraryTranscriptionStatisticsModel`.

- [ ] **Step 1: Add remaining action variants** — referrer, api-key reset, watermark/live sub-enums, statistics
- [ ] **Step 2: Implement all handlers**
- [ ] **Step 3: Create fixtures + integration tests**
- [ ] **Step 4: Run all tests**
- [ ] **Step 5: Commit** — `feat: add video library actions (referrer, watermark, api-key reset, statistics)`

---

## Chunk 8: Polish

### Task 24: CLAUDE.md + README.md

**Files:**
- Create: `CLAUDE.md`
- Create: `README.md`

- [ ] **Step 1: Create CLAUDE.md**

Follow the pattern from the updown project's CLAUDE.md. Include:
- Build & test commands
- Architecture overview (two-crate workspace, blocking reqwest, clap derive)
- Library structure (config, client, models)
- CLI structure (main, output, cmd)
- Testing approach (mockito + assert_cmd, live tests gated by BUNNYNET_LIVE_TEST=1)
- Gotchas (gzip feature, PascalCase rename, integer enums via serde_repr, AccessKey header)
- Publishing info (license, semantic-release)

- [ ] **Step 2: Create README.md**

Standard sections:
- Install (source, nix)
- Configuration (config file, env var, CLI flag)
- Usage examples for each resource
- Output modes (table, JSON)
- API coverage table (endpoint → command mapping)
- Development (build, test, clippy, fmt)
- License

- [ ] **Step 3: Commit**

```bash
git add CLAUDE.md README.md
git commit -m "docs: add CLAUDE.md and README.md"
```

---

### Task 25: Final verification

- [ ] **Step 1: Run full test suite**

Run: `cargo test --workspace`
Expected: All tests pass.

- [ ] **Step 2: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: No warnings.

- [ ] **Step 3: Run fmt check**

Run: `cargo fmt --all --check`
Expected: No formatting issues.

- [ ] **Step 4: Build release**

Run: `cargo build --workspace --release`
Expected: Builds successfully.

- [ ] **Step 5: Verify help output**

Run: `cargo run -- --help`
Expected: Shows all 12 subcommands.

Run: `cargo run -- pull-zone --help`
Expected: Shows pull-zone subcommands including nested hostname/certificate/edge-rule.

- [ ] **Step 6: Commit any fixes, then tag completion**

```bash
git log --oneline
# Verify commit history looks clean with conventional commits
```
