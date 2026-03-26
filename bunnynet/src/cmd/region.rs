use anyhow::Result;
use clap::Subcommand;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::region::{Region, RegionRow};

#[derive(Subcommand)]
pub enum RegionAction {
    /// List all regions
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
