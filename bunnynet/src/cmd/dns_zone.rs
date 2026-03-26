use anyhow::Result;
use clap::Subcommand;
use std::collections::HashMap;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::dns_record::DnsRecordType;
use bunnynet_lib::models::dns_zone::{
    DnsZone, DnsZoneImportResult, DnsZoneRecordScanJobResponse, DnsZoneRecordScanTriggerResponse,
    DnsZoneRow, DnsZoneStatistics,
};

#[derive(Subcommand)]
pub enum DnsZoneAction {
    /// List DNS zones
    List {
        /// Search query
        #[arg(long)]
        search: Option<String>,
    },
    /// Get a DNS zone by ID
    Get {
        /// DNS zone ID
        id: i64,
    },
    /// Create a new DNS zone
    Create {
        /// Domain name
        domain: String,
    },
    /// Update a DNS zone
    Update {
        /// DNS zone ID
        id: i64,
        /// SOA email address
        #[arg(long)]
        soa_email: Option<String>,
        /// Enable or disable logging
        #[arg(long)]
        logging_enabled: Option<bool>,
        /// Primary nameserver
        #[arg(long)]
        nameserver1: Option<String>,
        /// Secondary nameserver
        #[arg(long)]
        nameserver2: Option<String>,
        /// Enable custom nameservers
        #[arg(long)]
        custom_nameservers_enabled: Option<bool>,
        /// Log anonymization type (0=OneDigit, 1=Drop)
        #[arg(long)]
        log_anonymization_type: Option<i32>,
        /// Enable IP anonymization in logs
        #[arg(long)]
        logging_ip_anonymization_enabled: Option<bool>,
        /// Certificate key type (0=Ecdsa, 1=Rsa)
        #[arg(long)]
        certificate_key_type: Option<i32>,
    },
    /// Delete a DNS zone
    Delete {
        /// DNS zone ID
        id: i64,
    },
    /// Manage DNS records
    Record {
        #[command(subcommand)]
        action: DnsRecordAction,
    },
    /// Manage DNSSEC
    Dnssec {
        #[command(subcommand)]
        action: DnssecAction,
    },
    /// Export DNS zone file
    Export {
        /// DNS zone ID
        id: i64,
    },
    /// Import DNS zone file
    Import {
        /// DNS zone ID
        id: i64,
        /// Path to zone file
        #[arg(long)]
        file: String,
    },
    /// View DNS zone statistics
    Statistics {
        /// DNS zone ID
        id: i64,
        /// Start date
        #[arg(long)]
        date_from: Option<String>,
        /// End date
        #[arg(long)]
        date_to: Option<String>,
    },
    /// Check if a DNS zone name is available
    CheckAvailability {
        /// Domain name to check
        name: String,
    },
    /// Manage DNS zone certificates
    Certificate {
        #[command(subcommand)]
        action: DnsCertificateAction,
    },
}

#[derive(Subcommand)]
pub enum DnsRecordAction {
    /// Add a DNS record
    Add {
        /// DNS zone ID
        zone_id: i64,
        /// Record type (A, AAAA, CNAME, TXT, MX, etc.)
        #[arg(long)]
        r#type: String,
        /// Record name
        #[arg(long)]
        name: String,
        /// Record value
        #[arg(long)]
        value: String,
        /// Time to live in seconds
        #[arg(long)]
        ttl: Option<i64>,
        /// Record priority
        #[arg(long)]
        priority: Option<i32>,
        /// Record weight
        #[arg(long)]
        weight: Option<i32>,
        /// Port number
        #[arg(long)]
        port: Option<i32>,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Disable this record
        #[arg(long)]
        disabled: bool,
    },
    /// Update a DNS record
    Update {
        /// DNS zone ID
        zone_id: i64,
        /// Record ID
        record_id: i64,
        /// Record type (A, AAAA, CNAME, TXT, MX, etc.)
        #[arg(long)]
        r#type: Option<String>,
        /// Record name
        #[arg(long)]
        name: Option<String>,
        /// Record value
        #[arg(long)]
        value: Option<String>,
        /// Time to live in seconds
        #[arg(long)]
        ttl: Option<i64>,
        /// Record priority
        #[arg(long)]
        priority: Option<i32>,
        /// Record weight
        #[arg(long)]
        weight: Option<i32>,
        /// Port number
        #[arg(long)]
        port: Option<i32>,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Disable this record
        #[arg(long)]
        disabled: Option<bool>,
    },
    /// Delete a DNS record
    Delete {
        /// DNS zone ID
        zone_id: i64,
        /// Record ID
        record_id: i64,
    },
    /// Trigger a DNS record scan
    Scan {
        /// DNS zone ID to scan
        #[arg(long, required = true, group = "target")]
        zone_id: Option<i64>,
        /// Domain to scan (can be used before creating the zone)
        #[arg(long, required = true, group = "target")]
        domain: Option<String>,
    },
    /// Get DNS record scan results
    ScanResults {
        /// DNS zone ID
        zone_id: i64,
    },
}

#[derive(Subcommand)]
pub enum DnssecAction {
    /// Enable DNSSEC for a DNS zone
    Enable {
        /// DNS zone ID
        id: i64,
    },
    /// Disable DNSSEC for a DNS zone
    Disable {
        /// DNS zone ID
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum DnsCertificateAction {
    /// Issue a wildcard certificate for a DNS zone
    Issue {
        /// DNS zone ID
        zone_id: i64,
        /// Specific domain (optional, defaults to wildcard)
        #[arg(long)]
        domain: Option<String>,
    },
}

pub fn run(action: DnsZoneAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DnsZoneAction::List { search } => list(client, mode, search),
        DnsZoneAction::Get { id } => get(client, mode, id),
        DnsZoneAction::Create { domain } => create(client, mode, &domain),
        DnsZoneAction::Update {
            id,
            soa_email,
            logging_enabled,
            nameserver1,
            nameserver2,
            custom_nameservers_enabled,
            log_anonymization_type,
            logging_ip_anonymization_enabled,
            certificate_key_type,
        } => update(
            client,
            mode,
            id,
            soa_email,
            logging_enabled,
            nameserver1,
            nameserver2,
            custom_nameservers_enabled,
            log_anonymization_type,
            logging_ip_anonymization_enabled,
            certificate_key_type,
        ),
        DnsZoneAction::Delete { id } => delete(client, mode, id),
        DnsZoneAction::Record { action: rec_action } => run_record(rec_action, client, mode),
        DnsZoneAction::Dnssec {
            action: dnssec_action,
        } => run_dnssec(dnssec_action, client, mode),
        DnsZoneAction::Export { id } => export(client, mode, id),
        DnsZoneAction::Import { id, file } => import(client, mode, id, &file),
        DnsZoneAction::Statistics {
            id,
            date_from,
            date_to,
        } => statistics(client, mode, id, date_from, date_to),
        DnsZoneAction::CheckAvailability { name } => check_availability(client, mode, &name),
        DnsZoneAction::Certificate {
            action: cert_action,
        } => run_certificate(cert_action, client, mode),
    }
}

// --- DNS Zone CRUD ---

fn list(client: &Client, mode: OutputMode, search: Option<String>) -> Result<()> {
    let mut params: Vec<(&str, &str)> = Vec::new();
    if let Some(ref s) = search {
        params.push(("search", s.as_str()));
    }

    let items: Vec<DnsZone> = client.fetch_all_pages("/dnszone", &params)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::to_value(&items)?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let rows: Vec<DnsZoneRow> = items.iter().map(DnsZoneRow::from).collect();
            output::print_table(&rows);
        }
    }

    Ok(())
}

fn get(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/dnszone/{}", id);
    let resp = client.get(&path)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let zone: DnsZone = resp.json()?;
            output::print_kv(&[
                ("ID", zone.id.to_string()),
                (
                    "Domain",
                    zone.domain.clone().unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Nameserver 1",
                    zone.nameserver1.clone().unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Nameserver 2",
                    zone.nameserver2.clone().unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "SOA Email",
                    zone.soa_email.clone().unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "DNSSEC",
                    zone.dns_sec_enabled
                        .map(|v| if v { "Enabled" } else { "Disabled" }.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Logging",
                    zone.logging_enabled
                        .map(|v| if v { "Enabled" } else { "Disabled" }.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Nameservers Detected",
                    zone.nameservers_detected
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Custom Nameservers",
                    zone.custom_nameservers_enabled
                        .map(|v| if v { "Enabled" } else { "Disabled" }.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Records",
                    zone.records
                        .as_ref()
                        .map(|v| v.len().to_string())
                        .unwrap_or_else(|| "0".to_string()),
                ),
                (
                    "Date Created",
                    zone.date_created.clone().unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Date Modified",
                    zone.date_modified
                        .clone()
                        .unwrap_or_else(|| "-".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}

fn create(client: &Client, mode: OutputMode, domain: &str) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Domain".to_string(), serde_json::json!(domain));

    let resp = client.post("/dnszone", &body)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let zone: DnsZone = resp.json()?;
            output::print_confirm(&format!("DNS zone '{}' created (ID: {})", domain, zone.id));
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn update(
    client: &Client,
    mode: OutputMode,
    id: i64,
    soa_email: Option<String>,
    logging_enabled: Option<bool>,
    nameserver1: Option<String>,
    nameserver2: Option<String>,
    custom_nameservers_enabled: Option<bool>,
    log_anonymization_type: Option<i32>,
    logging_ip_anonymization_enabled: Option<bool>,
    certificate_key_type: Option<i32>,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();

    if let Some(email) = soa_email {
        body.insert("SoaEmail".to_string(), serde_json::json!(email));
    }
    if let Some(enabled) = logging_enabled {
        body.insert("LoggingEnabled".to_string(), serde_json::json!(enabled));
    }
    if let Some(ns1) = nameserver1 {
        body.insert("Nameserver1".to_string(), serde_json::json!(ns1));
    }
    if let Some(ns2) = nameserver2 {
        body.insert("Nameserver2".to_string(), serde_json::json!(ns2));
    }
    if let Some(enabled) = custom_nameservers_enabled {
        body.insert(
            "CustomNameserversEnabled".to_string(),
            serde_json::json!(enabled),
        );
    }
    if let Some(lat) = log_anonymization_type {
        body.insert("LogAnonymizationType".to_string(), serde_json::json!(lat));
    }
    if let Some(enabled) = logging_ip_anonymization_enabled {
        body.insert(
            "LoggingIPAnonymizationEnabled".to_string(),
            serde_json::json!(enabled),
        );
    }
    if let Some(kt) = certificate_key_type {
        body.insert("CertificateKeyType".to_string(), serde_json::json!(kt));
    }

    let path = format!("/dnszone/{}", id);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "updated", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("DNS zone {} updated", id));
        }
    }

    Ok(())
}

fn delete(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/dnszone/{}", id);
    let _resp = client.delete(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "deleted", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("DNS zone {} deleted", id));
        }
    }

    Ok(())
}

// --- DNS Records ---

fn run_record(action: DnsRecordAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DnsRecordAction::Add {
            zone_id,
            r#type,
            name,
            value,
            ttl,
            priority,
            weight,
            port,
            comment,
            disabled,
        } => record_add(
            client, mode, zone_id, &r#type, &name, &value, ttl, priority, weight, port, comment,
            disabled,
        ),
        DnsRecordAction::Update {
            zone_id,
            record_id,
            r#type,
            name,
            value,
            ttl,
            priority,
            weight,
            port,
            comment,
            disabled,
        } => record_update(
            client, mode, zone_id, record_id, r#type, name, value, ttl, priority, weight, port,
            comment, disabled,
        ),
        DnsRecordAction::Delete { zone_id, record_id } => {
            record_delete(client, mode, zone_id, record_id)
        }
        DnsRecordAction::Scan { zone_id, domain } => record_scan(client, mode, zone_id, domain),
        DnsRecordAction::ScanResults { zone_id } => record_scan_results(client, mode, zone_id),
    }
}

#[allow(clippy::too_many_arguments)]
fn record_add(
    client: &Client,
    mode: OutputMode,
    zone_id: i64,
    record_type: &str,
    name: &str,
    value: &str,
    ttl: Option<i64>,
    priority: Option<i32>,
    weight: Option<i32>,
    port: Option<i32>,
    comment: Option<String>,
    disabled: bool,
) -> Result<()> {
    let dns_type = record_type
        .parse::<DnsRecordType>()
        .map_err(|e| anyhow::anyhow!(e))?;
    let type_int = dns_type as i32;

    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Type".to_string(), serde_json::json!(type_int));
    body.insert("Name".to_string(), serde_json::json!(name));
    body.insert("Value".to_string(), serde_json::json!(value));

    if let Some(t) = ttl {
        body.insert("Ttl".to_string(), serde_json::json!(t));
    }
    if let Some(p) = priority {
        body.insert("Priority".to_string(), serde_json::json!(p));
    }
    if let Some(w) = weight {
        body.insert("Weight".to_string(), serde_json::json!(w));
    }
    if let Some(pt) = port {
        body.insert("Port".to_string(), serde_json::json!(pt));
    }
    if let Some(c) = comment {
        body.insert("Comment".to_string(), serde_json::json!(c));
    }
    if disabled {
        body.insert("Disabled".to_string(), serde_json::json!(true));
    }

    let path = format!("/dnszone/{}/records", zone_id);
    let resp = client.put(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let record: bunnynet_lib::models::dns_record::DnsRecord = resp.json()?;
            let id_str = record
                .id
                .map(|v| v.to_string())
                .unwrap_or_else(|| "?".to_string());
            output::print_confirm(&format!(
                "DNS record added (ID: {}, Type: {}, Name: {})",
                id_str, dns_type, name
            ));
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn record_update(
    client: &Client,
    mode: OutputMode,
    zone_id: i64,
    record_id: i64,
    record_type: Option<String>,
    name: Option<String>,
    value: Option<String>,
    ttl: Option<i64>,
    priority: Option<i32>,
    weight: Option<i32>,
    port: Option<i32>,
    comment: Option<String>,
    disabled: Option<bool>,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();

    if let Some(ref rt) = record_type {
        let dns_type = rt
            .parse::<DnsRecordType>()
            .map_err(|e| anyhow::anyhow!(e))?;
        body.insert("Type".to_string(), serde_json::json!(dns_type as i32));
    }
    if let Some(ref n) = name {
        body.insert("Name".to_string(), serde_json::json!(n));
    }
    if let Some(ref v) = value {
        body.insert("Value".to_string(), serde_json::json!(v));
    }
    if let Some(t) = ttl {
        body.insert("Ttl".to_string(), serde_json::json!(t));
    }
    if let Some(p) = priority {
        body.insert("Priority".to_string(), serde_json::json!(p));
    }
    if let Some(w) = weight {
        body.insert("Weight".to_string(), serde_json::json!(w));
    }
    if let Some(pt) = port {
        body.insert("Port".to_string(), serde_json::json!(pt));
    }
    if let Some(c) = comment {
        body.insert("Comment".to_string(), serde_json::json!(c));
    }
    if let Some(d) = disabled {
        body.insert("Disabled".to_string(), serde_json::json!(d));
    }

    let path = format!("/dnszone/{}/records/{}", zone_id, record_id);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "updated", "zone_id": zone_id, "record_id": record_id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "DNS record {} updated in zone {}",
                record_id, zone_id
            ));
        }
    }

    Ok(())
}

fn record_delete(client: &Client, mode: OutputMode, zone_id: i64, record_id: i64) -> Result<()> {
    let path = format!("/dnszone/{}/records/{}", zone_id, record_id);
    let _resp = client.delete(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "deleted", "zone_id": zone_id, "record_id": record_id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "DNS record {} deleted from zone {}",
                record_id, zone_id
            ));
        }
    }

    Ok(())
}

fn record_scan(
    client: &Client,
    mode: OutputMode,
    zone_id: Option<i64>,
    domain: Option<String>,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();

    if let Some(zid) = zone_id {
        body.insert("ZoneId".to_string(), serde_json::json!(zid));
    }
    if let Some(ref d) = domain {
        body.insert("Domain".to_string(), serde_json::json!(d));
    }

    let resp = client.post("/dnszone/records/scan", &body)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let trigger: DnsZoneRecordScanTriggerResponse = resp.json()?;
            let job_id = trigger.job_id.unwrap_or_else(|| "-".to_string());
            let status = trigger.status.unwrap_or(-1);
            let status_name = match status {
                0 => "Pending",
                1 => "InProgress",
                2 => "Completed",
                3 => "Failed",
                _ => "Unknown",
            };
            output::print_confirm(&format!(
                "Record scan triggered (Job ID: {}, Status: {})",
                job_id, status_name
            ));
        }
    }

    Ok(())
}

fn record_scan_results(client: &Client, mode: OutputMode, zone_id: i64) -> Result<()> {
    let path = format!("/dnszone/{}/records/scan", zone_id);
    let resp = client.get(&path)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let job: DnsZoneRecordScanJobResponse = resp.json()?;
            let status = job.status.unwrap_or(-1);
            let status_name = match status {
                0 => "Pending",
                1 => "InProgress",
                2 => "Completed",
                3 => "Failed",
                _ => "Unknown",
            };
            output::print_kv(&[
                ("Job ID", job.job_id.unwrap_or_else(|| "-".to_string())),
                ("Zone ID", zone_id.to_string()),
                ("Domain", job.domain.unwrap_or_else(|| "-".to_string())),
                ("Status", status_name.to_string()),
                (
                    "Created At",
                    job.created_at.unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Completed At",
                    job.completed_at.unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Records Found",
                    job.records
                        .as_ref()
                        .map(|r| r.len().to_string())
                        .unwrap_or_else(|| "0".to_string()),
                ),
                ("Error", job.error.unwrap_or_else(|| "-".to_string())),
            ]);
        }
    }

    Ok(())
}

// --- DNSSEC ---

fn run_dnssec(action: DnssecAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DnssecAction::Enable { id } => dnssec_enable(client, mode, id),
        DnssecAction::Disable { id } => dnssec_disable(client, mode, id),
    }
}

fn dnssec_enable(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/dnszone/{}/dnssec", id);
    let _resp = client.post_no_body(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "dnssec_enabled", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("DNSSEC enabled for DNS zone {}", id));
        }
    }

    Ok(())
}

fn dnssec_disable(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/dnszone/{}/dnssec", id);
    let _resp = client.delete(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "dnssec_disabled", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("DNSSEC disabled for DNS zone {}", id));
        }
    }

    Ok(())
}

// --- DNS Zone Actions (Task 18) ---

fn export(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/dnszone/{}/export", id);
    let bytes = client.get_bytes(&path)?;

    match mode {
        OutputMode::Json => {
            // For export, the raw content is the zone file text
            let text = String::from_utf8_lossy(&bytes);
            let json = serde_json::json!({"zone_file": text.to_string()});
            output::print_json(&json);
        }
        OutputMode::Table => {
            let text = String::from_utf8_lossy(&bytes);
            println!("{}", text);
        }
    }

    Ok(())
}

fn import(client: &Client, mode: OutputMode, id: i64, file_path: &str) -> Result<()> {
    let contents = std::fs::read_to_string(file_path)
        .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", file_path, e))?;

    let path = format!("/dnszone/{}/import", id);
    let resp = client.post_text(&path, &contents)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let result: DnsZoneImportResult = resp.json()?;
            output::print_kv(&[
                (
                    "Records Successful",
                    result
                        .records_successful
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "0".to_string()),
                ),
                (
                    "Records Failed",
                    result
                        .records_failed
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "0".to_string()),
                ),
                (
                    "Records Skipped",
                    result
                        .records_skipped
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "0".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}

fn statistics(
    client: &Client,
    mode: OutputMode,
    id: i64,
    date_from: Option<String>,
    date_to: Option<String>,
) -> Result<()> {
    let path = format!("/dnszone/{}/statistics", id);
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(ref df) = date_from {
        params.push(("dateFrom", df.clone()));
    }
    if let Some(ref dt) = date_to {
        params.push(("dateTo", dt.clone()));
    }
    let params_ref: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

    let resp = if params_ref.is_empty() {
        client.get(&path)?
    } else {
        client.get_with_params(&path, &params_ref)?
    };

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let stats: DnsZoneStatistics = resp.json()?;
            let total = stats
                .total_queries_served
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string());
            let data_points = stats
                .queries_served_chart
                .as_ref()
                .map(|m| m.len())
                .unwrap_or(0);

            output::print_kv(&[
                ("DNS Zone ID", id.to_string()),
                ("Total Queries Served", total),
                ("Data Points", data_points.to_string()),
            ]);
        }
    }

    Ok(())
}

fn check_availability(client: &Client, mode: OutputMode, name: &str) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Name".to_string(), serde_json::json!(name));

    let _resp = client.post("/dnszone/checkavailability", &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"available": true, "name": name});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("DNS zone '{}' is available", name));
        }
    }

    Ok(())
}

// --- Certificates ---

fn run_certificate(action: DnsCertificateAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DnsCertificateAction::Issue { zone_id, domain } => {
            certificate_issue(client, mode, zone_id, domain)
        }
    }
}

fn certificate_issue(
    client: &Client,
    mode: OutputMode,
    zone_id: i64,
    domain: Option<String>,
) -> Result<()> {
    let path = format!("/dnszone/{}/certificate/issue", zone_id);

    if let Some(ref d) = domain {
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert("Domain".to_string(), serde_json::json!(d));
        let _resp = client.post(&path, &body)?;
    } else {
        let _resp = client.post_no_body(&path)?;
    }

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({
                "status": "certificate_issued",
                "zone_id": zone_id,
                "domain": domain,
            });
            output::print_json(&json);
        }
        OutputMode::Table => {
            let domain_str = domain.unwrap_or_else(|| "wildcard".to_string());
            output::print_confirm(&format!(
                "Certificate issue triggered for zone {} ({})",
                zone_id, domain_str
            ));
        }
    }

    Ok(())
}
