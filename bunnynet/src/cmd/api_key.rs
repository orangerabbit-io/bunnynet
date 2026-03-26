use anyhow::Result;
use clap::Subcommand;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::api_key::{ApiKey, ApiKeyRow};
use bunnynet_lib::models::pagination::PaginatedList;

#[derive(Subcommand)]
pub enum ApiKeyAction {
    /// List API keys
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
}

pub fn run(action: ApiKeyAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        ApiKeyAction::List { page, per_page } => list(client, mode, page, per_page),
    }
}

fn list(client: &Client, mode: OutputMode, page: Option<i32>, per_page: Option<i32>) -> Result<()> {
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(p) = page {
        params.push(("page", p.to_string()));
    }
    if let Some(pp) = per_page {
        params.push(("perPage", pp.to_string()));
    }
    let params_ref: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

    let resp = client.get_with_params("/apikey", &params_ref)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let list: PaginatedList<ApiKey> = resp.json()?;
            let rows: Vec<ApiKeyRow> = list.items.iter().map(ApiKeyRow::from).collect();
            output::print_table(&rows);
            output::print_pagination(list.current_page, list.total_items, list.has_more_items);
        }
    }

    Ok(())
}
