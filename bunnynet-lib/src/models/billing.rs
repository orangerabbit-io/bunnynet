use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BillingModel {
    pub balance: Option<f64>,
    pub this_month_charges: Option<f64>,
    pub last_recharge_balance: Option<f64>,
    pub billing_enabled: Option<bool>,
    pub monthly_bandwidth_used: Option<i64>,
    pub monthly_charges_storage: Option<f64>,
    #[serde(rename = "MonthlyChargesDNS")]
    pub monthly_charges_dns: Option<f64>,
    #[serde(rename = "MonthlyChargesEUTraffic")]
    pub monthly_charges_eu_traffic: Option<f64>,
    #[serde(rename = "MonthlyChargesUSTraffic")]
    pub monthly_charges_us_traffic: Option<f64>,
    #[serde(rename = "MonthlyChargesASIATraffic")]
    pub monthly_charges_asia_traffic: Option<f64>,
    #[serde(rename = "MonthlyChargesAFTraffic")]
    pub monthly_charges_af_traffic: Option<f64>,
    #[serde(rename = "MonthlyChargesSATraffic")]
    pub monthly_charges_sa_traffic: Option<f64>,
    pub monthly_charges_optimizer: Option<f64>,
    pub monthly_charges_transcribe: Option<f64>,
    pub monthly_charges_premium_encoding: Option<f64>,
    pub monthly_charges_extra_pull_zones: Option<f64>,
    pub monthly_charges_extra_storage_zones: Option<f64>,
    pub monthly_charges_extra_dns_zones: Option<f64>,
    pub monthly_charges_extra_video_libraries: Option<f64>,
    pub monthly_charges_scripting: Option<f64>,
    pub monthly_charges_scripting_requests: Option<f64>,
    pub monthly_charges_scripting_cpu: Option<f64>,
    pub monthly_charges_drm: Option<f64>,
    pub monthly_charges_magic_containers: Option<f64>,
    pub monthly_charges_shield: Option<f64>,
    pub monthly_charges_taxes: Option<f64>,
    pub monthly_charges_web_sockets: Option<f64>,
    #[serde(rename = "MonthlyChargesDB")]
    pub monthly_charges_db: Option<f64>,
    pub monthly_dns_smart_queries_served: Option<i64>,
    pub monthly_dns_normal_queries_served: Option<i64>,
    pub monthly_transcription_minutes: Option<i64>,
    pub monthly_premium_encoding_billable_minutes: Option<i64>,
    #[serde(rename = "MonthlyDRMLicensesIssued")]
    pub monthly_drm_licenses_issued: Option<i64>,
    pub monthly_scripting_requests: Option<i64>,
    pub monthly_scripting_cpu_time: Option<i64>,
    pub minimum_monthly_commit: Option<f64>,
    #[serde(rename = "VATRate")]
    pub vat_rate: Option<f64>,
    #[serde(rename = "NextMonthVATRate")]
    pub next_month_vat_rate: Option<f64>,
    pub automatic_payment_image_url: Option<String>,
    pub automatic_payment_card_type: Option<String>,
    pub automatic_payment_identifier: Option<String>,
    pub automatic_payment_amount: Option<f64>,
    pub automatic_recharge_treshold: Option<f64>,
    pub automatic_recharge_enabled: Option<bool>,
    pub automatic_payment_failure_count: Option<i32>,
    #[serde(rename = "EUUSDiscount")]
    pub eu_us_discount: Option<i32>,
    pub south_america_discount: Option<i32>,
    pub africa_discount: Option<i32>,
    pub asia_oceania_discount: Option<i32>,
    pub optimizer_monthly_price: Option<f64>,
    pub drm_base_monthly_price: Option<f64>,
    pub drm_cost_per_license: Option<f64>,

    // Complex/chart fields
    pub billing_records: Option<serde_json::Value>,
    pub billing_history_chart: Option<serde_json::Value>,
    pub saved_payment_methods: Option<serde_json::Value>,
    #[serde(rename = "MonthlyDBWrites")]
    pub monthly_db_writes: Option<serde_json::Value>,
    #[serde(rename = "MonthlyDBReads")]
    pub monthly_db_reads: Option<serde_json::Value>,
    #[serde(rename = "MonthlyDBStorage")]
    pub monthly_db_storage: Option<serde_json::Value>,
    #[serde(rename = "MonthlyDBReplica")]
    pub monthly_db_replica: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BillingSummaryItem {
    pub pull_zone_id: Option<i64>,
    pub monthly_usage: Option<f64>,
    pub monthly_bandwidth_used: Option<i64>,
}

#[derive(Debug, Tabled)]
pub struct BillingSummaryItemRow {
    #[tabled(rename = "PULL ZONE ID")]
    pub pull_zone_id: String,
    #[tabled(rename = "MONTHLY USAGE")]
    pub monthly_usage: String,
    #[tabled(rename = "BANDWIDTH USED")]
    pub bandwidth_used: String,
}

impl From<&BillingSummaryItem> for BillingSummaryItemRow {
    fn from(item: &BillingSummaryItem) -> Self {
        BillingSummaryItemRow {
            pull_zone_id: item
                .pull_zone_id
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            monthly_usage: item
                .monthly_usage
                .map(|v| format!("{:.4}", v))
                .unwrap_or_else(|| "-".to_string()),
            bandwidth_used: item
                .monthly_bandwidth_used
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BillingAffiliateDetails {
    pub affiliate_balance: Option<f64>,
    pub affiliate_url: Option<String>,
    pub claim_bonus_percentage: Option<f64>,
    pub minimum_payout_amount: Option<f64>,
    pub affiliate_clicks_chart: Option<serde_json::Value>,
    pub affiliate_signups_chart: Option<serde_json::Value>,
    pub affiliate_conversions_chart: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentRequest {
    pub id: Option<i64>,
    pub amount: Option<f64>,
    pub date_generated: Option<String>,
    pub date_due: Option<String>,
    pub description: Option<String>,
    pub paid: Option<bool>,
    pub date_paid: Option<String>,
    pub billing_invoice_id: Option<i64>,
    pub billing_invoice_download_link: Option<String>,
    pub bank_transfer_reference: Option<String>,
    pub tax_rate: Option<f64>,
    pub taxed_amount: Option<f64>,
}

#[derive(Debug, Tabled)]
pub struct PaymentRequestRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "AMOUNT")]
    pub amount: String,
    #[tabled(rename = "DATE")]
    pub date: String,
    #[tabled(rename = "DUE")]
    pub due: String,
    #[tabled(rename = "DESCRIPTION")]
    pub description: String,
    #[tabled(rename = "PAID")]
    pub paid: String,
}

impl From<&PaymentRequest> for PaymentRequestRow {
    fn from(pr: &PaymentRequest) -> Self {
        PaymentRequestRow {
            id: pr
                .id
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            amount: pr
                .amount
                .map(|v| format!("{:.2}", v))
                .unwrap_or_else(|| "-".to_string()),
            date: pr.date_generated.clone().unwrap_or_else(|| "-".to_string()),
            due: pr.date_due.clone().unwrap_or_else(|| "-".to_string()),
            description: pr.description.clone().unwrap_or_else(|| "-".to_string()),
            paid: pr
                .paid
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_billing_model() {
        let json = r#"{
            "Balance": 42.50,
            "ThisMonthCharges": 12.75,
            "BillingEnabled": true,
            "MonthlyBandwidthUsed": 9876543,
            "MonthlyChargesStorage": 1.25,
            "MonthlyChargesDNS": 0.50,
            "VATRate": 0.20,
            "EUUSDiscount": 10,
            "BillingHistoryChart": {"2024-01": 100}
        }"#;

        let billing: BillingModel = serde_json::from_str(json).unwrap();
        assert_eq!(billing.balance, Some(42.50));
        assert_eq!(billing.this_month_charges, Some(12.75));
        assert_eq!(billing.billing_enabled, Some(true));
        assert_eq!(billing.monthly_bandwidth_used, Some(9876543));
        assert_eq!(billing.monthly_charges_storage, Some(1.25));
        assert_eq!(billing.monthly_charges_dns, Some(0.50));
        assert_eq!(billing.vat_rate, Some(0.20));
        assert_eq!(billing.eu_us_discount, Some(10));
        assert!(billing.billing_history_chart.is_some());
    }

    #[test]
    fn test_deserialize_billing_summary_item() {
        let json = r#"{
            "PullZoneId": 123,
            "MonthlyUsage": 5.1234,
            "MonthlyBandwidthUsed": 1000000
        }"#;

        let item: BillingSummaryItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.pull_zone_id, Some(123));
        assert_eq!(item.monthly_usage, Some(5.1234));
        assert_eq!(item.monthly_bandwidth_used, Some(1000000));
    }

    #[test]
    fn test_billing_summary_item_row() {
        let item = BillingSummaryItem {
            pull_zone_id: Some(123),
            monthly_usage: Some(5.1234),
            monthly_bandwidth_used: Some(1000000),
        };

        let row = BillingSummaryItemRow::from(&item);
        assert_eq!(row.pull_zone_id, "123");
        assert_eq!(row.monthly_usage, "5.1234");
        assert_eq!(row.bandwidth_used, "1000000");
    }

    #[test]
    fn test_deserialize_affiliate_details() {
        let json = r#"{
            "AffiliateBalance": 100.00,
            "AffiliateUrl": "https://bunny.net?ref=abc123",
            "ClaimBonusPercentage": 20.0,
            "MinimumPayoutAmount": 50.0,
            "AffiliateClicksChart": {"2024-01-01": 10}
        }"#;

        let details: BillingAffiliateDetails = serde_json::from_str(json).unwrap();
        assert_eq!(details.affiliate_balance, Some(100.0));
        assert_eq!(
            details.affiliate_url,
            Some("https://bunny.net?ref=abc123".to_string())
        );
        assert_eq!(details.claim_bonus_percentage, Some(20.0));
        assert_eq!(details.minimum_payout_amount, Some(50.0));
        assert!(details.affiliate_clicks_chart.is_some());
    }

    #[test]
    fn test_deserialize_payment_request() {
        let json = r#"{
            "Id": 456,
            "Amount": 99.99,
            "DateGenerated": "2024-01-15T00:00:00Z",
            "DateDue": "2024-02-15T00:00:00Z",
            "Description": "Monthly invoice",
            "Paid": false,
            "TaxRate": 0.20,
            "TaxedAmount": 119.99
        }"#;

        let pr: PaymentRequest = serde_json::from_str(json).unwrap();
        assert_eq!(pr.id, Some(456));
        assert_eq!(pr.amount, Some(99.99));
        assert_eq!(pr.date_generated, Some("2024-01-15T00:00:00Z".to_string()));
        assert_eq!(pr.date_due, Some("2024-02-15T00:00:00Z".to_string()));
        assert_eq!(pr.description, Some("Monthly invoice".to_string()));
        assert_eq!(pr.paid, Some(false));
    }

    #[test]
    fn test_payment_request_row() {
        let pr = PaymentRequest {
            id: Some(456),
            amount: Some(99.99),
            date_generated: Some("2024-01-15".to_string()),
            date_due: Some("2024-02-15".to_string()),
            description: Some("Monthly invoice".to_string()),
            paid: Some(false),
            date_paid: None,
            billing_invoice_id: None,
            billing_invoice_download_link: None,
            bank_transfer_reference: None,
            tax_rate: None,
            taxed_amount: None,
        };

        let row = PaymentRequestRow::from(&pr);
        assert_eq!(row.id, "456");
        assert_eq!(row.amount, "99.99");
        assert_eq!(row.date, "2024-01-15");
        assert_eq!(row.due, "2024-02-15");
        assert_eq!(row.description, "Monthly invoice");
        assert_eq!(row.paid, "false");
    }
}
