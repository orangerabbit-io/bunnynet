use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Region {
    pub id: i64,
    pub name: Option<String>,
    pub price_per_gigabyte: Option<f64>,
    pub region_code: Option<String>,
    pub continent_code: Option<String>,
    pub country_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub allow_latency_routing: Option<bool>,
}

#[derive(Debug, Tabled)]
pub struct RegionRow {
    #[tabled(rename = "ID")]
    pub id: i64,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "CODE")]
    pub code: String,
    #[tabled(rename = "CONTINENT")]
    pub continent: String,
    #[tabled(rename = "PRICE/GB")]
    pub price_per_gb: String,
    #[tabled(rename = "LATENCY ROUTING")]
    pub latency_routing: String,
}

impl From<&Region> for RegionRow {
    fn from(r: &Region) -> Self {
        RegionRow {
            id: r.id,
            name: r.name.clone().unwrap_or_else(|| "-".to_string()),
            code: r.region_code.clone().unwrap_or_else(|| "-".to_string()),
            continent: r.continent_code.clone().unwrap_or_else(|| "-".to_string()),
            price_per_gb: r
                .price_per_gigabyte
                .map(|p| format!("{:.4}", p))
                .unwrap_or_else(|| "-".to_string()),
            latency_routing: r
                .allow_latency_routing
                .map(|b| if b { "yes" } else { "no" }.to_string())
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_region() {
        let json = r#"{
            "Id": 1,
            "Name": "Europe (London)",
            "PricePerGigabyte": 0.01,
            "RegionCode": "UK",
            "ContinentCode": "EU",
            "CountryCode": "GB",
            "Latitude": 51.5,
            "Longitude": -0.12,
            "AllowLatencyRouting": true
        }"#;

        let region: Region = serde_json::from_str(json).unwrap();
        assert_eq!(region.id, 1);
        assert_eq!(region.name, Some("Europe (London)".to_string()));
        assert_eq!(region.region_code, Some("UK".to_string()));
        assert_eq!(region.allow_latency_routing, Some(true));
    }

    #[test]
    fn test_region_row_from_region() {
        let region = Region {
            id: 1,
            name: Some("Europe".to_string()),
            price_per_gigabyte: Some(0.01),
            region_code: Some("EU".to_string()),
            continent_code: Some("EU".to_string()),
            country_code: Some("GB".to_string()),
            latitude: Some(51.5),
            longitude: Some(-0.12),
            allow_latency_routing: Some(true),
        };

        let row = RegionRow::from(&region);
        assert_eq!(row.id, 1);
        assert_eq!(row.name, "Europe");
        assert_eq!(row.latency_routing, "yes");
    }
}
