use anyhow::Result;
use clap::Subcommand;
use std::collections::HashMap;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::storage_zone::{StorageZone, StorageZoneRow, StorageZoneStatistics};

#[derive(Subcommand)]
pub enum StorageZoneAction {
    /// List storage zones
    List {
        /// Search query
        #[arg(long)]
        search: Option<String>,
        /// Include deleted storage zones
        #[arg(long)]
        include_deleted: bool,
    },
    /// Get a storage zone by ID
    Get {
        /// Storage zone ID
        id: i64,
    },
    /// Create a new storage zone
    Create {
        /// Storage zone name
        name: String,
        /// Main region code (DE, NY, LA, SG)
        #[arg(long)]
        region: String,
        /// Replication regions (comma-separated, e.g. NY,LA)
        #[arg(long)]
        replication_regions: Option<String>,
        /// Zone tier (0=Standard, 1=Edge)
        #[arg(long)]
        zone_tier: Option<i32>,
        /// S3 support type (0=NotSupported, 1=Supported)
        #[arg(long)]
        storage_zone_type: Option<i32>,
    },
    /// Update a storage zone
    Update {
        /// Storage zone ID
        id: i64,
        /// Origin URL
        #[arg(long)]
        origin_url: Option<String>,
        /// Custom 404 file path
        #[arg(long)]
        custom_404_file_path: Option<String>,
        /// Rewrite 404 to 200 for extensionless URLs
        #[arg(long)]
        rewrite_404_to_200: Option<bool>,
        /// Replication zones (comma-separated)
        #[arg(long)]
        replication_zones: Option<String>,
    },
    /// Delete a storage zone
    Delete {
        /// Storage zone ID
        id: i64,
        /// Also delete linked pull zones
        #[arg(long)]
        delete_linked_pull_zones: bool,
    },
    /// Check if a storage zone name is available
    CheckAvailability {
        /// Name to check
        name: String,
    },
    /// Reset storage zone password
    ResetPassword {
        /// Storage zone ID
        id: i64,
    },
    /// Reset storage zone read-only password
    #[command(name = "reset-read-only-password")]
    ResetReadOnlyPassword {
        /// Storage zone ID
        #[arg(long)]
        id: i64,
    },
    /// View storage zone statistics
    Statistics {
        /// Storage zone ID
        id: i64,
        /// Start date for statistics range
        #[arg(long)]
        date_from: Option<String>,
        /// End date for statistics range
        #[arg(long)]
        date_to: Option<String>,
    },
}

pub fn run(action: StorageZoneAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        StorageZoneAction::List {
            search,
            include_deleted,
        } => list(client, mode, search, include_deleted),
        StorageZoneAction::Get { id } => get(client, mode, id),
        StorageZoneAction::Create {
            name,
            region,
            replication_regions,
            zone_tier,
            storage_zone_type,
        } => create(
            client,
            mode,
            &name,
            &region,
            replication_regions,
            zone_tier,
            storage_zone_type,
        ),
        StorageZoneAction::Update {
            id,
            origin_url,
            custom_404_file_path,
            rewrite_404_to_200,
            replication_zones,
        } => update(
            client,
            mode,
            id,
            origin_url,
            custom_404_file_path,
            rewrite_404_to_200,
            replication_zones,
        ),
        StorageZoneAction::Delete {
            id,
            delete_linked_pull_zones,
        } => delete(client, mode, id, delete_linked_pull_zones),
        StorageZoneAction::CheckAvailability { name } => check_availability(client, mode, &name),
        StorageZoneAction::ResetPassword { id } => reset_password(client, mode, id),
        StorageZoneAction::ResetReadOnlyPassword { id } => {
            reset_read_only_password(client, mode, id)
        }
        StorageZoneAction::Statistics {
            id,
            date_from,
            date_to,
        } => statistics(client, mode, id, date_from, date_to),
    }
}

fn list(
    client: &Client,
    mode: OutputMode,
    search: Option<String>,
    include_deleted: bool,
) -> Result<()> {
    let mut params: Vec<(&str, &str)> = Vec::new();
    if let Some(ref s) = search {
        params.push(("search", s.as_str()));
    }
    if include_deleted {
        params.push(("includeDeleted", "true"));
    }

    let items: Vec<StorageZone> = client.fetch_all_pages("/storagezone", &params)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::to_value(&items)?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let rows: Vec<StorageZoneRow> = items.iter().map(StorageZoneRow::from).collect();
            output::print_table(&rows);
        }
    }

    Ok(())
}

fn get(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/storagezone/{}", id);
    let resp = client.get(&path)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let sz: StorageZone = resp.json()?;
            output::print_kv(&[
                ("ID", sz.id.to_string()),
                ("Name", sz.name.clone().unwrap_or_else(|| "-".to_string())),
                (
                    "Region",
                    sz.region.clone().unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Storage Used",
                    sz.storage_used
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Files Stored",
                    sz.files_stored
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Hostname",
                    sz.storage_hostname
                        .clone()
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Tier",
                    sz.zone_tier
                        .map(|v| match v {
                            0 => "Standard".to_string(),
                            1 => "Edge".to_string(),
                            other => other.to_string(),
                        })
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Replication Regions",
                    sz.replication_regions
                        .as_ref()
                        .map(|v| v.join(", "))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Deleted",
                    sz.deleted
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}

fn create(
    client: &Client,
    mode: OutputMode,
    name: &str,
    region: &str,
    replication_regions: Option<String>,
    zone_tier: Option<i32>,
    storage_zone_type: Option<i32>,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Name".to_string(), serde_json::json!(name));
    body.insert("Region".to_string(), serde_json::json!(region));

    if let Some(rr) = replication_regions {
        let regions: Vec<&str> = rr.split(',').map(|s| s.trim()).collect();
        body.insert("ReplicationRegions".to_string(), serde_json::json!(regions));
    }
    if let Some(zt) = zone_tier {
        body.insert("ZoneTier".to_string(), serde_json::json!(zt));
    }
    if let Some(szt) = storage_zone_type {
        body.insert("StorageZoneType".to_string(), serde_json::json!(szt));
    }

    let resp = client.post("/storagezone", &body)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let sz: StorageZone = resp.json()?;
            output::print_confirm(&format!("Storage zone '{}' created (ID: {})", name, sz.id));
        }
    }

    Ok(())
}

fn update(
    client: &Client,
    mode: OutputMode,
    id: i64,
    origin_url: Option<String>,
    custom_404_file_path: Option<String>,
    rewrite_404_to_200: Option<bool>,
    replication_zones: Option<String>,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();

    if let Some(url) = origin_url {
        body.insert("OriginUrl".to_string(), serde_json::json!(url));
    }
    if let Some(path) = custom_404_file_path {
        body.insert("Custom404FilePath".to_string(), serde_json::json!(path));
    }
    if let Some(rewrite) = rewrite_404_to_200 {
        body.insert("Rewrite404To200".to_string(), serde_json::json!(rewrite));
    }
    if let Some(rz) = replication_zones {
        let zones: Vec<&str> = rz.split(',').map(|s| s.trim()).collect();
        body.insert("ReplicationZones".to_string(), serde_json::json!(zones));
    }

    let path = format!("/storagezone/{}", id);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "updated", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Storage zone {} updated", id));
        }
    }

    Ok(())
}

fn delete(
    client: &Client,
    mode: OutputMode,
    id: i64,
    delete_linked_pull_zones: bool,
) -> Result<()> {
    let path = if delete_linked_pull_zones {
        format!("/storagezone/{}?deleteLinkedPullZones=true", id)
    } else {
        format!("/storagezone/{}", id)
    };

    let _resp = client.delete(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "deleted", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Storage zone {} deleted", id));
        }
    }

    Ok(())
}

fn check_availability(client: &Client, mode: OutputMode, name: &str) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Name".to_string(), serde_json::json!(name));

    let _resp = client.post("/storagezone/checkavailability", &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"available": true, "name": name});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Storage zone name '{}' is available", name));
        }
    }

    Ok(())
}

fn reset_password(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/storagezone/{}/resetPassword", id);
    let _resp = client.post_no_body(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "password_reset", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Password reset for storage zone {}", id));
        }
    }

    Ok(())
}

fn reset_read_only_password(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let id_str = id.to_string();
    let params: Vec<(&str, &str)> = vec![("id", &id_str)];
    let _resp = client.post_with_params("/storagezone/resetReadOnlyPassword", &params)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "read_only_password_reset", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Read-only password reset for storage zone {}", id));
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
    let path = format!("/storagezone/{}/statistics", id);
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
            let stats: StorageZoneStatistics = resp.json()?;
            let storage_entries = stats
                .storage_used_chart
                .as_ref()
                .map(|m| m.len())
                .unwrap_or(0);
            let file_entries = stats
                .file_count_chart
                .as_ref()
                .map(|m| m.len())
                .unwrap_or(0);
            let latest_storage = stats
                .storage_used_chart
                .as_ref()
                .and_then(|m| m.values().max().copied())
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string());
            let latest_files = stats
                .file_count_chart
                .as_ref()
                .and_then(|m| m.values().max().copied())
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string());

            output::print_kv(&[
                ("Storage Zone ID", id.to_string()),
                ("Storage Data Points", storage_entries.to_string()),
                ("File Count Data Points", file_entries.to_string()),
                ("Peak Storage Used", latest_storage),
                ("Peak File Count", latest_files),
            ]);
        }
    }

    Ok(())
}
