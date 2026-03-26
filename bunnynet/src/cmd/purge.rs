use anyhow::Result;
use clap::Subcommand;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;

#[derive(Subcommand)]
pub enum PurgeAction {
    /// Purge a URL from the CDN cache
    Url {
        /// URL to purge
        url: String,
        /// Perform the purge asynchronously
        #[arg(long = "async")]
        is_async: bool,
        /// Only purge the exact path (no wildcard)
        #[arg(long)]
        exact_path: bool,
    },
}

pub fn run(action: PurgeAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        PurgeAction::Url {
            url,
            is_async,
            exact_path,
        } => purge_url(client, mode, &url, is_async, exact_path),
    }
}

fn purge_url(
    client: &Client,
    mode: OutputMode,
    url: &str,
    is_async: bool,
    exact_path: bool,
) -> Result<()> {
    let mut params: Vec<(&str, &str)> = vec![("url", url)];
    if is_async {
        params.push(("async", "true"));
    }
    if exact_path {
        params.push(("exactPath", "true"));
    }

    // Response may have empty body — do not call .json() on it
    let _resp = client.post_with_params("/purge", &params)?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({"status": "purged", "url": url});
            output::print_json(&json);
        }
        OutputMode::Table => {
            output::print_confirm(&format!("Purge queued for {}", url));
        }
    }

    Ok(())
}
