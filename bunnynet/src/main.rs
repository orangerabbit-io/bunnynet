mod cmd;
mod output;

use anyhow::Result;
use bunnynet_lib::client::Client;
use bunnynet_lib::config::Config;
use clap::{Parser, Subcommand};
use std::process;

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
pub enum Commands {
    /// Manage regions
    Region {
        #[command(subcommand)]
        action: cmd::region::RegionAction,
    },
    /// Manage countries
    Country {
        #[command(subcommand)]
        action: cmd::country::CountryAction,
    },
    /// Manage API keys
    #[command(name = "api-key")]
    ApiKey {
        #[command(subcommand)]
        action: cmd::api_key::ApiKeyAction,
    },
    /// Purge CDN cache
    Purge {
        #[command(subcommand)]
        action: cmd::purge::PurgeAction,
    },
    /// Search across all resources
    Search {
        /// Search query
        query: String,
        /// Number of results to skip
        #[arg(long)]
        from: Option<i64>,
        /// Maximum number of results
        #[arg(long)]
        size: Option<i64>,
    },
    /// Manage billing
    Billing {
        #[command(subcommand)]
        action: cmd::billing::BillingAction,
    },
    /// Manage storage zones
    #[command(name = "storage-zone")]
    StorageZone {
        #[command(subcommand)]
        action: cmd::storage_zone::StorageZoneAction,
    },
    /// Manage DNS zones
    #[command(name = "dns-zone")]
    DnsZone {
        #[command(subcommand)]
        action: cmd::dns_zone::DnsZoneAction,
    },
    /// Manage pull zones
    #[command(name = "pull-zone")]
    PullZone {
        #[command(subcommand)]
        action: cmd::pull_zone::PullZoneAction,
    },
    /// Manage video libraries
    #[command(name = "video-library")]
    VideoLibrary {
        #[command(subcommand)]
        action: cmd::video_library::VideoLibraryAction,
    },
    /// View CDN statistics
    Statistics {
        /// Start date for statistics range
        #[arg(long)]
        date_from: Option<String>,
        /// End date for statistics range
        #[arg(long)]
        date_to: Option<String>,
        /// Filter by pull zone ID
        #[arg(long)]
        pull_zone: Option<i64>,
        /// Filter by server zone ID
        #[arg(long)]
        server_zone_id: Option<i64>,
        /// Return hourly grouping
        #[arg(long)]
        hourly: bool,
        /// Load error response charts
        #[arg(long)]
        load_errors: bool,
        /// Load origin response time data
        #[arg(long)]
        load_origin_response_times: bool,
        /// Load requests served data
        #[arg(long)]
        load_requests_served: bool,
        /// Load bandwidth used data
        #[arg(long)]
        load_bandwidth_used: bool,
        /// Load origin traffic data
        #[arg(long)]
        load_origin_traffic: bool,
        /// Load origin shield bandwidth data
        #[arg(long)]
        load_origin_shield_bandwidth: bool,
        /// Load geographic traffic distribution data
        #[arg(long)]
        load_geographic_traffic_distribution: bool,
        /// Load user balance history data
        #[arg(long)]
        load_user_balance_history: bool,
    },
}

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
    match cli.command {
        Commands::Region { action } => cmd::region::run(action, &client, mode),
        Commands::Country { action } => cmd::country::run(action, &client, mode),
        Commands::ApiKey { action } => cmd::api_key::run(action, &client, mode),
        Commands::Purge { action } => cmd::purge::run(action, &client, mode),
        Commands::Search { query, from, size } => {
            cmd::search::run(&client, mode, &query, from, size)
        }
        Commands::Billing { action } => cmd::billing::run(action, &client, mode),
        Commands::StorageZone { action } => cmd::storage_zone::run(action, &client, mode),
        Commands::DnsZone { action } => cmd::dns_zone::run(action, &client, mode),
        Commands::PullZone { action } => cmd::pull_zone::run(action, &client, mode),
        Commands::VideoLibrary { action } => cmd::video_library::run(action, &client, mode),
        Commands::Statistics {
            date_from,
            date_to,
            pull_zone,
            server_zone_id,
            hourly,
            load_errors,
            load_origin_response_times,
            load_requests_served,
            load_bandwidth_used,
            load_origin_traffic,
            load_origin_shield_bandwidth,
            load_geographic_traffic_distribution,
            load_user_balance_history,
        } => cmd::statistics::run(
            &client,
            mode,
            date_from,
            date_to,
            pull_zone,
            server_zone_id,
            hourly,
            load_errors,
            load_origin_response_times,
            load_requests_served,
            load_bandwidth_used,
            load_origin_traffic,
            load_origin_shield_bandwidth,
            load_geographic_traffic_distribution,
            load_user_balance_history,
        ),
    }
}
