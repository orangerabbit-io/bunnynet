use anyhow::Result;
use clap::Subcommand;

use crate::output::{self, OutputMode};
use bunnynet_lib::client::Client;
use bunnynet_lib::models::billing::{
    BillingAffiliateDetails, BillingModel, BillingSummaryItem, BillingSummaryItemRow,
    PaymentRequest, PaymentRequestRow,
};

#[derive(Subcommand)]
pub enum BillingAction {
    /// Get billing details
    Get,
    /// Get billing summary
    Summary,
    /// Get affiliate details
    Affiliate,
    /// Get pending payment requests
    PaymentRequests,
    /// Download payment request invoice as PDF
    DownloadInvoice {
        /// Payment request ID
        id: i64,
        /// Output file path
        #[arg(long)]
        output: String,
    },
    /// Download billing summary document as PDF
    DownloadSummary {
        /// Billing record ID
        id: i64,
        /// Output file path
        #[arg(long)]
        output: String,
    },
}

pub fn run(action: BillingAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        BillingAction::Get => get(client, mode),
        BillingAction::Summary => summary(client, mode),
        BillingAction::Affiliate => affiliate(client, mode),
        BillingAction::PaymentRequests => payment_requests(client, mode),
        BillingAction::DownloadInvoice { id, output } => download_invoice(client, id, &output),
        BillingAction::DownloadSummary { id, output } => download_summary(client, id, &output),
    }
}

fn get(client: &Client, mode: OutputMode) -> Result<()> {
    let resp = client.get("/billing")?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let billing: BillingModel = resp.json()?;
            output::print_kv(&[
                (
                    "Balance",
                    billing
                        .balance
                        .map(|v| format!("${:.2}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "This Month Charges",
                    billing
                        .this_month_charges
                        .map(|v| format!("${:.2}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Bandwidth Used",
                    billing
                        .monthly_bandwidth_used
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}

fn summary(client: &Client, mode: OutputMode) -> Result<()> {
    let resp = client.get("/billing/summary")?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let items: Vec<BillingSummaryItem> = resp.json()?;
            let rows: Vec<BillingSummaryItemRow> =
                items.iter().map(BillingSummaryItemRow::from).collect();
            output::print_table(&rows);
        }
    }

    Ok(())
}

fn affiliate(client: &Client, mode: OutputMode) -> Result<()> {
    let resp = client.get("/billing/affiliate")?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let details: BillingAffiliateDetails = resp.json()?;
            output::print_kv(&[
                (
                    "Affiliate Balance",
                    details
                        .affiliate_balance
                        .map(|v| format!("${:.2}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Affiliate URL",
                    details
                        .affiliate_url
                        .clone()
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Claim Bonus %",
                    details
                        .claim_bonus_percentage
                        .map(|v| format!("{:.1}%", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                (
                    "Min Payout",
                    details
                        .minimum_payout_amount
                        .map(|v| format!("${:.2}", v))
                        .unwrap_or_else(|| "-".to_string()),
                ),
            ]);
        }
    }

    Ok(())
}

fn payment_requests(client: &Client, mode: OutputMode) -> Result<()> {
    let resp = client.get("/billing/payment-requests")?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let items: Vec<PaymentRequest> = resp.json()?;
            let rows: Vec<PaymentRequestRow> = items.iter().map(PaymentRequestRow::from).collect();
            output::print_table(&rows);
        }
    }

    Ok(())
}

fn download_invoice(client: &Client, id: i64, output_path: &str) -> Result<()> {
    let path = format!("/billing/payment-request-invoice/{}/pdf", id);
    let bytes = client.get_bytes(&path)?;
    std::fs::write(output_path, &bytes)?;
    output::print_confirm(&format!("Invoice saved to {}", output_path));
    Ok(())
}

fn download_summary(client: &Client, id: i64, output_path: &str) -> Result<()> {
    let path = format!("/billing/summary/{}/pdf", id);
    let bytes = client.get_bytes(&path)?;
    std::fs::write(output_path, &bytes)?;
    output::print_confirm(&format!("Summary saved to {}", output_path));
    Ok(())
}
