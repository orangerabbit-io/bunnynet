use anyhow::Result;
use clap::Subcommand;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::api_key::{ApiKey, ApiKeyRow};

#[derive(Subcommand)]
pub enum ApiKeyAction {
    /// List API keys
    List,
}

pub fn run(action: ApiKeyAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        ApiKeyAction::List => list(client, mode),
    }
}

fn list(client: &Client, mode: OutputMode) -> Result<()> {
    let items: Vec<ApiKey> = client.fetch_all_pages("/apikey", &[])?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::to_value(&items)?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let rows: Vec<ApiKeyRow> = items.iter().map(ApiKeyRow::from).collect();
            output::print_table(&rows);
        }
    }

    Ok(())
}
