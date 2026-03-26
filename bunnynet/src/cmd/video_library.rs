use anyhow::{bail, Context, Result};
use clap::Subcommand;
use std::collections::HashMap;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::pagination::PaginatedList;
use bunnynet_lib::models::video_library::{
    VideoLibrary, VideoLibraryDrmStatisticsModel, VideoLibraryLanguage, VideoLibraryLanguageRow,
    VideoLibraryRow, VideoLibraryTranscriptionStatisticsModel,
};

// --- Subcommand definitions ---

#[derive(Subcommand)]
pub enum VideoLibraryAction {
    /// List video libraries
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
    },
    /// Get a video library by ID
    Get {
        /// Video library ID
        id: i64,
    },
    /// Create a new video library
    Create {
        /// Video library name
        name: String,
        /// Comma-separated replication regions (e.g. DE,NY,SG)
        #[arg(long)]
        replication_regions: Option<String>,
        /// Player version
        #[arg(long)]
        player_version: Option<i32>,
    },
    /// Update a video library
    Update {
        /// Video library ID
        id: i64,
        /// Path to JSON file with settings
        #[arg(long)]
        json_body: Option<String>,
    },
    /// Delete a video library
    Delete {
        /// Video library ID
        id: i64,
    },
    /// List available languages
    Languages,
    /// Add an allowed referrer
    AddAllowedReferrer {
        /// Video library ID
        id: i64,
        /// Referrer hostname
        #[arg(long)]
        hostname: String,
    },
    /// Remove an allowed referrer
    RemoveAllowedReferrer {
        /// Video library ID
        id: i64,
        /// Referrer hostname
        #[arg(long)]
        hostname: String,
    },
    /// Add a blocked referrer
    AddBlockedReferrer {
        /// Video library ID
        id: i64,
        /// Referrer hostname
        #[arg(long)]
        hostname: String,
    },
    /// Remove a blocked referrer
    RemoveBlockedReferrer {
        /// Video library ID
        id: i64,
        /// Referrer hostname
        #[arg(long)]
        hostname: String,
    },
    /// Reset the API key
    ResetApiKey {
        /// Video library ID
        id: i64,
    },
    /// Reset the read-only API key
    ResetReadOnlyApiKey {
        /// Video library ID
        id: i64,
    },
    /// Manage watermark
    Watermark {
        #[command(subcommand)]
        action: WatermarkAction,
    },
    /// Manage live thumbnail
    LiveThumbnail {
        #[command(subcommand)]
        action: LiveThumbnailAction,
    },
    /// Manage live watermark
    LiveWatermark {
        #[command(subcommand)]
        action: LiveWatermarkAction,
    },
    /// View DRM statistics
    DrmStatistics {
        /// Video library ID
        id: i64,
        /// Start date
        #[arg(long)]
        date_from: Option<String>,
        /// End date
        #[arg(long)]
        date_to: Option<String>,
    },
    /// View transcribing statistics
    TranscribingStatistics {
        /// Video library ID
        id: i64,
        /// Start date
        #[arg(long)]
        date_from: Option<String>,
        /// End date
        #[arg(long)]
        date_to: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum WatermarkAction {
    /// Add a watermark (binary file upload)
    Add {
        /// Video library ID
        id: i64,
        /// Path to image file (PNG or JPEG)
        #[arg(long)]
        file: String,
    },
    /// Delete the watermark
    Delete {
        /// Video library ID
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum LiveThumbnailAction {
    /// Add a live thumbnail (binary file upload)
    Add {
        /// Video library ID
        id: i64,
        /// Path to image file (PNG or JPEG)
        #[arg(long)]
        file: String,
    },
    /// Delete the live thumbnail
    Delete {
        /// Video library ID
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum LiveWatermarkAction {
    /// Add a live watermark (binary file upload)
    Add {
        /// Video library ID
        id: i64,
        /// Path to image file (PNG or JPEG)
        #[arg(long)]
        file: String,
    },
    /// Delete the live watermark
    Delete {
        /// Video library ID
        id: i64,
    },
}

// --- Main dispatcher ---

#[allow(clippy::too_many_lines)]
pub fn run(action: VideoLibraryAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        VideoLibraryAction::List {
            search,
            page,
            per_page,
        } => list(client, mode, search, page, per_page),
        VideoLibraryAction::Get { id } => get(client, mode, id),
        VideoLibraryAction::Create {
            name,
            replication_regions,
            player_version,
        } => create(client, mode, &name, replication_regions, player_version),
        VideoLibraryAction::Update { id, json_body } => update(client, mode, id, json_body),
        VideoLibraryAction::Delete { id } => delete(client, mode, id),
        VideoLibraryAction::Languages => languages(client, mode),
        VideoLibraryAction::AddAllowedReferrer { id, hostname } => referrer_action(
            client,
            mode,
            id,
            &hostname,
            "addAllowedReferrer",
            "allowed referrer added",
        ),
        VideoLibraryAction::RemoveAllowedReferrer { id, hostname } => referrer_action(
            client,
            mode,
            id,
            &hostname,
            "removeAllowedReferrer",
            "allowed referrer removed",
        ),
        VideoLibraryAction::AddBlockedReferrer { id, hostname } => referrer_action(
            client,
            mode,
            id,
            &hostname,
            "addBlockedReferrer",
            "blocked referrer added",
        ),
        VideoLibraryAction::RemoveBlockedReferrer { id, hostname } => referrer_action(
            client,
            mode,
            id,
            &hostname,
            "removeBlockedReferrer",
            "blocked referrer removed",
        ),
        VideoLibraryAction::ResetApiKey { id } => reset_api_key(client, mode, id),
        VideoLibraryAction::ResetReadOnlyApiKey { id } => reset_read_only_api_key(client, mode, id),
        VideoLibraryAction::Watermark { action: w_action } => run_watermark(w_action, client, mode),
        VideoLibraryAction::LiveThumbnail { action: lt_action } => {
            run_live_thumbnail(lt_action, client, mode)
        }
        VideoLibraryAction::LiveWatermark { action: lw_action } => {
            run_live_watermark(lw_action, client, mode)
        }
        VideoLibraryAction::DrmStatistics {
            id,
            date_from,
            date_to,
        } => drm_statistics(client, mode, id, date_from, date_to),
        VideoLibraryAction::TranscribingStatistics {
            id,
            date_from,
            date_to,
        } => transcribing_statistics(client, mode, id, date_from, date_to),
    }
}

// --- Shared helpers ---

fn load_json_body(path: &str) -> Result<HashMap<String, serde_json::Value>> {
    let contents =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read {}", path))?;
    let value: serde_json::Value = serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse JSON from {}", path))?;
    match value {
        serde_json::Value::Object(map) => Ok(map.into_iter().collect()),
        _ => bail!("Expected JSON object in {}", path),
    }
}

fn build_stats_params<'a>(
    date_from: &'a Option<String>,
    date_to: &'a Option<String>,
) -> Vec<(&'a str, &'a str)> {
    let mut params: Vec<(&str, &str)> = Vec::new();
    if let Some(ref df) = date_from {
        params.push(("dateFrom", df.as_str()));
    }
    if let Some(ref dt) = date_to {
        params.push(("dateTo", dt.as_str()));
    }
    params
}

fn content_type_from_extension(path: &str) -> &'static str {
    let lower = path.to_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else {
        "application/octet-stream"
    }
}

// --- Task 22: Video Library CRUD + Languages ---

fn list(
    client: &Client,
    mode: OutputMode,
    search: Option<String>,
    page: i32,
    per_page: Option<i32>,
) -> Result<()> {
    let page_str = page.to_string();
    let mut params: Vec<(&str, String)> = vec![("page", page_str)];
    if let Some(ref s) = search {
        params.push(("search", s.clone()));
    }
    if let Some(pp) = per_page {
        params.push(("perPage", pp.to_string()));
    }
    let params_ref: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

    let resp = client.get_with_params("/videolibrary", &params_ref)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let list: PaginatedList<VideoLibrary> = resp.json()?;
            let rows: Vec<VideoLibraryRow> = list.items.iter().map(VideoLibraryRow::from).collect();
            output::print_table(&rows);
            output::print_pagination(list.current_page, list.total_items, list.has_more_items);
        }
    }

    Ok(())
}

fn get(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/videolibrary/{}", id);
    let resp = client.get(&path)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let vl: VideoLibrary = resp.json()?;
            output::print_kv(&[
                ("ID", vl.id.to_string()),
                ("Name", vl.name.clone().unwrap_or_else(|| "-".to_string())),
                (
                    "Video Count",
                    vl.video_count
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Traffic Usage",
                    vl.traffic_usage
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Storage Usage",
                    vl.storage_usage
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Pull Zone ID",
                    vl.pull_zone_id
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Storage Zone ID",
                    vl.storage_zone_id
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "DRM Enabled",
                    vl.enable_drm
                        .map(|e| if e { "Yes" } else { "No" }.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "DRM Version",
                    vl.drm_version
                        .map(|v| format!("{:?}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Encoding Tier",
                    vl.encoding_tier
                        .map(|v| format!("{:?}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Enabled Resolutions",
                    vl.enabled_resolutions
                        .clone()
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "MP4 Fallback",
                    vl.enable_mp4_fallback
                        .map(|e| if e { "Yes" } else { "No" }.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Transcribing",
                    vl.enable_transcribing
                        .map(|e| if e { "Yes" } else { "No" }.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Date Created",
                    vl.date_created.clone().unwrap_or_else(|| "-".to_string()),
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
    replication_regions: Option<String>,
    player_version: Option<i32>,
) -> Result<()> {
    let mut body: HashMap<String, serde_json::Value> = HashMap::new();
    body.insert("Name".to_string(), serde_json::json!(name));

    if let Some(regions) = replication_regions {
        let regions_vec: Vec<&str> = regions.split(',').map(|s| s.trim()).collect();
        body.insert(
            "ReplicationRegions".to_string(),
            serde_json::json!(regions_vec),
        );
    }

    if let Some(pv) = player_version {
        body.insert("PlayerVersion".to_string(), serde_json::json!(pv));
    }

    let resp = client.post("/videolibrary", &body)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let vl: VideoLibrary = resp.json()?;
            output::print_confirm(&format!("Video library '{}' created (ID: {})", name, vl.id));
        }
    }

    Ok(())
}

fn update(client: &Client, mode: OutputMode, id: i64, json_body: Option<String>) -> Result<()> {
    let body: HashMap<String, serde_json::Value> = if let Some(ref path) = json_body {
        load_json_body(path)?
    } else {
        HashMap::new()
    };

    let path = format!("/videolibrary/{}", id);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "updated", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Video library {} updated", id));
        }
    }

    Ok(())
}

fn delete(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/videolibrary/{}", id);
    let _resp = client.delete(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "deleted", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Video library {} deleted", id));
        }
    }

    Ok(())
}

fn languages(client: &Client, mode: OutputMode) -> Result<()> {
    let resp = client.get("/videolibrary/languages")?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let langs: Vec<VideoLibraryLanguage> = resp.json()?;
            let rows: Vec<VideoLibraryLanguageRow> =
                langs.iter().map(VideoLibraryLanguageRow::from).collect();
            output::print_table(&rows);
        }
    }

    Ok(())
}

// --- Task 23: Video Library actions ---

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

    let path = format!("/videolibrary/{}/{}", id, endpoint);
    let _resp = client.post(&path, &body)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": description, "id": id, "hostname": hostname});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "'{}' {} for video library {}",
                hostname, description, id
            ));
        }
    }

    Ok(())
}

fn reset_api_key(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/videolibrary/{}/resetApiKey", id);
    let _resp = client.post_no_body(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "api_key_reset", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("API key reset for video library {}", id));
        }
    }

    Ok(())
}

fn reset_read_only_api_key(client: &Client, mode: OutputMode, id: i64) -> Result<()> {
    let path = format!("/videolibrary/{}/resetReadOnlyApiKey", id);
    let _resp = client.post_no_body(&path)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "read_only_api_key_reset", "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Read-only API key reset for video library {}", id));
        }
    }

    Ok(())
}

// --- Watermark sub-commands ---

fn run_watermark(action: WatermarkAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        WatermarkAction::Add { id, file } => watermark_upload(client, mode, id, &file, "watermark"),
        WatermarkAction::Delete { id } => watermark_delete(client, mode, id, "watermark"),
    }
}

fn run_live_thumbnail(
    action: LiveThumbnailAction,
    client: &Client,
    mode: OutputMode,
) -> Result<()> {
    match action {
        LiveThumbnailAction::Add { id, file } => {
            watermark_upload(client, mode, id, &file, "live/thumbnail")
        }
        LiveThumbnailAction::Delete { id } => watermark_delete(client, mode, id, "live/thumbnail"),
    }
}

fn run_live_watermark(
    action: LiveWatermarkAction,
    client: &Client,
    mode: OutputMode,
) -> Result<()> {
    match action {
        LiveWatermarkAction::Add { id, file } => {
            watermark_upload(client, mode, id, &file, "live/watermark")
        }
        LiveWatermarkAction::Delete { id } => watermark_delete(client, mode, id, "live/watermark"),
    }
}

fn watermark_upload(
    client: &Client,
    mode: OutputMode,
    id: i64,
    file_path: &str,
    resource: &str,
) -> Result<()> {
    let data =
        std::fs::read(file_path).with_context(|| format!("Failed to read file: {}", file_path))?;
    let content_type = content_type_from_extension(file_path);

    let path = format!("/videolibrary/{}/{}", id, resource);
    let _resp = client.put_file(&path, data, content_type)?;

    let label = resource.replace('/', " ");
    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": format!("{}_uploaded", label.replace(' ', "_")), "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "{} uploaded for video library {}",
                capitalize_first(&label),
                id
            ));
        }
    }

    Ok(())
}

fn watermark_delete(client: &Client, mode: OutputMode, id: i64, resource: &str) -> Result<()> {
    let path = format!("/videolibrary/{}/{}", id, resource);
    let _resp = client.delete(&path)?;

    let label = resource.replace('/', " ");
    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": format!("{}_deleted", label.replace(' ', "_")), "id": id});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!(
                "{} deleted for video library {}",
                capitalize_first(&label),
                id
            ));
        }
    }

    Ok(())
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

// --- Statistics ---

fn drm_statistics(
    client: &Client,
    mode: OutputMode,
    id: i64,
    date_from: Option<String>,
    date_to: Option<String>,
) -> Result<()> {
    let path = format!("/videolibrary/{}/drm/statistics", id);
    let params = build_stats_params(&date_from, &date_to);

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
            let stats: VideoLibraryDrmStatisticsModel = resp.json()?;
            output::print_kv(&[
                ("Video Library ID", id.to_string()),
                (
                    "Total Licenses Issued",
                    stats
                        .total_licenses_issued
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Data Points",
                    stats
                        .licenses_issued_chart
                        .as_ref()
                        .map(|m| m.len().to_string())
                        .unwrap_or_else(|| "0".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}

fn transcribing_statistics(
    client: &Client,
    mode: OutputMode,
    id: i64,
    date_from: Option<String>,
    date_to: Option<String>,
) -> Result<()> {
    let path = format!("/videolibrary/{}/transcribing/statistics", id);
    let params = build_stats_params(&date_from, &date_to);

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
            let stats: VideoLibraryTranscriptionStatisticsModel = resp.json()?;
            output::print_kv(&[
                ("Video Library ID", id.to_string()),
                (
                    "Total Transcription Seconds",
                    stats
                        .total_transcription_seconds
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Data Points",
                    stats
                        .transcription_seconds_chart
                        .as_ref()
                        .map(|m| m.len().to_string())
                        .unwrap_or_else(|| "0".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}
