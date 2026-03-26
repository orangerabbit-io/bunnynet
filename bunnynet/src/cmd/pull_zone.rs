use anyhow::{bail, Context, Result};
use base64::Engine;
use clap::Subcommand;
use std::collections::HashMap;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::pagination::PaginatedList;
use bunnynet_lib::models::pull_zone::{
    OptimizerStatisticsModel, OriginShieldConcurrencyStatisticsModel, PullZone, PullZoneRow,
    SafeHopStatisticsModel,
};

// --- Subcommand definitions ---

#[derive(Subcommand)]
pub enum PullZoneAction {
    /// List pull zones
    List {
        /// Search query
        #[arg(long)]
        search: Option<String>,
        /// Page number (starts at 1)
        #[arg(long, default_value = "1")]
        page: i32,
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
        /// Include certificate data in response
        #[arg(long)]
        include_certificate: bool,
    },
    /// Get a pull zone by ID
    Get {
        /// Pull zone ID
        id: i64,
        /// Include certificate data in response
        #[arg(long)]
        include_certificate: bool,
    },
    /// Create a new pull zone
    Create {
        /// Pull zone name
        name: String,
        /// Origin URL
        #[arg(long)]
        origin_url: Option<String>,
        /// Origin type (0=OriginUrl, 1=DnsAccelerate, 2=StorageZone, etc.)
        #[arg(long)]
        origin_type: Option<i32>,
        /// Storage zone ID to link
        #[arg(long)]
        storage_zone_id: Option<i64>,
        /// Path to JSON file with full settings
        #[arg(long)]
        json_body: Option<String>,
    },
    /// Update a pull zone
    Update {
        /// Pull zone ID
        id: i64,
        /// Origin URL
        #[arg(long)]
        origin_url: Option<String>,
        /// Path to JSON file with full settings
        #[arg(long)]
        json_body: Option<String>,
    },
    /// Delete a pull zone
    Delete {
        /// Pull zone ID
        id: i64,
    },
    /// Purge the pull zone cache
    PurgeCache {
        /// Pull zone ID
        id: i64,
        /// Optional cache tag to purge selectively
        #[arg(long)]
        cache_tag: Option<String>,
    },
    /// Check if a pull zone name is available
    CheckAvailability {
        /// Name to check
        name: String,
    },
    /// Manage pull zone hostnames
    Hostname {
        #[command(subcommand)]
        action: PullZoneHostnameAction,
    },
    /// Manage pull zone certificates
    Certificate {
        #[command(subcommand)]
        action: PullZoneCertificateAction,
    },
    /// Manage pull zone edge rules
    EdgeRule {
        #[command(subcommand)]
        action: PullZoneEdgeRuleAction,
    },
    /// Manage pull zone referrer rules
    Referrer {
        #[command(subcommand)]
        action: PullZoneReferrerAction,
    },
    /// Manage pull zone blocked IPs
    BlockedIp {
        #[command(subcommand)]
        action: PullZoneBlockedIpAction,
    },
    /// Reset the pull zone security key
    ResetSecurityKey {
        /// Pull zone ID
        id: i64,
        /// Optional new security key value
        #[arg(long)]
        security_key: Option<String>,
    },
    /// View optimizer statistics for a pull zone
    OptimizerStatistics {
        /// Pull zone ID
        id: i64,
        /// Start date
        #[arg(long)]
        date_from: Option<String>,
        /// End date
        #[arg(long)]
        date_to: Option<String>,
        /// Return hourly grouping
        #[arg(long)]
        hourly: bool,
    },
    /// View origin shield queue statistics for a pull zone
    OriginShieldStatistics {
        /// Pull zone ID
        id: i64,
        /// Start date
        #[arg(long)]
        date_from: Option<String>,
        /// End date
        #[arg(long)]
        date_to: Option<String>,
        /// Return hourly grouping
        #[arg(long)]
        hourly: bool,
    },
    /// View SafeHop statistics for a pull zone
    SafehopStatistics {
        /// Pull zone ID
        id: i64,
        /// Start date
        #[arg(long)]
        date_from: Option<String>,
        /// End date
        #[arg(long)]
        date_to: Option<String>,
        /// Return hourly grouping
        #[arg(long)]
        hourly: bool,
    },
}

#[derive(Subcommand)]
pub enum PullZoneHostnameAction {
    /// Add a custom hostname
    Add {
        /// Pull zone ID
        id: i64,
        /// Hostname to add
        #[arg(long)]
        hostname: String,
    },
    /// Remove a custom hostname
    Remove {
        /// Pull zone ID
        id: i64,
        /// Hostname to remove
        #[arg(long)]
        hostname: String,
    },
    /// Set force SSL for a hostname
    SetForceSsl {
        /// Pull zone ID
        id: i64,
        /// Hostname
        #[arg(long)]
        hostname: String,
        /// Enable or disable force SSL (true/false)
        #[arg(long, num_args = 1)]
        force_ssl: bool,
    },
    /// Set private key type for a hostname
    SetPrivateKeyType {
        /// Pull zone ID
        id: i64,
        /// Hostname
        #[arg(long)]
        hostname: String,
        /// Key type (0=Ecdsa, 1=Rsa)
        #[arg(long)]
        key_type: i32,
    },
}

#[derive(Subcommand)]
pub enum PullZoneCertificateAction {
    /// Add a custom certificate
    Add {
        /// Pull zone ID
        id: i64,
        /// Hostname the certificate is for
        #[arg(long)]
        hostname: String,
        /// Path to certificate file (will be base64 encoded)
        #[arg(long)]
        certificate: String,
        /// Path to certificate key file (will be base64 encoded)
        #[arg(long)]
        certificate_key: String,
    },
    /// Remove a certificate
    Remove {
        /// Pull zone ID
        id: i64,
        /// Hostname to remove certificate from
        #[arg(long)]
        hostname: String,
    },
    /// Load a free Let's Encrypt certificate
    LoadFree {
        /// Hostname to load certificate for
        hostname: String,
        /// Use only HTTP-01 challenge
        #[arg(long)]
        use_only_http01: bool,
    },
}

#[derive(Subcommand)]
pub enum PullZoneEdgeRuleAction {
    /// Add or update an edge rule (provide JSON file)
    AddOrUpdate {
        /// Pull zone ID
        id: i64,
        /// Path to JSON file with edge rule definition
        #[arg(long)]
        json_body: String,
    },
    /// Delete an edge rule
    Delete {
        /// Pull zone ID
        zone_id: i64,
        /// Edge rule GUID
        rule_id: String,
    },
    /// Enable or disable an edge rule
    SetEnabled {
        /// Pull zone ID
        zone_id: i64,
        /// Edge rule GUID
        rule_id: String,
        /// Enable or disable (true/false)
        #[arg(long, num_args = 1)]
        enabled: bool,
    },
}

#[derive(Subcommand)]
pub enum PullZoneReferrerAction {
    /// Add an allowed referrer
    AddAllowed {
        /// Pull zone ID
        id: i64,
        /// Referrer hostname
        #[arg(long)]
        hostname: String,
    },
    /// Remove an allowed referrer
    RemoveAllowed {
        /// Pull zone ID
        id: i64,
        /// Referrer hostname
        #[arg(long)]
        hostname: String,
    },
    /// Add a blocked referrer
    AddBlocked {
        /// Pull zone ID
        id: i64,
        /// Referrer hostname
        #[arg(long)]
        hostname: String,
    },
    /// Remove a blocked referrer
    RemoveBlocked {
        /// Pull zone ID
        id: i64,
        /// Referrer hostname
        #[arg(long)]
        hostname: String,
    },
}

#[derive(Subcommand)]
pub enum PullZoneBlockedIpAction {
    /// Add a blocked IP
    Add {
        /// Pull zone ID
        id: i64,
        /// IP address to block
        #[arg(long)]
        ip: String,
    },
    /// Remove a blocked IP
    Remove {
        /// Pull zone ID
        id: i64,
        /// IP address to unblock
        #[arg(long)]
        ip: String,
    },
}

// --- Main dispatcher ---

#[allow(clippy::too_many_lines)]
pub fn run(action: PullZoneAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        PullZoneAction::List {
            search,
            page,
            per_page,
            include_certificate,
        } => list(client, mode, search, page, per_page, include_certificate),
        PullZoneAction::Get {
            id,
            include_certificate,
        } => get(client, mode, id, include_certificate),
        PullZoneAction::Create {
            name,
            origin_url,
            origin_type,
            storage_zone_id,
            json_body,
        } => create(client, mode, &name, origin_url, origin_type, storage_zone_id, json_body),
        PullZoneAction::Update {
            id,
            origin_url,
            json_body,
        } => update(client, mode, id, origin_url, json_body),
        PullZoneAction::Delete { id } => delete(client, mode, id),
        PullZoneAction::PurgeCache { id, cache_tag } => purge_cache(client, mode, id, cache_tag),
        PullZoneAction::CheckAvailability { name } => check_availability(client, mode, &name),
        PullZoneAction::Hostname { action: h_action } => run_hostname(h_action, client, mode),
        PullZoneAction::Certificate { action: c_action } => {
            run_certificate(c_action, client, mode)
        }
        PullZoneAction::EdgeRule { action: e_action } => run_edge_rule(e_action, client, mode),
        PullZoneAction::Referrer { action: r_action } => run_referrer(r_action, client, mode),
        PullZoneAction::BlockedIp { action: b_action } => run_blocked_ip(b_action, client, mode),
        PullZoneAction::ResetSecurityKey { id, security_key } => {
            reset_security_key(client, mode, id, security_key)
        }
        PullZoneAction::OptimizerStatistics {
            id,
            date_from,
            date_to,
            hourly,
        } => optimizer_statistics(client, mode, id, date_from, date_to, hourly),
        PullZoneAction::OriginShieldStatistics {
            id,
            date_from,
            date_to,
            hourly,
        } => origin_shield_statistics(client, mode, id, date_from, date_to, hourly),
        PullZoneAction::SafehopStatistics {
            id,
            date_from,
            date_to,
            hourly,
        } => safehop_statistics(client, mode, id, date_from, date_to, hourly),
    }
}

// --- Shared helpers ---

fn load_json_body(path: &str) -> Result<HashMap<String, serde_json::Value>> {
    let contents =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read {}", path))?;
    let value: serde_json::Value =
        serde_json::from_str(&contents).with_context(|| format!("Failed to parse JSON from {}", path))?;
    match value {
        serde_json::Value::Object(map) => Ok(map.into_iter().collect()),
        _ => bail!("Expected JSON object in {}", path),
    }
}

fn build_stats_params<'a>(
    date_from: &'a Option<String>,
    date_to: &'a Option<String>,
    hourly: bool,
    hourly_str: &'a str,
) -> Vec<(&'a str, &'a str)> {
    let mut params: Vec<(&str, &str)> = Vec::new();
    if let Some(ref df) = date_from {
        params.push(("dateFrom", df.as_str()));
    }
    if let Some(ref dt) = date_to {
        params.push(("dateTo", dt.as_str()));
    }
    if hourly {
        params.push(("hourly", hourly_str));
    }
    params
}

// --- Task 19: Pull Zone CRUD ---

fn list(
    client: &Client,
    mode: OutputMode,
    search: Option<String>,
    page: i32,
    per_page: Option<i32>,
    include_certificate: bool,
) -> Result<()> {
    let page_str = page.to_string();
    let mut params: Vec<(&str, String)> = vec![("page", page_str)];
    if let Some(ref s) = search {
        params.push(("search", s.clone()));
    }
    if let Some(pp) = per_page {
        params.push(("perPage", pp.to_string()));
    }
    if include_certificate {
        params.push(("includeCertificate", "true".to_string()));
    }
    let params_ref: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

    let resp = client.get_with_params("/pullzone", &params_ref)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let list: PaginatedList<PullZone> = resp.json()?;
            let rows: Vec<PullZoneRow> = list.items.iter().map(PullZoneRow::from).collect();
            output::print_table(&rows);
            output::print_pagination(list.current_page, list.total_items, list.has_more_items);
        }
    }

    Ok(())
}

fn get(client: &Client, mode: OutputMode, id: i64, include_certificate: bool) -> Result<()> {
    let path = format!("/pullzone/{}", id);

    let resp = if include_certificate {
        let params: Vec<(&str, &str)> = vec![("includeCertificate", "true")];
        client.get_with_params(&path, &params)?
    } else {
        client.get(&path)?
    };

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let pz: PullZone = resp.json()?;
            output::print_kv(&[
                ("ID", pz.id.to_string()),
                (
                    "Name",
                    pz.name.clone().unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Origin URL",
                    pz.origin_url.clone().unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Origin Type",
                    pz.origin_type
                        .map(|t| format!("{:?}", t))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Enabled",
                    pz.enabled
                        .map(|e| if e { "Yes" } else { "No" }.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Suspended",
                    pz.suspended
                        .map(|s| if s { "Yes" } else { "No" }.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Monthly Bandwidth Used",
                    pz.monthly_bandwidth_used
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Monthly Bandwidth Limit",
                    pz.monthly_bandwidth_limit
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Monthly Charges",
                    pz.monthly_charges
                        .map(|v| format!("{:.4}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Storage Zone ID",
                    pz.storage_zone_id
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Hostnames",
                    pz.hostnames
                        .as_ref()
                        .map(|h| {
                            h.iter()
                                .filter_map(|hn| hn.value.clone())
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "CNAME Domain",
                    pz.cname_domain
                        .clone()
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Zone Security",
                    pz.zone_security_enabled
                        .map(|e| if e { "Enabled" } else { "Disabled" }.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn create(
    client: &Client,
    mode: OutputMode,
    name: &str,
    origin_url: Option<String>,
    origin_type: Option<i32>,
    storage_zone_id: Option<i64>,
    json_body: Option<String>,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = if let Some(ref path) = json_body {
        load_json_body(path)?
    } else {
        HashMap::new()
    };

    // CLI flags override json_body fields
    body.insert("Name".to_string(), serde_json::json!(name));
    if let Some(url) = origin_url {
        body.insert("OriginUrl".to_string(), serde_json::json!(url));
    }
    if let Some(ot) = origin_type {
        body.insert("OriginType".to_string(), serde_json::json!(ot));
    }
    if let Some(sz_id) = storage_zone_id {
        body.insert("StorageZoneId".to_string(), serde_json::json!(sz_id));
    }

    let resp = client.post("/pullzone", &body)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let pz: PullZone = resp.json()?;
            output::print_confirm(&format!("Pull zone '{}' created (ID: {})", name, pz.id));
        }
    }

    Ok(())
}

fn update(
    client: &Client,
    mode: OutputMode,
    id: i64,
    origin_url: Option<String>,
    json_body: Option<String>,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = if let Some(ref path) = json_body {
        load_json_body(path)?
    } else {
        HashMap::new()
    };

    // CLI flags override json_body fields
    if let Some(url) = origin_url {
        body.insert("OriginUrl".to_string(), serde_json::json!(url));
    }

    let path = format!("/pullzone/{}", id);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "updated", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Pull zone {} updated", id));
        }
    }

    Ok(())
}

fn delete(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/pullzone/{}", id);
    let _resp = client.delete(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "deleted", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Pull zone {} deleted", id));
        }
    }

    Ok(())
}

fn purge_cache(
    client: &Client,
    mode: OutputMode,
    id: i64,
    cache_tag: Option<String>,
) -> Result<()> {
    let path = format!("/pullzone/{}/purgeCache", id);

    if let Some(ref tag) = cache_tag {
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert("CacheTag".to_string(), serde_json::json!(tag));
        let _resp = client.post(&path, &body)?;
    } else {
        let _resp = client.post_no_body(&path)?;
    }

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "cache_purged", "id": id, "cache_tag": cache_tag});
            output::print_json(&json);
        }
        OutputMode::Table => {
            if let Some(tag) = cache_tag {
                output::print_confirm(&format!(
                    "Cache purged for pull zone {} (tag: {})",
                    id, tag
                ));
            } else {
                output::print_confirm(&format!("Cache purged for pull zone {}", id));
            }
        }
    }

    Ok(())
}

fn check_availability(client: &Client, mode: OutputMode, name: &str) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Name".to_string(), serde_json::json!(name));

    let _resp = client.post("/pullzone/checkavailability", &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"available": true, "name": name});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Pull zone name '{}' is available", name));
        }
    }

    Ok(())
}

// --- Task 20: Hostname + Certificate + Edge Rules ---

fn run_hostname(action: PullZoneHostnameAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        PullZoneHostnameAction::Add { id, hostname } => hostname_add(client, mode, id, &hostname),
        PullZoneHostnameAction::Remove { id, hostname } => {
            hostname_remove(client, mode, id, &hostname)
        }
        PullZoneHostnameAction::SetForceSsl {
            id,
            hostname,
            force_ssl,
        } => hostname_set_force_ssl(client, mode, id, &hostname, force_ssl),
        PullZoneHostnameAction::SetPrivateKeyType {
            id,
            hostname,
            key_type,
        } => hostname_set_private_key_type(client, mode, id, &hostname, key_type),
    }
}

fn hostname_add(client: &Client, mode: OutputMode, id: i64, hostname: &str) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Hostname".to_string(), serde_json::json!(hostname));

    let path = format!("/pullzone/{}/addHostname", id);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "hostname_added", "id": id, "hostname": hostname});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "Hostname '{}' added to pull zone {}",
                hostname, id
            ));
        }
    }

    Ok(())
}

fn hostname_remove(client: &Client, mode: OutputMode, id: i64, hostname: &str) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Hostname".to_string(), serde_json::json!(hostname));

    let path = format!("/pullzone/{}/removeHostname", id);
    let _resp = client.delete_with_body(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "hostname_removed", "id": id, "hostname": hostname});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "Hostname '{}' removed from pull zone {}",
                hostname, id
            ));
        }
    }

    Ok(())
}

fn hostname_set_force_ssl(
    client: &Client,
    mode: OutputMode,
    id: i64,
    hostname: &str,
    force_ssl: bool,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Hostname".to_string(), serde_json::json!(hostname));
    body.insert("ForceSSL".to_string(), serde_json::json!(force_ssl));

    let path = format!("/pullzone/{}/setForceSSL", id);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "force_ssl_set", "id": id, "hostname": hostname, "force_ssl": force_ssl});
            output::print_json(&json);
        }
        OutputMode::Table => {
            let state = if force_ssl { "enabled" } else { "disabled" };
            output::print_confirm(&format!(
                "Force SSL {} for '{}' on pull zone {}",
                state, hostname, id
            ));
        }
    }

    Ok(())
}

fn hostname_set_private_key_type(
    client: &Client,
    mode: OutputMode,
    id: i64,
    hostname: &str,
    key_type: i32,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Hostname".to_string(), serde_json::json!(hostname));
    body.insert("KeyType".to_string(), serde_json::json!(key_type));

    let path = format!("/pullzone/{}/updatePrivateKeyType", id);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "private_key_type_set", "id": id, "hostname": hostname, "key_type": key_type});
            output::print_json(&json);
        }
        OutputMode::Table => {
            let type_name = match key_type {
                0 => "Ecdsa".to_string(),
                1 => "Rsa".to_string(),
                other => other.to_string(),
            };
            output::print_confirm(&format!(
                "Private key type set to {} for '{}' on pull zone {}",
                type_name, hostname, id
            ));
        }
    }

    Ok(())
}

fn run_certificate(
    action: PullZoneCertificateAction,
    client: &Client,
    mode: OutputMode,
) -> Result<()> {
    match action {
        PullZoneCertificateAction::Add {
            id,
            hostname,
            certificate,
            certificate_key,
        } => certificate_add(client, mode, id, &hostname, &certificate, &certificate_key),
        PullZoneCertificateAction::Remove { id, hostname } => {
            certificate_remove(client, mode, id, &hostname)
        }
        PullZoneCertificateAction::LoadFree {
            hostname,
            use_only_http01,
        } => certificate_load_free(client, mode, &hostname, use_only_http01),
    }
}

fn certificate_add(
    client: &Client,
    mode: OutputMode,
    id: i64,
    hostname: &str,
    cert_path: &str,
    key_path: &str,
) -> Result<()> {
    // API expects Base64 encoded binary data per AddCertificateRequestModel
    let cert_data = std::fs::read(cert_path)
        .with_context(|| format!("Failed to read certificate file: {}", cert_path))?;
    let key_data = std::fs::read(key_path)
        .with_context(|| format!("Failed to read certificate key file: {}", key_path))?;

    let engine = base64::engine::general_purpose::STANDARD;
    let cert_b64 = engine.encode(&cert_data);
    let key_b64 = engine.encode(&key_data);

    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Hostname".to_string(), serde_json::json!(hostname));
    body.insert("Certificate".to_string(), serde_json::json!(cert_b64));
    body.insert("CertificateKey".to_string(), serde_json::json!(key_b64));

    let path = format!("/pullzone/{}/addCertificate", id);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "certificate_added", "id": id, "hostname": hostname});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "Certificate added for '{}' on pull zone {}",
                hostname, id
            ));
        }
    }

    Ok(())
}

fn certificate_remove(client: &Client, mode: OutputMode, id: i64, hostname: &str) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Hostname".to_string(), serde_json::json!(hostname));

    let path = format!("/pullzone/{}/removeCertificate", id);
    let _resp = client.delete_with_body(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "certificate_removed", "id": id, "hostname": hostname});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "Certificate removed for '{}' on pull zone {}",
                hostname, id
            ));
        }
    }

    Ok(())
}

fn certificate_load_free(
    client: &Client,
    mode: OutputMode,
    hostname: &str,
    use_only_http01: bool,
) -> Result<()> {
    let mut params: Vec<(&str, &str)> = vec![("hostname", hostname)];
    if use_only_http01 {
        params.push(("useOnlyHttp01", "true"));
    }

    let _resp = client.get_with_params("/pullzone/loadFreeCertificate", &params)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "free_certificate_loaded", "hostname": hostname});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "Free certificate loaded for '{}'",
                hostname
            ));
        }
    }

    Ok(())
}

fn run_edge_rule(
    action: PullZoneEdgeRuleAction,
    client: &Client,
    mode: OutputMode,
) -> Result<()> {
    match action {
        PullZoneEdgeRuleAction::AddOrUpdate { id, json_body } => {
            edge_rule_add_or_update(client, mode, id, &json_body)
        }
        PullZoneEdgeRuleAction::Delete { zone_id, rule_id } => {
            edge_rule_delete(client, mode, zone_id, &rule_id)
        }
        PullZoneEdgeRuleAction::SetEnabled {
            zone_id,
            rule_id,
            enabled,
        } => edge_rule_set_enabled(client, mode, zone_id, &rule_id, enabled),
    }
}

fn edge_rule_add_or_update(
    client: &Client,
    mode: OutputMode,
    id: i64,
    json_body_path: &str,
) -> Result<()> {
    let body = load_json_body(json_body_path)?;

    let path = format!("/pullzone/{}/edgerules/addOrUpdate", id);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "edge_rule_saved", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "Edge rule added/updated on pull zone {}",
                id
            ));
        }
    }

    Ok(())
}

fn edge_rule_delete(
    client: &Client,
    mode: OutputMode,
    zone_id: i64,
    rule_id: &str,
) -> Result<()> {
    let path = format!("/pullzone/{}/edgerules/{}", zone_id, rule_id);
    let _resp = client.delete(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "edge_rule_deleted", "zone_id": zone_id, "rule_id": rule_id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "Edge rule {} deleted from pull zone {}",
                rule_id, zone_id
            ));
        }
    }

    Ok(())
}

fn edge_rule_set_enabled(
    client: &Client,
    mode: OutputMode,
    zone_id: i64,
    rule_id: &str,
    enabled: bool,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Id".to_string(), serde_json::json!(rule_id));
    body.insert("Value".to_string(), serde_json::json!(enabled));

    let path = format!(
        "/pullzone/{}/edgerules/{}/setEdgeRuleEnabled",
        zone_id, rule_id
    );
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "edge_rule_enabled_set", "zone_id": zone_id, "rule_id": rule_id, "enabled": enabled});
            output::print_json(&json);
        }
        OutputMode::Table => {
            let state = if enabled { "enabled" } else { "disabled" };
            output::print_confirm(&format!(
                "Edge rule {} {} on pull zone {}",
                rule_id, state, zone_id
            ));
        }
    }

    Ok(())
}

// --- Task 21: Referrer + Blocked IP + Stats ---

fn run_referrer(action: PullZoneReferrerAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        PullZoneReferrerAction::AddAllowed { id, hostname } => {
            referrer_action(client, mode, id, &hostname, "addAllowedReferrer", "allowed referrer added")
        }
        PullZoneReferrerAction::RemoveAllowed { id, hostname } => {
            referrer_action(client, mode, id, &hostname, "removeAllowedReferrer", "allowed referrer removed")
        }
        PullZoneReferrerAction::AddBlocked { id, hostname } => {
            referrer_action(client, mode, id, &hostname, "addBlockedReferrer", "blocked referrer added")
        }
        PullZoneReferrerAction::RemoveBlocked { id, hostname } => {
            referrer_action(client, mode, id, &hostname, "removeBlockedReferrer", "blocked referrer removed")
        }
    }
}

fn referrer_action(
    client: &Client,
    mode: OutputMode,
    id: i64,
    hostname: &str,
    endpoint: &str,
    description: &str,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Hostname".to_string(), serde_json::json!(hostname));

    let path = format!("/pullzone/{}/{}", id, endpoint);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": description, "id": id, "hostname": hostname});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "'{}' {} for pull zone {}",
                hostname, description, id
            ));
        }
    }

    Ok(())
}

fn run_blocked_ip(
    action: PullZoneBlockedIpAction,
    client: &Client,
    mode: OutputMode,
) -> Result<()> {
    match action {
        PullZoneBlockedIpAction::Add { id, ip } => blocked_ip_action(
            client,
            mode,
            id,
            &ip,
            "addBlockedIp",
            "blocked IP added",
        ),
        PullZoneBlockedIpAction::Remove { id, ip } => blocked_ip_action(
            client,
            mode,
            id,
            &ip,
            "removeBlockedIp",
            "blocked IP removed",
        ),
    }
}

fn blocked_ip_action(
    client: &Client,
    mode: OutputMode,
    id: i64,
    ip: &str,
    endpoint: &str,
    description: &str,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("BlockedIp".to_string(), serde_json::json!(ip));

    let path = format!("/pullzone/{}/{}", id, endpoint);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": description, "id": id, "ip": ip});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "'{}' {} for pull zone {}",
                ip, description, id
            ));
        }
    }

    Ok(())
}

fn reset_security_key(
    client: &Client,
    mode: OutputMode,
    id: i64,
    security_key: Option<String>,
) -> Result<()> {
    let path = format!("/pullzone/{}/resetSecurityKey", id);

    if let Some(ref key) = security_key {
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert("SecurityKey".to_string(), serde_json::json!(key));
        let _resp = client.post(&path, &body)?;
    } else {
        let _resp = client.post_no_body(&path)?;
    }

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "security_key_reset", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Security key reset for pull zone {}", id));
        }
    }

    Ok(())
}

fn optimizer_statistics(
    client: &Client,
    mode: OutputMode,
    id: i64,
    date_from: Option<String>,
    date_to: Option<String>,
    hourly: bool,
) -> Result<()> {
    let path = format!("/pullzone/{}/optimizer/statistics", id);
    let hourly_str = "true".to_string();
    let params = build_stats_params(&date_from, &date_to, hourly, &hourly_str);

    let resp = if params.is_empty() {
        client.get(&path)?
    } else {
        client.get_with_params(&path, &params)?
    };

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let stats: OptimizerStatisticsModel = resp.json()?;
            output::print_kv(&[
                ("Pull Zone ID", id.to_string()),
                (
                    "Total Requests Optimized",
                    stats
                        .total_requests_optimized
                        .map(|v| format!("{:.0}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Total Traffic Saved",
                    stats
                        .total_traffic_saved
                        .map(|v| format!("{:.0}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Average Processing Time",
                    stats
                        .average_processing_time
                        .map(|v| format!("{:.2}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Average Compression Ratio",
                    stats
                        .average_compression_ratio
                        .map(|v| format!("{:.2}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Data Points",
                    stats
                        .requests_optimized_chart
                        .as_ref()
                        .map(|m| m.len().to_string())
                        .unwrap_or_else(|| "0".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}

fn origin_shield_statistics(
    client: &Client,
    mode: OutputMode,
    id: i64,
    date_from: Option<String>,
    date_to: Option<String>,
    hourly: bool,
) -> Result<()> {
    let path = format!("/pullzone/{}/originshield/queuestatistics", id);
    let hourly_str = "true".to_string();
    let params = build_stats_params(&date_from, &date_to, hourly, &hourly_str);

    let resp = if params.is_empty() {
        client.get(&path)?
    } else {
        client.get_with_params(&path, &params)?
    };

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let stats: OriginShieldConcurrencyStatisticsModel = resp.json()?;
            let concurrent_points = stats
                .concurrent_requests_chart
                .as_ref()
                .map(|m| m.len())
                .unwrap_or(0);
            let queued_points = stats
                .queued_requests_chart
                .as_ref()
                .map(|m| m.len())
                .unwrap_or(0);

            output::print_kv(&[
                ("Pull Zone ID", id.to_string()),
                (
                    "Concurrent Requests Data Points",
                    concurrent_points.to_string(),
                ),
                ("Queued Requests Data Points", queued_points.to_string()),
            ]);
        }
    }

    Ok(())
}

fn safehop_statistics(
    client: &Client,
    mode: OutputMode,
    id: i64,
    date_from: Option<String>,
    date_to: Option<String>,
    hourly: bool,
) -> Result<()> {
    let path = format!("/pullzone/{}/safehop/statistics", id);
    let hourly_str = "true".to_string();
    let params = build_stats_params(&date_from, &date_to, hourly, &hourly_str);

    let resp = if params.is_empty() {
        client.get(&path)?
    } else {
        client.get_with_params(&path, &params)?
    };

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let stats: SafeHopStatisticsModel = resp.json()?;
            output::print_kv(&[
                ("Pull Zone ID", id.to_string()),
                (
                    "Total Requests Retried",
                    stats
                        .total_requests_retried
                        .map(|v| format!("{:.0}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Total Requests Saved",
                    stats
                        .total_requests_saved
                        .map(|v| format!("{:.0}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Data Points",
                    stats
                        .requests_retried_chart
                        .as_ref()
                        .map(|m| m.len().to_string())
                        .unwrap_or_else(|| "0".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}
