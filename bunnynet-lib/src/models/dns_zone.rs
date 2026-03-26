use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tabled::Tabled;

use super::dns_record::DnsRecord;

/// Log anonymization type: OneDigit (0) or Drop (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum LogAnonymizationType {
    OneDigit = 0,
    Drop = 1,
}

/// Private key type for automatic certificates: Ecdsa (0) or Rsa (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum CertificateKeyType {
    Ecdsa = 0,
    Rsa = 1,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DnsZone {
    pub id: i64,
    pub domain: Option<String>,
    pub records: Option<Vec<DnsRecord>>,
    pub date_modified: Option<String>,
    pub date_created: Option<String>,
    pub nameservers_detected: Option<bool>,
    pub custom_nameservers_enabled: Option<bool>,
    pub nameserver1: Option<String>,
    pub nameserver2: Option<String>,
    pub soa_email: Option<String>,
    pub nameservers_next_check: Option<String>,
    pub logging_enabled: Option<bool>,
    #[serde(rename = "LoggingIPAnonymizationEnabled")]
    pub logging_ip_anonymization_enabled: Option<bool>,
    pub log_anonymization_type: Option<i32>,
    pub dns_sec_enabled: Option<bool>,
    pub certificate_key_type: Option<i32>,
}

#[derive(Debug, Tabled)]
pub struct DnsZoneRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "DOMAIN")]
    pub domain: String,
    #[tabled(rename = "NAMESERVERS")]
    pub nameservers: String,
    #[tabled(rename = "DNSSEC")]
    pub dnssec: String,
    #[tabled(rename = "LOGGING")]
    pub logging: String,
    #[tabled(rename = "CREATED")]
    pub created: String,
}

impl From<&DnsZone> for DnsZoneRow {
    fn from(z: &DnsZone) -> Self {
        let ns = match (&z.nameserver1, &z.nameserver2) {
            (Some(ns1), Some(ns2)) => format!("{}, {}", ns1, ns2),
            (Some(ns1), None) => ns1.clone(),
            (None, Some(ns2)) => ns2.clone(),
            (None, None) => "-".to_string(),
        };

        DnsZoneRow {
            id: z.id.to_string(),
            domain: z.domain.clone().unwrap_or_else(|| "-".to_string()),
            nameservers: ns,
            dnssec: z
                .dns_sec_enabled
                .map(|v| if v { "Enabled" } else { "Disabled" }.to_string())
                .unwrap_or_else(|| "-".to_string()),
            logging: z
                .logging_enabled
                .map(|v| if v { "Enabled" } else { "Disabled" }.to_string())
                .unwrap_or_else(|| "-".to_string()),
            created: z.date_created.clone().unwrap_or_else(|| "-".to_string()),
        }
    }
}

/// Statistics model for a DNS zone
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DnsZoneStatistics {
    pub total_queries_served: Option<i64>,
    pub queries_served_chart: Option<std::collections::HashMap<String, f64>>,
    pub normal_queries_served_chart: Option<std::collections::HashMap<String, f64>>,
    pub smart_queries_served_chart: Option<std::collections::HashMap<String, f64>>,
    pub queries_by_type_chart: Option<std::collections::HashMap<String, i64>>,
}

/// Result of a DNS zone import operation
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DnsZoneImportResult {
    pub records_successful: Option<i32>,
    pub records_failed: Option<i32>,
    pub records_skipped: Option<i32>,
}

/// DNS zone scan job status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum DnsZoneScanJobStatus {
    Pending = 0,
    InProgress = 1,
    Completed = 2,
    Failed = 3,
}

/// Response from triggering a DNS zone record scan
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DnsZoneRecordScanTriggerResponse {
    pub job_id: Option<String>,
    pub status: Option<i32>,
}

/// Discovered record type enum (differs from DnsRecordType — has gaps)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum DnsZoneDiscoveredRecordType {
    A = 0,
    AAAA = 1,
    CNAME = 2,
    TXT = 3,
    MX = 4,
    SRV = 8,
    CAA = 9,
    PTR = 10,
    NS = 12,
    Svcb = 13,
    HTTPS = 14,
    TLSA = 15,
    SOA = 16,
}

/// A discovered DNS record from a scan
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DnsZoneDiscoveredRecord {
    pub name: Option<String>,
    pub r#type: Option<i32>,
    pub ttl: Option<i32>,
    pub value: Option<String>,
    pub priority: Option<i32>,
    pub weight: Option<i32>,
    pub port: Option<i32>,
    pub is_proxied: Option<bool>,
}

/// Response from getting DNS zone scan results
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DnsZoneRecordScanJobResponse {
    pub job_id: Option<String>,
    pub zone_id: Option<i64>,
    pub domain: Option<String>,
    pub account_id: Option<String>,
    pub status: Option<i32>,
    pub created_at: Option<String>,
    pub completed_at: Option<String>,
    pub records: Option<Vec<DnsZoneDiscoveredRecord>>,
    pub error: Option<String>,
}

/// DNSSEC DS record info
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DnsSecDsRecord {
    pub enabled: Option<bool>,
    pub ds_record: Option<String>,
    pub digest: Option<String>,
    pub digest_type: Option<String>,
    pub algorithm: Option<i32>,
    pub public_key: Option<String>,
    pub key_tag: Option<i32>,
    pub flags: Option<i32>,
    pub ds_configured: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_dns_zone() {
        let json = r#"{
            "Id": 12345,
            "Domain": "example.com",
            "Records": [],
            "DateModified": "2024-06-15T10:30:00Z",
            "DateCreated": "2024-01-01T00:00:00Z",
            "NameserversDetected": true,
            "CustomNameserversEnabled": false,
            "Nameserver1": "ns1.bunny.net",
            "Nameserver2": "ns2.bunny.net",
            "SoaEmail": "admin@example.com",
            "NameserversNextCheck": "2024-06-16T10:30:00Z",
            "LoggingEnabled": true,
            "LoggingIPAnonymizationEnabled": false,
            "LogAnonymizationType": 0,
            "DnsSecEnabled": true,
            "CertificateKeyType": 0
        }"#;

        let zone: DnsZone = serde_json::from_str(json).unwrap();
        assert_eq!(zone.id, 12345);
        assert_eq!(zone.domain, Some("example.com".to_string()));
        assert_eq!(zone.records.as_ref().unwrap().len(), 0);
        assert_eq!(zone.nameservers_detected, Some(true));
        assert_eq!(zone.custom_nameservers_enabled, Some(false));
        assert_eq!(zone.nameserver1, Some("ns1.bunny.net".to_string()));
        assert_eq!(zone.nameserver2, Some("ns2.bunny.net".to_string()));
        assert_eq!(zone.soa_email, Some("admin@example.com".to_string()));
        assert_eq!(zone.logging_enabled, Some(true));
        assert_eq!(zone.logging_ip_anonymization_enabled, Some(false));
        assert_eq!(zone.log_anonymization_type, Some(0));
        assert_eq!(zone.dns_sec_enabled, Some(true));
        assert_eq!(zone.certificate_key_type, Some(0));
    }

    #[test]
    fn test_dns_zone_row() {
        let zone = DnsZone {
            id: 100,
            domain: Some("example.com".to_string()),
            records: None,
            date_modified: None,
            date_created: Some("2024-01-01T00:00:00Z".to_string()),
            nameservers_detected: None,
            custom_nameservers_enabled: None,
            nameserver1: Some("ns1.bunny.net".to_string()),
            nameserver2: Some("ns2.bunny.net".to_string()),
            soa_email: None,
            nameservers_next_check: None,
            logging_enabled: Some(true),
            logging_ip_anonymization_enabled: None,
            log_anonymization_type: None,
            dns_sec_enabled: Some(false),
            certificate_key_type: None,
        };

        let row = DnsZoneRow::from(&zone);
        assert_eq!(row.id, "100");
        assert_eq!(row.domain, "example.com");
        assert_eq!(row.nameservers, "ns1.bunny.net, ns2.bunny.net");
        assert_eq!(row.dnssec, "Disabled");
        assert_eq!(row.logging, "Enabled");
        assert_eq!(row.created, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_dns_zone_row_defaults() {
        let zone = DnsZone {
            id: 1,
            domain: None,
            records: None,
            date_modified: None,
            date_created: None,
            nameservers_detected: None,
            custom_nameservers_enabled: None,
            nameserver1: None,
            nameserver2: None,
            soa_email: None,
            nameservers_next_check: None,
            logging_enabled: None,
            logging_ip_anonymization_enabled: None,
            log_anonymization_type: None,
            dns_sec_enabled: None,
            certificate_key_type: None,
        };

        let row = DnsZoneRow::from(&zone);
        assert_eq!(row.id, "1");
        assert_eq!(row.domain, "-");
        assert_eq!(row.nameservers, "-");
        assert_eq!(row.dnssec, "-");
        assert_eq!(row.logging, "-");
        assert_eq!(row.created, "-");
    }

    #[test]
    fn test_log_anonymization_type_serde() {
        let one_digit: LogAnonymizationType = serde_json::from_str("0").unwrap();
        let drop: LogAnonymizationType = serde_json::from_str("1").unwrap();

        assert_eq!(one_digit, LogAnonymizationType::OneDigit);
        assert_eq!(drop, LogAnonymizationType::Drop);

        assert_eq!(serde_json::to_string(&one_digit).unwrap(), "0");
        assert_eq!(serde_json::to_string(&drop).unwrap(), "1");
    }

    #[test]
    fn test_certificate_key_type_serde() {
        let ecdsa: CertificateKeyType = serde_json::from_str("0").unwrap();
        let rsa: CertificateKeyType = serde_json::from_str("1").unwrap();

        assert_eq!(ecdsa, CertificateKeyType::Ecdsa);
        assert_eq!(rsa, CertificateKeyType::Rsa);

        assert_eq!(serde_json::to_string(&ecdsa).unwrap(), "0");
        assert_eq!(serde_json::to_string(&rsa).unwrap(), "1");
    }

    #[test]
    fn test_dns_zone_statistics_deserialize() {
        let json = r#"{
            "TotalQueriesServed": 12345,
            "QueriesServedChart": {"2024-06-01": 100.0, "2024-06-02": 200.0},
            "NormalQueriesServedChart": {"2024-06-01": 80.0},
            "SmartQueriesServedChart": {"2024-06-01": 20.0},
            "QueriesByTypeChart": {"A": 500, "AAAA": 200}
        }"#;

        let stats: DnsZoneStatistics = serde_json::from_str(json).unwrap();
        assert_eq!(stats.total_queries_served, Some(12345));
        let chart = stats.queries_served_chart.unwrap();
        assert_eq!(chart.get("2024-06-01"), Some(&100.0));
        assert_eq!(chart.get("2024-06-02"), Some(&200.0));
        let by_type = stats.queries_by_type_chart.unwrap();
        assert_eq!(by_type.get("A"), Some(&500));
    }

    #[test]
    fn test_dns_zone_import_result_deserialize() {
        let json = r#"{
            "RecordsSuccessful": 10,
            "RecordsFailed": 2,
            "RecordsSkipped": 1
        }"#;

        let result: DnsZoneImportResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.records_successful, Some(10));
        assert_eq!(result.records_failed, Some(2));
        assert_eq!(result.records_skipped, Some(1));
    }

    #[test]
    fn test_dns_zone_scan_job_status_serde() {
        let pending: DnsZoneScanJobStatus = serde_json::from_str("0").unwrap();
        let in_progress: DnsZoneScanJobStatus = serde_json::from_str("1").unwrap();
        let completed: DnsZoneScanJobStatus = serde_json::from_str("2").unwrap();
        let failed: DnsZoneScanJobStatus = serde_json::from_str("3").unwrap();

        assert_eq!(pending, DnsZoneScanJobStatus::Pending);
        assert_eq!(in_progress, DnsZoneScanJobStatus::InProgress);
        assert_eq!(completed, DnsZoneScanJobStatus::Completed);
        assert_eq!(failed, DnsZoneScanJobStatus::Failed);
    }

    #[test]
    fn test_dns_zone_scan_trigger_response_deserialize() {
        let json = r#"{
            "JobId": "abc-123",
            "Status": 0
        }"#;

        let resp: DnsZoneRecordScanTriggerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.job_id, Some("abc-123".to_string()));
        assert_eq!(resp.status, Some(0));
    }

    #[test]
    fn test_dns_zone_scan_job_response_deserialize() {
        let json = r#"{
            "JobId": "abc-123",
            "ZoneId": 456,
            "Domain": "example.com",
            "Status": 2,
            "CreatedAt": "2024-06-01T00:00:00Z",
            "CompletedAt": "2024-06-01T00:01:00Z",
            "Records": [
                {
                    "Name": "@",
                    "Type": 0,
                    "Ttl": 300,
                    "Value": "1.2.3.4",
                    "IsProxied": false
                }
            ]
        }"#;

        let resp: DnsZoneRecordScanJobResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.job_id, Some("abc-123".to_string()));
        assert_eq!(resp.zone_id, Some(456));
        assert_eq!(resp.domain, Some("example.com".to_string()));
        assert_eq!(resp.status, Some(2));
        let records = resp.records.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, Some("@".to_string()));
        assert_eq!(records[0].value, Some("1.2.3.4".to_string()));
    }
}
