use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;
use std::str::FromStr;
use tabled::Tabled;

/// DNS record type enum. Values match the Bunny.net API integer encoding.
///
/// Note: the OpenAPI spec has NS=12 and Script=11, but the task spec
/// has a different order. We follow the OpenAPI spec which is authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum DnsRecordType {
    A = 0,
    AAAA = 1,
    CNAME = 2,
    TXT = 3,
    MX = 4,
    Redirect = 5,
    Flatten = 6,
    PullZone = 7,
    SRV = 8,
    CAA = 9,
    PTR = 10,
    Script = 11,
    NS = 12,
    SVCB = 13,
    HTTPS = 14,
    TLSA = 15,
}

impl fmt::Display for DnsRecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DnsRecordType::A => write!(f, "A"),
            DnsRecordType::AAAA => write!(f, "AAAA"),
            DnsRecordType::CNAME => write!(f, "CNAME"),
            DnsRecordType::TXT => write!(f, "TXT"),
            DnsRecordType::MX => write!(f, "MX"),
            DnsRecordType::Redirect => write!(f, "Redirect"),
            DnsRecordType::Flatten => write!(f, "Flatten"),
            DnsRecordType::PullZone => write!(f, "PullZone"),
            DnsRecordType::SRV => write!(f, "SRV"),
            DnsRecordType::CAA => write!(f, "CAA"),
            DnsRecordType::PTR => write!(f, "PTR"),
            DnsRecordType::Script => write!(f, "Script"),
            DnsRecordType::NS => write!(f, "NS"),
            DnsRecordType::SVCB => write!(f, "SVCB"),
            DnsRecordType::HTTPS => write!(f, "HTTPS"),
            DnsRecordType::TLSA => write!(f, "TLSA"),
        }
    }
}

impl FromStr for DnsRecordType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(DnsRecordType::A),
            "AAAA" => Ok(DnsRecordType::AAAA),
            "CNAME" => Ok(DnsRecordType::CNAME),
            "TXT" => Ok(DnsRecordType::TXT),
            "MX" => Ok(DnsRecordType::MX),
            "REDIRECT" => Ok(DnsRecordType::Redirect),
            "FLATTEN" => Ok(DnsRecordType::Flatten),
            "PULLZONE" => Ok(DnsRecordType::PullZone),
            "SRV" => Ok(DnsRecordType::SRV),
            "CAA" => Ok(DnsRecordType::CAA),
            "PTR" => Ok(DnsRecordType::PTR),
            "SCRIPT" => Ok(DnsRecordType::Script),
            "NS" => Ok(DnsRecordType::NS),
            "SVCB" => Ok(DnsRecordType::SVCB),
            "HTTPS" => Ok(DnsRecordType::HTTPS),
            "TLSA" => Ok(DnsRecordType::TLSA),
            _ => Err(format!(
                "Unknown DNS record type '{}'. Valid types: A, AAAA, CNAME, TXT, MX, \
                 Redirect, Flatten, PullZone, SRV, CAA, PTR, Script, NS, SVCB, HTTPS, TLSA",
                s
            )),
        }
    }
}

/// Helper to convert a DnsRecordType integer back to a display name
pub fn record_type_name(type_id: i32) -> String {
    match type_id {
        0 => "A".to_string(),
        1 => "AAAA".to_string(),
        2 => "CNAME".to_string(),
        3 => "TXT".to_string(),
        4 => "MX".to_string(),
        5 => "Redirect".to_string(),
        6 => "Flatten".to_string(),
        7 => "PullZone".to_string(),
        8 => "SRV".to_string(),
        9 => "CAA".to_string(),
        10 => "PTR".to_string(),
        11 => "Script".to_string(),
        12 => "NS".to_string(),
        13 => "SVCB".to_string(),
        14 => "HTTPS".to_string(),
        15 => "TLSA".to_string(),
        other => other.to_string(),
    }
}

/// DNS smart routing type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum DnsSmartRoutingType {
    None = 0,
    Latency = 1,
    Geolocation = 2,
}

/// DNS monitoring type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum DnsMonitoringType {
    None = 0,
    Ping = 1,
    Http = 2,
    Monitor = 3,
}

/// DNS monitoring status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum DnsMonitoringStatus {
    Unknown = 0,
    Online = 1,
    Offline = 2,
}

/// DNS record acceleration status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum AcceleratedStatus {
    None = 0,
    Pending = 1,
    Processing = 2,
    Completed = 3,
    Failed = 4,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DnsRecord {
    pub id: Option<i64>,
    pub r#type: Option<i32>,
    pub ttl: Option<i64>,
    pub value: Option<String>,
    pub name: Option<String>,
    pub weight: Option<i32>,
    pub priority: Option<i32>,
    pub port: Option<i32>,
    pub flags: Option<i32>,
    pub tag: Option<String>,
    pub accelerated: Option<bool>,
    pub accelerated_pull_zone_id: Option<i64>,
    pub link_name: Option<String>,
    pub monitor_status: Option<i32>,
    pub monitor_type: Option<i32>,
    pub geolocation_latitude: Option<f64>,
    pub geolocation_longitude: Option<f64>,
    pub latency_zone: Option<String>,
    pub smart_routing_type: Option<i32>,
    pub disabled: Option<bool>,
    pub enviromental_variables: Option<serde_json::Value>,
    pub comment: Option<String>,
    pub auto_ssl_issuance: Option<bool>,
    pub acceleration_status: Option<i32>,
}

#[derive(Debug, Tabled)]
pub struct DnsRecordRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "TYPE")]
    pub record_type: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "VALUE")]
    pub value: String,
    #[tabled(rename = "TTL")]
    pub ttl: String,
    #[tabled(rename = "PRIORITY")]
    pub priority: String,
    #[tabled(rename = "DISABLED")]
    pub disabled: String,
}

impl From<&DnsRecord> for DnsRecordRow {
    fn from(r: &DnsRecord) -> Self {
        DnsRecordRow {
            id: r
                .id
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            record_type: r
                .r#type
                .map(record_type_name)
                .unwrap_or_else(|| "-".to_string()),
            name: r.name.clone().unwrap_or_else(|| "-".to_string()),
            value: r.value.clone().unwrap_or_else(|| "-".to_string()),
            ttl: r
                .ttl
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            priority: r
                .priority
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            disabled: r
                .disabled
                .map(|v| if v { "Yes" } else { "No" }.to_string())
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_record_type_display() {
        assert_eq!(DnsRecordType::A.to_string(), "A");
        assert_eq!(DnsRecordType::AAAA.to_string(), "AAAA");
        assert_eq!(DnsRecordType::CNAME.to_string(), "CNAME");
        assert_eq!(DnsRecordType::TXT.to_string(), "TXT");
        assert_eq!(DnsRecordType::MX.to_string(), "MX");
        assert_eq!(DnsRecordType::Redirect.to_string(), "Redirect");
        assert_eq!(DnsRecordType::Flatten.to_string(), "Flatten");
        assert_eq!(DnsRecordType::PullZone.to_string(), "PullZone");
        assert_eq!(DnsRecordType::SRV.to_string(), "SRV");
        assert_eq!(DnsRecordType::CAA.to_string(), "CAA");
        assert_eq!(DnsRecordType::PTR.to_string(), "PTR");
        assert_eq!(DnsRecordType::Script.to_string(), "Script");
        assert_eq!(DnsRecordType::NS.to_string(), "NS");
        assert_eq!(DnsRecordType::SVCB.to_string(), "SVCB");
        assert_eq!(DnsRecordType::HTTPS.to_string(), "HTTPS");
        assert_eq!(DnsRecordType::TLSA.to_string(), "TLSA");
    }

    #[test]
    fn test_dns_record_type_from_str() {
        assert_eq!(DnsRecordType::from_str("A").unwrap(), DnsRecordType::A);
        assert_eq!(DnsRecordType::from_str("a").unwrap(), DnsRecordType::A);
        assert_eq!(
            DnsRecordType::from_str("AAAA").unwrap(),
            DnsRecordType::AAAA
        );
        assert_eq!(
            DnsRecordType::from_str("cname").unwrap(),
            DnsRecordType::CNAME
        );
        assert_eq!(DnsRecordType::from_str("TXT").unwrap(), DnsRecordType::TXT);
        assert_eq!(DnsRecordType::from_str("MX").unwrap(), DnsRecordType::MX);
        assert_eq!(
            DnsRecordType::from_str("redirect").unwrap(),
            DnsRecordType::Redirect
        );
        assert_eq!(
            DnsRecordType::from_str("PULLZONE").unwrap(),
            DnsRecordType::PullZone
        );
        assert_eq!(DnsRecordType::from_str("SRV").unwrap(), DnsRecordType::SRV);
        assert_eq!(DnsRecordType::from_str("CAA").unwrap(), DnsRecordType::CAA);
        assert_eq!(DnsRecordType::from_str("NS").unwrap(), DnsRecordType::NS);
        assert_eq!(
            DnsRecordType::from_str("SVCB").unwrap(),
            DnsRecordType::SVCB
        );
        assert_eq!(
            DnsRecordType::from_str("HTTPS").unwrap(),
            DnsRecordType::HTTPS
        );
        assert_eq!(
            DnsRecordType::from_str("TLSA").unwrap(),
            DnsRecordType::TLSA
        );
        assert!(DnsRecordType::from_str("INVALID").is_err());
    }

    #[test]
    fn test_dns_record_type_serde() {
        let a: DnsRecordType = serde_json::from_str("0").unwrap();
        assert_eq!(a, DnsRecordType::A);
        assert_eq!(serde_json::to_string(&a).unwrap(), "0");

        let cname: DnsRecordType = serde_json::from_str("2").unwrap();
        assert_eq!(cname, DnsRecordType::CNAME);
        assert_eq!(serde_json::to_string(&cname).unwrap(), "2");

        let tlsa: DnsRecordType = serde_json::from_str("15").unwrap();
        assert_eq!(tlsa, DnsRecordType::TLSA);
        assert_eq!(serde_json::to_string(&tlsa).unwrap(), "15");
    }

    #[test]
    fn test_deserialize_dns_record() {
        let json = r#"{
            "Id": 999,
            "Type": 0,
            "Ttl": 300,
            "Value": "1.2.3.4",
            "Name": "www",
            "Weight": 100,
            "Priority": 10,
            "Port": 443,
            "Flags": 0,
            "Tag": "issue",
            "Accelerated": false,
            "AcceleratedPullZoneId": 0,
            "LinkName": null,
            "MonitorStatus": 0,
            "MonitorType": 0,
            "GeolocationLatitude": 51.5074,
            "GeolocationLongitude": -0.1278,
            "LatencyZone": "EU",
            "SmartRoutingType": 0,
            "Disabled": false,
            "EnviromentalVariables": null,
            "Comment": "test record",
            "AutoSslIssuance": true,
            "AccelerationStatus": 0
        }"#;

        let record: DnsRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, Some(999));
        assert_eq!(record.r#type, Some(0));
        assert_eq!(record.ttl, Some(300));
        assert_eq!(record.value, Some("1.2.3.4".to_string()));
        assert_eq!(record.name, Some("www".to_string()));
        assert_eq!(record.weight, Some(100));
        assert_eq!(record.priority, Some(10));
        assert_eq!(record.port, Some(443));
        assert_eq!(record.disabled, Some(false));
        assert_eq!(record.comment, Some("test record".to_string()));
        assert_eq!(record.auto_ssl_issuance, Some(true));
    }

    #[test]
    fn test_dns_record_row() {
        let record = DnsRecord {
            id: Some(42),
            r#type: Some(0),
            ttl: Some(300),
            value: Some("1.2.3.4".to_string()),
            name: Some("www".to_string()),
            weight: None,
            priority: Some(10),
            port: None,
            flags: None,
            tag: None,
            accelerated: None,
            accelerated_pull_zone_id: None,
            link_name: None,
            monitor_status: None,
            monitor_type: None,
            geolocation_latitude: None,
            geolocation_longitude: None,
            latency_zone: None,
            smart_routing_type: None,
            disabled: Some(false),
            enviromental_variables: None,
            comment: None,
            auto_ssl_issuance: None,
            acceleration_status: None,
        };

        let row = DnsRecordRow::from(&record);
        assert_eq!(row.id, "42");
        assert_eq!(row.record_type, "A");
        assert_eq!(row.name, "www");
        assert_eq!(row.value, "1.2.3.4");
        assert_eq!(row.ttl, "300");
        assert_eq!(row.priority, "10");
        assert_eq!(row.disabled, "No");
    }

    #[test]
    fn test_dns_record_row_defaults() {
        let record = DnsRecord {
            id: None,
            r#type: None,
            ttl: None,
            value: None,
            name: None,
            weight: None,
            priority: None,
            port: None,
            flags: None,
            tag: None,
            accelerated: None,
            accelerated_pull_zone_id: None,
            link_name: None,
            monitor_status: None,
            monitor_type: None,
            geolocation_latitude: None,
            geolocation_longitude: None,
            latency_zone: None,
            smart_routing_type: None,
            disabled: None,
            enviromental_variables: None,
            comment: None,
            auto_ssl_issuance: None,
            acceleration_status: None,
        };

        let row = DnsRecordRow::from(&record);
        assert_eq!(row.id, "-");
        assert_eq!(row.record_type, "-");
        assert_eq!(row.name, "-");
        assert_eq!(row.value, "-");
        assert_eq!(row.ttl, "-");
        assert_eq!(row.priority, "-");
        assert_eq!(row.disabled, "-");
    }

    #[test]
    fn test_smart_routing_type_serde() {
        let none: DnsSmartRoutingType = serde_json::from_str("0").unwrap();
        let latency: DnsSmartRoutingType = serde_json::from_str("1").unwrap();
        let geo: DnsSmartRoutingType = serde_json::from_str("2").unwrap();

        assert_eq!(none, DnsSmartRoutingType::None);
        assert_eq!(latency, DnsSmartRoutingType::Latency);
        assert_eq!(geo, DnsSmartRoutingType::Geolocation);
    }

    #[test]
    fn test_monitoring_type_serde() {
        let none: DnsMonitoringType = serde_json::from_str("0").unwrap();
        let ping: DnsMonitoringType = serde_json::from_str("1").unwrap();
        let http: DnsMonitoringType = serde_json::from_str("2").unwrap();
        let monitor: DnsMonitoringType = serde_json::from_str("3").unwrap();

        assert_eq!(none, DnsMonitoringType::None);
        assert_eq!(ping, DnsMonitoringType::Ping);
        assert_eq!(http, DnsMonitoringType::Http);
        assert_eq!(monitor, DnsMonitoringType::Monitor);
    }

    #[test]
    fn test_monitoring_status_serde() {
        let unknown: DnsMonitoringStatus = serde_json::from_str("0").unwrap();
        let online: DnsMonitoringStatus = serde_json::from_str("1").unwrap();
        let offline: DnsMonitoringStatus = serde_json::from_str("2").unwrap();

        assert_eq!(unknown, DnsMonitoringStatus::Unknown);
        assert_eq!(online, DnsMonitoringStatus::Online);
        assert_eq!(offline, DnsMonitoringStatus::Offline);
    }

    #[test]
    fn test_accelerated_status_serde() {
        let none: AcceleratedStatus = serde_json::from_str("0").unwrap();
        let pending: AcceleratedStatus = serde_json::from_str("1").unwrap();
        let processing: AcceleratedStatus = serde_json::from_str("2").unwrap();
        let completed: AcceleratedStatus = serde_json::from_str("3").unwrap();
        let failed: AcceleratedStatus = serde_json::from_str("4").unwrap();

        assert_eq!(none, AcceleratedStatus::None);
        assert_eq!(pending, AcceleratedStatus::Pending);
        assert_eq!(processing, AcceleratedStatus::Processing);
        assert_eq!(completed, AcceleratedStatus::Completed);
        assert_eq!(failed, AcceleratedStatus::Failed);
    }
}
