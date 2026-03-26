use anyhow::Result;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::search::{SearchResultItemRow, SearchResults};

pub fn run(
    client: &Client,
    mode: OutputMode,
    query: &str,
    from: Option<i64>,
    size: Option<i64>,
) -> Result<()> {
    let mut params: Vec<(&str, String)> = vec![("search", query.to_string())];
    if let Some(f) = from {
        params.push(("from", f.to_string()));
    }
    if let Some(s) = size {
        params.push(("size", s.to_string()));
    }
    let params_ref: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

    let resp = client.get_with_params("/search", &params_ref)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let results: SearchResults = resp.json()?;
            let items = results.search_results.unwrap_or_default();
            let rows: Vec<SearchResultItemRow> =
                items.iter().map(SearchResultItemRow::from).collect();
            output::print_table(&rows);
        }
    }

    Ok(())
}
