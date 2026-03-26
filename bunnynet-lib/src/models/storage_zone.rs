use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tabled::Tabled;

/// Storage zone tier: Standard (0) or Edge (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum StorageZoneTier {
    Standard = 0,
    Edge = 1,
}

/// Storage zone S3 support type: NotSupported (0) or Supported (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum StorageZoneS3Type {
    NotSupported = 0,
    Supported = 1,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorageZone {
    pub id: i64,
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub date_modified: Option<String>,
    pub deleted: Option<bool>,
    pub storage_used: Option<i64>,
    pub files_stored: Option<i64>,
    pub region: Option<String>,
    pub replication_regions: Option<Vec<String>>,
    pub pull_zones: Option<serde_json::Value>,
    pub read_only_password: Option<String>,
    pub rewrite404_to200: Option<bool>,
    pub custom404_file_path: Option<String>,
    pub storage_hostname: Option<String>,
    pub zone_tier: Option<i32>,
    pub replication_change_in_progress: Option<bool>,
    pub price_override: Option<f64>,
    pub discount: Option<i32>,
    pub storage_zone_type: Option<i32>,
}

#[derive(Debug, Tabled)]
pub struct StorageZoneRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "REGION")]
    pub region: String,
    #[tabled(rename = "STORAGE USED")]
    pub storage_used: String,
    #[tabled(rename = "FILES")]
    pub files: String,
    #[tabled(rename = "HOSTNAME")]
    pub hostname: String,
    #[tabled(rename = "TIER")]
    pub tier: String,
}

impl From<&StorageZone> for StorageZoneRow {
    fn from(sz: &StorageZone) -> Self {
        StorageZoneRow {
            id: sz.id.to_string(),
            name: sz.name.clone().unwrap_or_else(|| "-".to_string()),
            region: sz.region.clone().unwrap_or_else(|| "-".to_string()),
            storage_used: sz
                .storage_used
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            files: sz
                .files_stored
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            hostname: sz
                .storage_hostname
                .clone()
                .unwrap_or_else(|| "-".to_string()),
            tier: sz
                .zone_tier
                .map(|v| match v {
                    0 => "Standard".to_string(),
                    1 => "Edge".to_string(),
                    other => other.to_string(),
                })
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

/// Statistics model for a storage zone
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorageZoneStatistics {
    pub storage_used_chart: Option<std::collections::HashMap<String, i64>>,
    pub file_count_chart: Option<std::collections::HashMap<String, i64>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_storage_zone() {
        let json = r#"{
            "Id": 12345,
            "UserId": "user-1",
            "Name": "my-zone",
            "Password": "secret-pw",
            "DateModified": "2024-06-15T10:30:00Z",
            "Deleted": false,
            "StorageUsed": 1048576,
            "FilesStored": 42,
            "Region": "DE",
            "ReplicationRegions": ["NY", "LA"],
            "PullZones": [],
            "ReadOnlyPassword": "ro-pw",
            "Rewrite404To200": true,
            "Custom404FilePath": "/404.html",
            "StorageHostname": "storage.bunnycdn.com",
            "ZoneTier": 0,
            "ReplicationChangeInProgress": false,
            "PriceOverride": 0.01,
            "Discount": 5,
            "StorageZoneType": 1
        }"#;

        let sz: StorageZone = serde_json::from_str(json).unwrap();
        assert_eq!(sz.id, 12345);
        assert_eq!(sz.user_id, Some("user-1".to_string()));
        assert_eq!(sz.name, Some("my-zone".to_string()));
        assert_eq!(sz.password, Some("secret-pw".to_string()));
        assert_eq!(sz.deleted, Some(false));
        assert_eq!(sz.storage_used, Some(1048576));
        assert_eq!(sz.files_stored, Some(42));
        assert_eq!(sz.region, Some("DE".to_string()));
        assert_eq!(
            sz.replication_regions,
            Some(vec!["NY".to_string(), "LA".to_string()])
        );
        assert_eq!(sz.rewrite404_to200, Some(true));
        assert_eq!(sz.custom404_file_path, Some("/404.html".to_string()));
        assert_eq!(
            sz.storage_hostname,
            Some("storage.bunnycdn.com".to_string())
        );
        assert_eq!(sz.zone_tier, Some(0));
        assert_eq!(sz.replication_change_in_progress, Some(false));
        assert_eq!(sz.price_override, Some(0.01));
        assert_eq!(sz.discount, Some(5));
        assert_eq!(sz.storage_zone_type, Some(1));
    }

    #[test]
    fn test_storage_zone_row() {
        let sz = StorageZone {
            id: 100,
            user_id: None,
            name: Some("test-zone".to_string()),
            password: None,
            date_modified: None,
            deleted: None,
            storage_used: Some(2048),
            files_stored: Some(10),
            region: Some("NY".to_string()),
            replication_regions: None,
            pull_zones: None,
            read_only_password: None,
            rewrite404_to200: None,
            custom404_file_path: None,
            storage_hostname: Some("ny.storage.bunnycdn.com".to_string()),
            zone_tier: Some(1),
            replication_change_in_progress: None,
            price_override: None,
            discount: None,
            storage_zone_type: None,
        };

        let row = StorageZoneRow::from(&sz);
        assert_eq!(row.id, "100");
        assert_eq!(row.name, "test-zone");
        assert_eq!(row.region, "NY");
        assert_eq!(row.storage_used, "2048");
        assert_eq!(row.files, "10");
        assert_eq!(row.hostname, "ny.storage.bunnycdn.com");
        assert_eq!(row.tier, "Edge");
    }

    #[test]
    fn test_storage_zone_row_defaults() {
        let sz = StorageZone {
            id: 1,
            user_id: None,
            name: None,
            password: None,
            date_modified: None,
            deleted: None,
            storage_used: None,
            files_stored: None,
            region: None,
            replication_regions: None,
            pull_zones: None,
            read_only_password: None,
            rewrite404_to200: None,
            custom404_file_path: None,
            storage_hostname: None,
            zone_tier: None,
            replication_change_in_progress: None,
            price_override: None,
            discount: None,
            storage_zone_type: None,
        };

        let row = StorageZoneRow::from(&sz);
        assert_eq!(row.id, "1");
        assert_eq!(row.name, "-");
        assert_eq!(row.region, "-");
        assert_eq!(row.storage_used, "-");
        assert_eq!(row.files, "-");
        assert_eq!(row.hostname, "-");
        assert_eq!(row.tier, "-");
    }

    #[test]
    fn test_storage_zone_tier_serde() {
        let json_standard = "0";
        let json_edge = "1";

        let standard: StorageZoneTier = serde_json::from_str(json_standard).unwrap();
        let edge: StorageZoneTier = serde_json::from_str(json_edge).unwrap();

        assert_eq!(standard, StorageZoneTier::Standard);
        assert_eq!(edge, StorageZoneTier::Edge);

        assert_eq!(serde_json::to_string(&standard).unwrap(), "0");
        assert_eq!(serde_json::to_string(&edge).unwrap(), "1");
    }

    #[test]
    fn test_storage_zone_s3_type_serde() {
        let json_not_supported = "0";
        let json_supported = "1";

        let not_supported: StorageZoneS3Type =
            serde_json::from_str(json_not_supported).unwrap();
        let supported: StorageZoneS3Type = serde_json::from_str(json_supported).unwrap();

        assert_eq!(not_supported, StorageZoneS3Type::NotSupported);
        assert_eq!(supported, StorageZoneS3Type::Supported);

        assert_eq!(serde_json::to_string(&not_supported).unwrap(), "0");
        assert_eq!(serde_json::to_string(&supported).unwrap(), "1");
    }

    #[test]
    fn test_deserialize_storage_zone_statistics() {
        let json = r#"{
            "StorageUsedChart": {"2024-06-01": 1024, "2024-06-02": 2048},
            "FileCountChart": {"2024-06-01": 10, "2024-06-02": 20}
        }"#;

        let stats: StorageZoneStatistics = serde_json::from_str(json).unwrap();
        let used = stats.storage_used_chart.unwrap();
        assert_eq!(used.get("2024-06-01"), Some(&1024));
        assert_eq!(used.get("2024-06-02"), Some(&2048));

        let files = stats.file_count_chart.unwrap();
        assert_eq!(files.get("2024-06-01"), Some(&10));
        assert_eq!(files.get("2024-06-02"), Some(&20));
    }
}
