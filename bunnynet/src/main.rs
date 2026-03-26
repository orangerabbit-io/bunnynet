mod cmd;
#[allow(dead_code)]
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process;
use bunnynet_lib::client::Client;
use bunnynet_lib::config::Config;

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
pub enum Commands {}

#[allow(unreachable_code, unused_variables)]
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
    let _ = (&client, mode); // suppress unused warnings until commands are added

    match cli.command {}
}
