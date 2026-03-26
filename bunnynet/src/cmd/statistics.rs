use anyhow::Result;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::statistics::Statistics;

#[allow(clippy::too_many_arguments)]
pub fn run(
    client: &Client,
    mode: OutputMode,
    date_from: Option<String>,
    date_to: Option<String>,
    pull_zone: Option<i64>,
    server_zone_id: Option<i64>,
    hourly: bool,
    load_errors: bool,
    load_origin_response_times: bool,
    load_requests_served: bool,
    load_bandwidth_used: bool,
    load_origin_traffic: bool,
    load_origin_shield_bandwidth: bool,
    load_geographic_traffic_distribution: bool,
    load_user_balance_history: bool,
) -> Result<()> {
    let mut params: Vec<(&str, String)> = Vec::new();

    if let Some(ref df) = date_from {
        params.push(("dateFrom", df.clone()));
    }
    if let Some(ref dt) = date_to {
        params.push(("dateTo", dt.clone()));
    }
    if let Some(pz) = pull_zone {
        params.push(("pullZone", pz.to_string()));
    }
    if let Some(sz) = server_zone_id {
        params.push(("serverZoneId", sz.to_string()));
    }
    if hourly {
        params.push(("hourly", "true".to_string()));
    }
    if load_errors {
        params.push(("loadErrors", "true".to_string()));
    }
    if load_origin_response_times {
        params.push(("loadOriginResponseTimes", "true".to_string()));
    }
    if load_requests_served {
        params.push(("loadRequestsServed", "true".to_string()));
    }
    if load_bandwidth_used {
        params.push(("loadBandwidthUsed", "true".to_string()));
    }
    if load_origin_traffic {
        params.push(("loadOriginTraffic", "true".to_string()));
    }
    if load_origin_shield_bandwidth {
        params.push(("loadOriginShieldBandwidth", "true".to_string()));
    }
    if load_geographic_traffic_distribution {
        params.push(("loadGeographicTrafficDistribution", "true".to_string()));
    }
    if load_user_balance_history {
        params.push(("loadUserBalanceHistory", "true".to_string()));
    }

    let params_ref: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
    let resp = client.get_with_params("/statistics", &params_ref)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let stats: Statistics = resp.json()?;
            output::print_kv(&[
                (
                    "Total Bandwidth",
                    stats
                        .total_bandwidth_used
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Total Requests",
                    stats
                        .total_requests_served
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Cache Hit Rate",
                    stats
                        .cache_hit_rate
                        .map(|v| format!("{:.2}%", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Avg Origin Response Time",
                    stats
                        .average_origin_response_time
                        .map(|v| format!("{}ms", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Total Origin Traffic",
                    stats
                        .total_origin_traffic
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}
