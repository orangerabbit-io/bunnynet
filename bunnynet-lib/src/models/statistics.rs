use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Statistics {
    pub total_bandwidth_used: Option<i64>,
    pub total_origin_traffic: Option<i64>,
    pub average_origin_response_time: Option<i32>,
    pub total_requests_served: Option<i64>,
    pub cache_hit_rate: Option<f64>,

    // Chart fields — complex maps, only useful in JSON mode
    pub origin_response_time_chart: Option<serde_json::Value>,
    pub bandwidth_used_chart: Option<serde_json::Value>,
    pub bandwidth_cached_chart: Option<serde_json::Value>,
    pub cache_hit_rate_chart: Option<serde_json::Value>,
    pub requests_served_chart: Option<serde_json::Value>,
    pub pull_requests_pulled_chart: Option<serde_json::Value>,
    pub origin_shield_bandwidth_used_chart: Option<serde_json::Value>,
    pub origin_shield_internal_bandwidth_used_chart: Option<serde_json::Value>,
    pub origin_traffic_chart: Option<serde_json::Value>,
    pub user_balance_history_chart: Option<serde_json::Value>,
    pub geo_traffic_distribution: Option<serde_json::Value>,
    #[serde(rename = "Error3xxChart")]
    pub error_3xx_chart: Option<serde_json::Value>,
    #[serde(rename = "Error4xxChart")]
    pub error_4xx_chart: Option<serde_json::Value>,
    #[serde(rename = "Error5xxChart")]
    pub error_5xx_chart: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_statistics() {
        let json = r#"{
            "TotalBandwidthUsed": 1234567890,
            "TotalOriginTraffic": 987654321,
            "AverageOriginResponseTime": 250,
            "TotalRequestsServed": 5000000,
            "CacheHitRate": 95.5,
            "BandwidthUsedChart": {
                "2024-01-01": 100.0,
                "2024-01-02": 200.0
            },
            "GeoTrafficDistribution": {
                "US": 50000,
                "DE": 30000
            }
        }"#;

        let stats: Statistics = serde_json::from_str(json).unwrap();
        assert_eq!(stats.total_bandwidth_used, Some(1234567890));
        assert_eq!(stats.total_origin_traffic, Some(987654321));
        assert_eq!(stats.average_origin_response_time, Some(250));
        assert_eq!(stats.total_requests_served, Some(5000000));
        assert_eq!(stats.cache_hit_rate, Some(95.5));
        assert!(stats.bandwidth_used_chart.is_some());
        assert!(stats.geo_traffic_distribution.is_some());
    }

    #[test]
    fn test_deserialize_statistics_minimal() {
        let json = r#"{}"#;

        let stats: Statistics = serde_json::from_str(json).unwrap();
        assert_eq!(stats.total_bandwidth_used, None);
        assert_eq!(stats.cache_hit_rate, None);
        assert!(stats.bandwidth_used_chart.is_none());
    }

    #[test]
    fn test_deserialize_statistics_error_charts() {
        let json = r#"{
            "Error3xxChart": {"2024-01-01": 10.0},
            "Error4xxChart": {"2024-01-01": 20.0},
            "Error5xxChart": {"2024-01-01": 5.0}
        }"#;

        let stats: Statistics = serde_json::from_str(json).unwrap();
        assert!(stats.error_3xx_chart.is_some());
        assert!(stats.error_4xx_chart.is_some());
        assert!(stats.error_5xx_chart.is_some());
    }
}
