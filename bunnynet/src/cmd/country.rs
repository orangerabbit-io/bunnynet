use anyhow::Result;
use clap::Subcommand;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::country::{Country, CountryRow};

#[derive(Subcommand)]
pub enum CountryAction {
    /// List all countries
    List,
}

pub fn run(action: CountryAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        CountryAction::List => list(client, mode),
    }
}

fn list(client: &Client, mode: OutputMode) -> Result<()> {
    let resp = client.get("/country")?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let countries: Vec<Country> = resp.json()?;
            let rows: Vec<CountryRow> = countries.iter().map(CountryRow::from).collect();
            output::print_table(&rows);
        }
    }

    Ok(())
}
