use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use tabled::Tabled;

// --- Enums (serde_repr) ---

/// Pull zone type: Premium (0) or Volume (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum PullZoneType {
    Premium = 0,
    Volume = 1,
}

/// Pull zone origin type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum PullZoneOriginType {
    OriginUrl = 0,
    DnsAccelerate = 1,
    StorageZone = 2,
    LoadBalancer = 3,
    EdgeScript = 4,
    MagicContainers = 5,
    PushZone = 6,
}

/// Pull zone log format: Plain (0) or JSON (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum PullZoneLogFormat {
    Plain = 0,
    Json = 1,
}

/// Pull zone log forwarder protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum PullZoneLogForwarderProtocolType {
    Udp = 0,
    Tcp = 1,
    TcpEncrypted = 2,
    DataDog = 3,
}

/// Optimizer watermark position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum OptimizerWatermarkPosition {
    BottomLeft = 0,
    BottomRight = 1,
    TopLeft = 2,
    TopRight = 3,
    Center = 4,
    CenterStretch = 5,
}

/// Perma-cache type: Automatic (0) or Manual (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum PermaCacheType {
    Automatic = 0,
    Manual = 1,
}

/// Preloading screen theme: Light (0) or Dark (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum PreloadingScreenTheme {
    Light = 0,
    Dark = 1,
}

/// Sticky session type: Off (0) or On (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum StickySessionType {
    Off = 0,
    On = 1,
}

/// Shield DDoS protection type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum ShieldDDosProtectionType {
    DetectOnly = 0,
    ActiveStandard = 1,
    ActiveAggressive = 2,
}

/// Certificate provision type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum CertificateProvisionType {
    Unknown = 0,
    Http01 = 1,
    Dns01 = 2,
    Custom = 3,
    Managed = 4,
}

/// Private key type: Ecdsa (0) or Rsa (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum PrivateKeyType {
    Ecdsa = 0,
    Rsa = 1,
}

/// Edge rule action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum EdgeRuleActionType {
    ForceSSL = 0,
    Redirect = 1,
    OriginUrl = 2,
    OverrideCacheTime = 3,
    BlockRequest = 4,
    SetResponseHeader = 5,
    SetRequestHeader = 6,
    ForceDownload = 7,
    DisableTokenAuthentication = 8,
    EnableTokenAuthentication = 9,
    OverrideCacheTimePublic = 10,
    IgnoreQueryString = 11,
    DisableOptimizer = 12,
    ForceCompression = 13,
    SetStatusCode = 14,
    BypassPermaCache = 15,
    OverrideBrowserCacheTime = 16,
    OriginStorage = 17,
    SetNetworkRateLimit = 18,
    SetConnectionLimit = 19,
    SetRequestsPerSecondLimit = 20,
    RunEdgeScript = 21,
    OriginMagicContainers = 22,
    DisableWAF = 23,
    RetryOrigin = 24,
    OverrideBrowserCacheResponseHeader = 25,
    RemoveBrowserCacheResponseHeader = 26,
    DisableShieldChallenge = 27,
    DisableShield = 28,
    DisableShieldBotDetection = 29,
    BypassAwsS3Authentication = 30,
    DisableShieldAccessLists = 31,
    DisableShieldRateLimiting = 32,
    EnableRequestCoalescing = 33,
    DisableRequestCoalescing = 34,
    StripCookiesClientToOrigin = 37,
}

/// Trigger types for edge rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum TriggerTypes {
    Url = 0,
    RequestHeader = 1,
    ResponseHeader = 2,
    UrlExtension = 3,
    CountryCode = 4,
    RemoteIP = 5,
    UrlQueryString = 6,
    RandomChance = 7,
    StatusCode = 8,
    RequestMethod = 9,
    CookieValue = 10,
    CountryStateCode = 11,
    OriginRetryAttemptCount = 12,
    OriginConnectionError = 13,
}

/// Trigger matching types: MatchAny (0), MatchAll (1), MatchNone (2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum TriggerMatchingTypes {
    MatchAny = 0,
    MatchAll = 1,
    MatchNone = 2,
}

/// Pattern matching types: MatchAny (0), MatchAll (1), MatchNone (2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum PatternMatchingTypes {
    MatchAny = 0,
    MatchAll = 1,
    MatchNone = 2,
}

/// Execution phase: Cache (0) or LoadBalancer (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum ExecutionPhase {
    Cache = 0,
    LoadBalancer = 1,
}

/// Log anonymization type: OneDigit (0) or Drop (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum LogAnonymizationType {
    OneDigit = 0,
    Drop = 1,
}

// --- Nested models ---

/// Hostname model for pull zones
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostnameModel {
    pub id: Option<i64>,
    pub value: Option<String>,
    #[serde(rename = "ForceSSL")]
    pub force_ssl: Option<bool>,
    pub is_system_hostname: Option<bool>,
    pub is_managed_hostname: Option<bool>,
    pub has_certificate: Option<bool>,
    pub certificate: Option<String>,
    pub certificate_key: Option<String>,
    pub certificate_key_type: Option<PrivateKeyType>,
    pub certificate_provision_type: Option<CertificateProvisionType>,
}

/// Edge rule trigger
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Trigger {
    pub r#type: Option<TriggerTypes>,
    pub pattern_matches: Option<Vec<String>>,
    pub pattern_matching_type: Option<PatternMatchingTypes>,
    pub parameter1: Option<String>,
}

/// Edge rule extra action model
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EdgeRuleV2ActionModel {
    pub action_type: Option<EdgeRuleActionType>,
    pub action_parameter1: Option<String>,
    pub action_parameter2: Option<String>,
    pub action_parameter3: Option<String>,
}

/// Edge rule V2 model
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EdgeRuleV2Model {
    pub guid: Option<String>,
    pub action_type: Option<EdgeRuleActionType>,
    pub action_parameter1: Option<String>,
    pub action_parameter2: Option<String>,
    pub action_parameter3: Option<String>,
    pub triggers: Option<Vec<Trigger>>,
    pub extra_actions: Option<Vec<EdgeRuleV2ActionModel>>,
    pub trigger_matching_type: Option<TriggerMatchingTypes>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub order_index: Option<i32>,
    pub read_only: Option<bool>,
}

/// Optimizer class model
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OptimizerClassModel {
    pub name: Option<String>,
    pub properties: Option<HashMap<String, String>>,
}

/// Bunny AI image blueprint model
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BunnyAiImageBlueprintModel {
    pub name: Option<String>,
    pub properties: Option<HashMap<String, String>>,
}

// --- Main PullZone model (164 fields) ---

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PullZone {
    pub id: i64,
    pub name: Option<String>,
    pub origin_url: Option<String>,
    pub enabled: Option<bool>,
    pub suspended: Option<bool>,
    pub hostnames: Option<Vec<HostnameModel>>,
    pub storage_zone_id: Option<i64>,
    pub edge_script_id: Option<i64>,
    pub edge_script_execution_phase: Option<ExecutionPhase>,
    pub middleware_script_id: Option<i64>,
    pub magic_containers_app_id: Option<String>,
    pub magic_containers_endpoint_id: Option<String>,
    pub allowed_referrers: Option<Vec<String>>,
    pub blocked_referrers: Option<Vec<String>>,
    #[serde(rename = "BlockedIps")]
    pub blocked_ips: Option<Vec<String>>,
    #[serde(rename = "EnableGeoZoneUS")]
    pub enable_geo_zone_us: Option<bool>,
    #[serde(rename = "EnableGeoZoneEU")]
    pub enable_geo_zone_eu: Option<bool>,
    #[serde(rename = "EnableGeoZoneASIA")]
    pub enable_geo_zone_asia: Option<bool>,
    #[serde(rename = "EnableGeoZoneSA")]
    pub enable_geo_zone_sa: Option<bool>,
    #[serde(rename = "EnableGeoZoneAF")]
    pub enable_geo_zone_af: Option<bool>,
    pub zone_security_enabled: Option<bool>,
    pub zone_security_key: Option<String>,
    #[serde(rename = "ZoneSecurityIncludeHashRemoteIP")]
    pub zone_security_include_hash_remote_ip: Option<bool>,
    pub ignore_query_strings: Option<bool>,
    pub monthly_bandwidth_limit: Option<i64>,
    pub monthly_bandwidth_used: Option<i64>,
    pub monthly_charges: Option<f64>,
    pub add_host_header: Option<bool>,
    pub origin_host_header: Option<String>,
    #[serde(rename = "Type")]
    pub pull_zone_type: Option<PullZoneType>,
    pub access_control_origin_header_extensions: Option<Vec<String>>,
    pub enable_access_control_origin_header: Option<bool>,
    pub disable_cookies: Option<bool>,
    pub budget_redirected_countries: Option<Vec<String>>,
    pub blocked_countries: Option<Vec<String>>,
    pub enable_origin_shield: Option<bool>,
    pub cache_control_max_age_override: Option<i64>,
    pub cache_control_public_max_age_override: Option<i64>,
    pub burst_size: Option<i32>,
    pub request_limit: Option<i32>,
    pub block_root_path_access: Option<bool>,
    pub block_post_requests: Option<bool>,
    pub limit_rate_per_second: Option<f64>,
    pub limit_rate_after: Option<f64>,
    #[serde(rename = "ConnectionLimitPerIPCount")]
    pub connection_limit_per_ip_count: Option<i32>,
    pub price_override: Option<f64>,
    pub optimizer_pricing: Option<f64>,
    pub add_canonical_header: Option<bool>,
    pub enable_logging: Option<bool>,
    pub enable_cache_slice: Option<bool>,
    pub enable_smart_cache: Option<bool>,
    pub edge_rules: Option<Vec<EdgeRuleV2Model>>,
    #[serde(rename = "EnableWebPVary")]
    pub enable_web_p_vary: Option<bool>,
    pub enable_avif_vary: Option<bool>,
    pub enable_country_code_vary: Option<bool>,
    pub enable_country_state_code_vary: Option<bool>,
    pub enable_mobile_vary: Option<bool>,
    pub enable_cookie_vary: Option<bool>,
    pub cookie_vary_parameters: Option<Vec<String>>,
    pub enable_hostname_vary: Option<bool>,
    pub cname_domain: Option<String>,
    #[serde(rename = "AWSSigningEnabled")]
    pub aws_signing_enabled: Option<bool>,
    #[serde(rename = "AWSSigningKey")]
    pub aws_signing_key: Option<String>,
    #[serde(rename = "AWSSigningSecret")]
    pub aws_signing_secret: Option<String>,
    #[serde(rename = "AWSSigningRegionName")]
    pub aws_signing_region_name: Option<String>,
    #[serde(rename = "LoggingIPAnonymizationEnabled")]
    pub logging_ip_anonymization_enabled: Option<bool>,
    #[serde(rename = "EnableTLS1")]
    pub enable_tls1: Option<bool>,
    #[serde(rename = "EnableTLS1_1")]
    pub enable_tls1_1: Option<bool>,
    #[serde(rename = "VerifyOriginSSL")]
    pub verify_origin_ssl: Option<bool>,
    pub error_page_enable_custom_code: Option<bool>,
    pub error_page_custom_code: Option<String>,
    pub error_page_enable_statuspage_widget: Option<bool>,
    pub error_page_statuspage_code: Option<String>,
    pub error_page_whitelabel: Option<bool>,
    pub origin_shield_zone_code: Option<String>,
    pub log_forwarding_enabled: Option<bool>,
    pub log_forwarding_hostname: Option<String>,
    pub log_forwarding_port: Option<i32>,
    pub log_forwarding_token: Option<String>,
    pub log_forwarding_protocol: Option<PullZoneLogForwarderProtocolType>,
    pub logging_save_to_storage: Option<bool>,
    pub logging_storage_zone_id: Option<i64>,
    pub follow_redirects: Option<bool>,
    pub video_library_id: Option<i64>,
    #[serde(rename = "DnsRecordId")]
    pub dns_record_id: Option<i64>,
    #[serde(rename = "DnsZoneId")]
    pub dns_zone_id: Option<i64>,
    #[serde(rename = "DnsRecordValue")]
    pub dns_record_value: Option<String>,
    pub optimizer_enabled: Option<bool>,
    pub optimizer_tunnel_enabled: Option<bool>,
    pub optimizer_desktop_max_width: Option<i32>,
    pub optimizer_mobile_max_width: Option<i32>,
    pub optimizer_image_quality: Option<i32>,
    pub optimizer_mobile_image_quality: Option<i32>,
    #[serde(rename = "OptimizerEnableWebP")]
    pub optimizer_enable_web_p: Option<bool>,
    pub optimizer_prerender_html: Option<bool>,
    pub optimizer_enable_manipulation_engine: Option<bool>,
    #[serde(rename = "OptimizerMinifyCSS")]
    pub optimizer_minify_css: Option<bool>,
    pub optimizer_minify_java_script: Option<bool>,
    pub optimizer_watermark_enabled: Option<bool>,
    pub optimizer_watermark_url: Option<String>,
    pub optimizer_watermark_position: Option<OptimizerWatermarkPosition>,
    pub optimizer_watermark_offset: Option<f64>,
    pub optimizer_watermark_min_image_size: Option<i32>,
    pub optimizer_automatic_optimization_enabled: Option<bool>,
    pub perma_cache_storage_zone_id: Option<i64>,
    pub perma_cache_type: Option<PermaCacheType>,
    pub origin_retries: Option<i32>,
    pub origin_connect_timeout: Option<i32>,
    pub origin_response_timeout: Option<i32>,
    pub use_stale_while_updating: Option<bool>,
    pub use_stale_while_offline: Option<bool>,
    #[serde(rename = "OriginRetry5XXResponses")]
    pub origin_retry_5xx_responses: Option<bool>,
    pub origin_retry_connection_timeout: Option<bool>,
    pub origin_retry_response_timeout: Option<bool>,
    pub origin_retry_delay: Option<i32>,
    pub query_string_vary_parameters: Option<Vec<String>>,
    pub origin_shield_enable_concurrency_limit: Option<bool>,
    pub origin_shield_max_concurrent_requests: Option<i32>,
    pub enable_safe_hop: Option<bool>,
    pub cache_error_responses: Option<bool>,
    pub origin_shield_queue_max_wait_time: Option<i32>,
    pub origin_shield_max_queued_requests: Option<i32>,
    pub optimizer_classes: Option<Vec<OptimizerClassModel>>,
    pub optimizer_force_classes: Option<bool>,
    pub optimizer_static_html_enabled: Option<bool>,
    #[serde(rename = "OptimizerStaticHtmlWordPressPath")]
    pub optimizer_static_html_word_press_path: Option<String>,
    #[serde(rename = "OptimizerStaticHtmlWordPressBypassCookie")]
    pub optimizer_static_html_word_press_bypass_cookie: Option<String>,
    pub use_background_update: Option<bool>,
    #[serde(rename = "EnableAutoSSL")]
    pub enable_auto_ssl: Option<bool>,
    pub enable_query_string_ordering: Option<bool>,
    pub log_anonymization_type: Option<LogAnonymizationType>,
    pub log_format: Option<PullZoneLogFormat>,
    pub log_forwarding_format: Option<PullZoneLogFormat>,
    #[serde(rename = "ShieldDDosProtectionType")]
    pub shield_ddos_protection_type: Option<ShieldDDosProtectionType>,
    #[serde(rename = "ShieldDDosProtectionEnabled")]
    pub shield_ddos_protection_enabled: Option<bool>,
    pub origin_type: Option<PullZoneOriginType>,
    pub enable_request_coalescing: Option<bool>,
    pub request_coalescing_timeout: Option<i32>,
    pub origin_link_value: Option<String>,
    pub disable_lets_encrypt: Option<bool>,
    #[serde(rename = "EnableBunnyImageAi")]
    pub enable_bunny_image_ai: Option<bool>,
    #[serde(rename = "BunnyAiImageBlueprints")]
    pub bunny_ai_image_blueprints: Option<Vec<BunnyAiImageBlueprintModel>>,
    pub preloading_screen_enabled: Option<bool>,
    pub preloading_screen_show_on_first_visit: Option<bool>,
    pub preloading_screen_code: Option<String>,
    pub preloading_screen_logo_url: Option<String>,
    pub preloading_screen_code_enabled: Option<bool>,
    pub preloading_screen_theme: Option<PreloadingScreenTheme>,
    pub preloading_screen_delay: Option<i32>,
    #[serde(rename = "EUUSDiscount")]
    pub euus_discount: Option<i32>,
    pub south_america_discount: Option<i32>,
    pub africa_discount: Option<i32>,
    pub asia_oceania_discount: Option<i32>,
    pub routing_filters: Option<Vec<String>>,
    pub block_none_referrer: Option<bool>,
    pub sticky_session_type: Option<StickySessionType>,
    pub sticky_session_cookie_name: Option<String>,
    pub sticky_session_client_headers: Option<String>,
    pub user_id: Option<String>,
    pub cache_version: Option<i64>,
    pub optimizer_enable_upscaling: Option<bool>,
    pub enable_web_sockets: Option<bool>,
    pub max_web_socket_connections: Option<i32>,
    pub enable_extended_logging: Option<bool>,
}

// --- Statistics models ---

/// Optimizer statistics model
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OptimizerStatisticsModel {
    pub requests_optimized_chart: Option<HashMap<String, i64>>,
    pub average_compression_chart: Option<HashMap<String, i64>>,
    pub traffic_saved_chart: Option<HashMap<String, i64>>,
    pub average_processing_time_chart: Option<HashMap<String, i64>>,
    pub total_requests_optimized: Option<f64>,
    pub total_traffic_saved: Option<f64>,
    pub average_processing_time: Option<f64>,
    pub average_compression_ratio: Option<f64>,
}

/// Origin shield concurrency statistics model
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OriginShieldConcurrencyStatisticsModel {
    pub concurrent_requests_chart: Option<HashMap<String, i64>>,
    pub queued_requests_chart: Option<HashMap<String, i64>>,
}

/// SafeHop statistics model
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SafeHopStatisticsModel {
    pub requests_retried_chart: Option<HashMap<String, i64>>,
    pub requests_saved_chart: Option<HashMap<String, i64>>,
    pub total_requests_retried: Option<f64>,
    pub total_requests_saved: Option<f64>,
}

// --- Display row for table output ---

#[derive(Debug, Tabled)]
pub struct PullZoneRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "ORIGIN URL")]
    pub origin_url: String,
    #[tabled(rename = "ORIGIN TYPE")]
    pub origin_type: String,
    #[tabled(rename = "ENABLED")]
    pub enabled: String,
    #[tabled(rename = "MONTHLY BANDWIDTH")]
    pub monthly_bandwidth: String,
    #[tabled(rename = "HOSTNAMES")]
    pub hostnames: String,
}

impl From<&PullZone> for PullZoneRow {
    fn from(pz: &PullZone) -> Self {
        PullZoneRow {
            id: pz.id.to_string(),
            name: pz.name.clone().unwrap_or_else(|| "-".to_string()),
            origin_url: pz.origin_url.clone().unwrap_or_else(|| "-".to_string()),
            origin_type: pz
                .origin_type
                .map(|t| match t {
                    PullZoneOriginType::OriginUrl => "OriginUrl".to_string(),
                    PullZoneOriginType::DnsAccelerate => "DnsAccelerate".to_string(),
                    PullZoneOriginType::StorageZone => "StorageZone".to_string(),
                    PullZoneOriginType::LoadBalancer => "LoadBalancer".to_string(),
                    PullZoneOriginType::EdgeScript => "EdgeScript".to_string(),
                    PullZoneOriginType::MagicContainers => "MagicContainers".to_string(),
                    PullZoneOriginType::PushZone => "PushZone".to_string(),
                })
                .unwrap_or_else(|| "-".to_string()),
            enabled: pz
                .enabled
                .map(|e| if e { "Yes" } else { "No" }.to_string())
                .unwrap_or_else(|| "-".to_string()),
            monthly_bandwidth: pz
                .monthly_bandwidth_used
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            hostnames: pz
                .hostnames
                .as_ref()
                .map(|h| h.len().to_string())
                .unwrap_or_else(|| "0".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_pull_zone() {
        let json = r#"{
            "Id": 12345,
            "Name": "my-cdn",
            "OriginUrl": "https://example.com",
            "Enabled": true,
            "Suspended": false,
            "Hostnames": [
                {
                    "Id": 1,
                    "Value": "cdn.example.com",
                    "ForceSSL": true,
                    "IsSystemHostname": false,
                    "IsManagedHostname": false,
                    "HasCertificate": true
                }
            ],
            "StorageZoneId": 0,
            "EdgeScriptId": 0,
            "EdgeScriptExecutionPhase": 0,
            "AllowedReferrers": ["example.com"],
            "BlockedReferrers": [],
            "BlockedIps": ["1.2.3.4"],
            "EnableGeoZoneUS": true,
            "EnableGeoZoneEU": true,
            "EnableGeoZoneASIA": false,
            "EnableGeoZoneSA": false,
            "EnableGeoZoneAF": false,
            "ZoneSecurityEnabled": false,
            "ZoneSecurityIncludeHashRemoteIP": false,
            "IgnoreQueryStrings": true,
            "MonthlyBandwidthLimit": 1073741824,
            "MonthlyBandwidthUsed": 536870912,
            "MonthlyCharges": 0.05,
            "AddHostHeader": false,
            "Type": 0,
            "EnableAccessControlOriginHeader": true,
            "DisableCookies": false,
            "EnableOriginShield": false,
            "CacheControlMaxAgeOverride": 86400,
            "CacheControlPublicMaxAgeOverride": 86400,
            "BurstSize": 0,
            "RequestLimit": 0,
            "BlockRootPathAccess": false,
            "BlockPostRequests": false,
            "LimitRatePerSecond": 0.0,
            "LimitRateAfter": 0.0,
            "ConnectionLimitPerIPCount": 0,
            "PriceOverride": 0.0,
            "OptimizerPricing": 0.0,
            "AddCanonicalHeader": false,
            "EnableLogging": true,
            "EnableCacheSlice": false,
            "EnableSmartCache": false,
            "EdgeRules": [],
            "EnableWebPVary": false,
            "EnableAvifVary": false,
            "EnableCountryCodeVary": false,
            "EnableCountryStateCodeVary": false,
            "EnableMobileVary": false,
            "EnableCookieVary": false,
            "EnableHostnameVary": false,
            "AWSSigningEnabled": false,
            "LoggingIPAnonymizationEnabled": false,
            "EnableTLS1": false,
            "EnableTLS1_1": false,
            "VerifyOriginSSL": true,
            "ErrorPageEnableCustomCode": false,
            "ErrorPageEnableStatuspageWidget": false,
            "ErrorPageWhitelabel": false,
            "LogForwardingEnabled": false,
            "LogForwardingPort": 0,
            "LogForwardingProtocol": 0,
            "LoggingSaveToStorage": false,
            "LoggingStorageZoneId": 0,
            "FollowRedirects": false,
            "VideoLibraryId": 0,
            "DnsRecordId": 0,
            "DnsZoneId": 0,
            "OptimizerEnabled": false,
            "OptimizerTunnelEnabled": false,
            "OptimizerDesktopMaxWidth": 1600,
            "OptimizerMobileMaxWidth": 800,
            "OptimizerImageQuality": 85,
            "OptimizerMobileImageQuality": 70,
            "OptimizerEnableWebP": true,
            "OptimizerPrerenderHtml": false,
            "OptimizerEnableManipulationEngine": false,
            "OptimizerMinifyCSS": false,
            "OptimizerMinifyJavaScript": false,
            "OptimizerWatermarkEnabled": false,
            "OptimizerWatermarkPosition": 0,
            "OptimizerWatermarkOffset": 0.0,
            "OptimizerWatermarkMinImageSize": 0,
            "OptimizerAutomaticOptimizationEnabled": true,
            "PermaCacheStorageZoneId": 0,
            "PermaCacheType": 0,
            "OriginRetries": 0,
            "OriginConnectTimeout": 10,
            "OriginResponseTimeout": 60,
            "UseStaleWhileUpdating": false,
            "UseStaleWhileOffline": false,
            "OriginRetry5XXResponses": false,
            "OriginRetryConnectionTimeout": true,
            "OriginRetryResponseTimeout": false,
            "OriginRetryDelay": 0,
            "OriginShieldEnableConcurrencyLimit": false,
            "OriginShieldMaxConcurrentRequests": 5000,
            "EnableSafeHop": false,
            "CacheErrorResponses": false,
            "OriginShieldQueueMaxWaitTime": 30,
            "OriginShieldMaxQueuedRequests": 5000,
            "OptimizerForceClasses": false,
            "OptimizerStaticHtmlEnabled": false,
            "UseBackgroundUpdate": false,
            "EnableAutoSSL": true,
            "EnableQueryStringOrdering": false,
            "LogAnonymizationType": 0,
            "LogFormat": 0,
            "LogForwardingFormat": 0,
            "ShieldDDosProtectionType": 1,
            "ShieldDDosProtectionEnabled": false,
            "OriginType": 0,
            "EnableRequestCoalescing": false,
            "RequestCoalescingTimeout": 0,
            "DisableLetsEncrypt": false,
            "EnableBunnyImageAi": false,
            "PreloadingScreenEnabled": false,
            "PreloadingScreenShowOnFirstVisit": false,
            "PreloadingScreenCodeEnabled": false,
            "PreloadingScreenTheme": 0,
            "PreloadingScreenDelay": 0,
            "EUUSDiscount": 0,
            "SouthAmericaDiscount": 0,
            "AfricaDiscount": 0,
            "AsiaOceaniaDiscount": 0,
            "BlockNoneReferrer": false,
            "StickySessionType": 0,
            "CacheVersion": 1,
            "OptimizerEnableUpscaling": false,
            "EnableWebSockets": false,
            "MaxWebSocketConnections": 0,
            "EnableExtendedLogging": false
        }"#;

        let pz: PullZone = serde_json::from_str(json).unwrap();
        assert_eq!(pz.id, 12345);
        assert_eq!(pz.name, Some("my-cdn".to_string()));
        assert_eq!(pz.origin_url, Some("https://example.com".to_string()));
        assert_eq!(pz.enabled, Some(true));
        assert_eq!(pz.suspended, Some(false));
        assert_eq!(pz.storage_zone_id, Some(0));
        assert_eq!(pz.monthly_bandwidth_limit, Some(1073741824));
        assert_eq!(pz.monthly_bandwidth_used, Some(536870912));
        assert_eq!(pz.pull_zone_type, Some(PullZoneType::Premium));
        assert_eq!(pz.origin_type, Some(PullZoneOriginType::OriginUrl));
        assert_eq!(
            pz.shield_ddos_protection_type,
            Some(ShieldDDosProtectionType::ActiveStandard)
        );
        assert_eq!(pz.enable_tls1, Some(false));
        assert_eq!(pz.enable_tls1_1, Some(false));
        assert_eq!(pz.verify_origin_ssl, Some(true));
        assert_eq!(pz.enable_auto_ssl, Some(true));
        assert_eq!(pz.optimizer_minify_css, Some(false));
        assert_eq!(pz.origin_retry_5xx_responses, Some(false));
        assert_eq!(pz.enable_geo_zone_us, Some(true));
        assert_eq!(pz.enable_geo_zone_asia, Some(false));
        assert_eq!(pz.blocked_ips, Some(vec!["1.2.3.4".to_string()]));
        assert_eq!(pz.connection_limit_per_ip_count, Some(0));
        assert_eq!(pz.dns_record_id, Some(0));
        assert_eq!(pz.dns_zone_id, Some(0));
        assert_eq!(pz.euus_discount, Some(0));
        assert_eq!(pz.aws_signing_enabled, Some(false));
        assert_eq!(pz.logging_ip_anonymization_enabled, Some(false));

        // Test hostnames
        let hostnames = pz.hostnames.unwrap();
        assert_eq!(hostnames.len(), 1);
        assert_eq!(hostnames[0].value, Some("cdn.example.com".to_string()));
        assert_eq!(hostnames[0].force_ssl, Some(true));
        assert_eq!(hostnames[0].has_certificate, Some(true));
    }

    #[test]
    fn test_pull_zone_row() {
        let pz = PullZone {
            id: 100,
            name: Some("test-zone".to_string()),
            origin_url: Some("https://origin.example.com".to_string()),
            enabled: Some(true),
            origin_type: Some(PullZoneOriginType::OriginUrl),
            monthly_bandwidth_used: Some(1048576),
            hostnames: Some(vec![
                HostnameModel {
                    id: Some(1),
                    value: Some("cdn.test.com".to_string()),
                    force_ssl: Some(true),
                    is_system_hostname: Some(false),
                    is_managed_hostname: Some(false),
                    has_certificate: Some(true),
                    certificate: None,
                    certificate_key: None,
                    certificate_key_type: None,
                    certificate_provision_type: None,
                },
                HostnameModel {
                    id: Some(2),
                    value: Some("cdn2.test.com".to_string()),
                    force_ssl: Some(false),
                    is_system_hostname: Some(true),
                    is_managed_hostname: Some(false),
                    has_certificate: Some(false),
                    certificate: None,
                    certificate_key: None,
                    certificate_key_type: None,
                    certificate_provision_type: None,
                },
            ]),
            // All remaining fields None for simplicity
            suspended: None,
            storage_zone_id: None,
            edge_script_id: None,
            edge_script_execution_phase: None,
            middleware_script_id: None,
            magic_containers_app_id: None,
            magic_containers_endpoint_id: None,
            allowed_referrers: None,
            blocked_referrers: None,
            blocked_ips: None,
            enable_geo_zone_us: None,
            enable_geo_zone_eu: None,
            enable_geo_zone_asia: None,
            enable_geo_zone_sa: None,
            enable_geo_zone_af: None,
            zone_security_enabled: None,
            zone_security_key: None,
            zone_security_include_hash_remote_ip: None,
            ignore_query_strings: None,
            monthly_bandwidth_limit: None,
            monthly_charges: None,
            add_host_header: None,
            origin_host_header: None,
            pull_zone_type: None,
            access_control_origin_header_extensions: None,
            enable_access_control_origin_header: None,
            disable_cookies: None,
            budget_redirected_countries: None,
            blocked_countries: None,
            enable_origin_shield: None,
            cache_control_max_age_override: None,
            cache_control_public_max_age_override: None,
            burst_size: None,
            request_limit: None,
            block_root_path_access: None,
            block_post_requests: None,
            limit_rate_per_second: None,
            limit_rate_after: None,
            connection_limit_per_ip_count: None,
            price_override: None,
            optimizer_pricing: None,
            add_canonical_header: None,
            enable_logging: None,
            enable_cache_slice: None,
            enable_smart_cache: None,
            edge_rules: None,
            enable_web_p_vary: None,
            enable_avif_vary: None,
            enable_country_code_vary: None,
            enable_country_state_code_vary: None,
            enable_mobile_vary: None,
            enable_cookie_vary: None,
            cookie_vary_parameters: None,
            enable_hostname_vary: None,
            cname_domain: None,
            aws_signing_enabled: None,
            aws_signing_key: None,
            aws_signing_secret: None,
            aws_signing_region_name: None,
            logging_ip_anonymization_enabled: None,
            enable_tls1: None,
            enable_tls1_1: None,
            verify_origin_ssl: None,
            error_page_enable_custom_code: None,
            error_page_custom_code: None,
            error_page_enable_statuspage_widget: None,
            error_page_statuspage_code: None,
            error_page_whitelabel: None,
            origin_shield_zone_code: None,
            log_forwarding_enabled: None,
            log_forwarding_hostname: None,
            log_forwarding_port: None,
            log_forwarding_token: None,
            log_forwarding_protocol: None,
            logging_save_to_storage: None,
            logging_storage_zone_id: None,
            follow_redirects: None,
            video_library_id: None,
            dns_record_id: None,
            dns_zone_id: None,
            dns_record_value: None,
            optimizer_enabled: None,
            optimizer_tunnel_enabled: None,
            optimizer_desktop_max_width: None,
            optimizer_mobile_max_width: None,
            optimizer_image_quality: None,
            optimizer_mobile_image_quality: None,
            optimizer_enable_web_p: None,
            optimizer_prerender_html: None,
            optimizer_enable_manipulation_engine: None,
            optimizer_minify_css: None,
            optimizer_minify_java_script: None,
            optimizer_watermark_enabled: None,
            optimizer_watermark_url: None,
            optimizer_watermark_position: None,
            optimizer_watermark_offset: None,
            optimizer_watermark_min_image_size: None,
            optimizer_automatic_optimization_enabled: None,
            perma_cache_storage_zone_id: None,
            perma_cache_type: None,
            origin_retries: None,
            origin_connect_timeout: None,
            origin_response_timeout: None,
            use_stale_while_updating: None,
            use_stale_while_offline: None,
            origin_retry_5xx_responses: None,
            origin_retry_connection_timeout: None,
            origin_retry_response_timeout: None,
            origin_retry_delay: None,
            query_string_vary_parameters: None,
            origin_shield_enable_concurrency_limit: None,
            origin_shield_max_concurrent_requests: None,
            enable_safe_hop: None,
            cache_error_responses: None,
            origin_shield_queue_max_wait_time: None,
            origin_shield_max_queued_requests: None,
            optimizer_classes: None,
            optimizer_force_classes: None,
            optimizer_static_html_enabled: None,
            optimizer_static_html_word_press_path: None,
            optimizer_static_html_word_press_bypass_cookie: None,
            use_background_update: None,
            enable_auto_ssl: None,
            enable_query_string_ordering: None,
            log_anonymization_type: None,
            log_format: None,
            log_forwarding_format: None,
            shield_ddos_protection_type: None,
            shield_ddos_protection_enabled: None,
            enable_request_coalescing: None,
            request_coalescing_timeout: None,
            origin_link_value: None,
            disable_lets_encrypt: None,
            enable_bunny_image_ai: None,
            bunny_ai_image_blueprints: None,
            preloading_screen_enabled: None,
            preloading_screen_show_on_first_visit: None,
            preloading_screen_code: None,
            preloading_screen_logo_url: None,
            preloading_screen_code_enabled: None,
            preloading_screen_theme: None,
            preloading_screen_delay: None,
            euus_discount: None,
            south_america_discount: None,
            africa_discount: None,
            asia_oceania_discount: None,
            routing_filters: None,
            block_none_referrer: None,
            sticky_session_type: None,
            sticky_session_cookie_name: None,
            sticky_session_client_headers: None,
            user_id: None,
            cache_version: None,
            optimizer_enable_upscaling: None,
            enable_web_sockets: None,
            max_web_socket_connections: None,
            enable_extended_logging: None,
        };

        let row = PullZoneRow::from(&pz);
        assert_eq!(row.id, "100");
        assert_eq!(row.name, "test-zone");
        assert_eq!(row.origin_url, "https://origin.example.com");
        assert_eq!(row.origin_type, "OriginUrl");
        assert_eq!(row.enabled, "Yes");
        assert_eq!(row.monthly_bandwidth, "1048576");
        assert_eq!(row.hostnames, "2");
    }

    #[test]
    fn test_pull_zone_row_defaults() {
        let pz = PullZone {
            id: 1,
            name: None,
            origin_url: None,
            enabled: None,
            origin_type: None,
            monthly_bandwidth_used: None,
            hostnames: None,
            suspended: None,
            storage_zone_id: None,
            edge_script_id: None,
            edge_script_execution_phase: None,
            middleware_script_id: None,
            magic_containers_app_id: None,
            magic_containers_endpoint_id: None,
            allowed_referrers: None,
            blocked_referrers: None,
            blocked_ips: None,
            enable_geo_zone_us: None,
            enable_geo_zone_eu: None,
            enable_geo_zone_asia: None,
            enable_geo_zone_sa: None,
            enable_geo_zone_af: None,
            zone_security_enabled: None,
            zone_security_key: None,
            zone_security_include_hash_remote_ip: None,
            ignore_query_strings: None,
            monthly_bandwidth_limit: None,
            monthly_charges: None,
            add_host_header: None,
            origin_host_header: None,
            pull_zone_type: None,
            access_control_origin_header_extensions: None,
            enable_access_control_origin_header: None,
            disable_cookies: None,
            budget_redirected_countries: None,
            blocked_countries: None,
            enable_origin_shield: None,
            cache_control_max_age_override: None,
            cache_control_public_max_age_override: None,
            burst_size: None,
            request_limit: None,
            block_root_path_access: None,
            block_post_requests: None,
            limit_rate_per_second: None,
            limit_rate_after: None,
            connection_limit_per_ip_count: None,
            price_override: None,
            optimizer_pricing: None,
            add_canonical_header: None,
            enable_logging: None,
            enable_cache_slice: None,
            enable_smart_cache: None,
            edge_rules: None,
            enable_web_p_vary: None,
            enable_avif_vary: None,
            enable_country_code_vary: None,
            enable_country_state_code_vary: None,
            enable_mobile_vary: None,
            enable_cookie_vary: None,
            cookie_vary_parameters: None,
            enable_hostname_vary: None,
            cname_domain: None,
            aws_signing_enabled: None,
            aws_signing_key: None,
            aws_signing_secret: None,
            aws_signing_region_name: None,
            logging_ip_anonymization_enabled: None,
            enable_tls1: None,
            enable_tls1_1: None,
            verify_origin_ssl: None,
            error_page_enable_custom_code: None,
            error_page_custom_code: None,
            error_page_enable_statuspage_widget: None,
            error_page_statuspage_code: None,
            error_page_whitelabel: None,
            origin_shield_zone_code: None,
            log_forwarding_enabled: None,
            log_forwarding_hostname: None,
            log_forwarding_port: None,
            log_forwarding_token: None,
            log_forwarding_protocol: None,
            logging_save_to_storage: None,
            logging_storage_zone_id: None,
            follow_redirects: None,
            video_library_id: None,
            dns_record_id: None,
            dns_zone_id: None,
            dns_record_value: None,
            optimizer_enabled: None,
            optimizer_tunnel_enabled: None,
            optimizer_desktop_max_width: None,
            optimizer_mobile_max_width: None,
            optimizer_image_quality: None,
            optimizer_mobile_image_quality: None,
            optimizer_enable_web_p: None,
            optimizer_prerender_html: None,
            optimizer_enable_manipulation_engine: None,
            optimizer_minify_css: None,
            optimizer_minify_java_script: None,
            optimizer_watermark_enabled: None,
            optimizer_watermark_url: None,
            optimizer_watermark_position: None,
            optimizer_watermark_offset: None,
            optimizer_watermark_min_image_size: None,
            optimizer_automatic_optimization_enabled: None,
            perma_cache_storage_zone_id: None,
            perma_cache_type: None,
            origin_retries: None,
            origin_connect_timeout: None,
            origin_response_timeout: None,
            use_stale_while_updating: None,
            use_stale_while_offline: None,
            origin_retry_5xx_responses: None,
            origin_retry_connection_timeout: None,
            origin_retry_response_timeout: None,
            origin_retry_delay: None,
            query_string_vary_parameters: None,
            origin_shield_enable_concurrency_limit: None,
            origin_shield_max_concurrent_requests: None,
            enable_safe_hop: None,
            cache_error_responses: None,
            origin_shield_queue_max_wait_time: None,
            origin_shield_max_queued_requests: None,
            optimizer_classes: None,
            optimizer_force_classes: None,
            optimizer_static_html_enabled: None,
            optimizer_static_html_word_press_path: None,
            optimizer_static_html_word_press_bypass_cookie: None,
            use_background_update: None,
            enable_auto_ssl: None,
            enable_query_string_ordering: None,
            log_anonymization_type: None,
            log_format: None,
            log_forwarding_format: None,
            shield_ddos_protection_type: None,
            shield_ddos_protection_enabled: None,
            enable_request_coalescing: None,
            request_coalescing_timeout: None,
            origin_link_value: None,
            disable_lets_encrypt: None,
            enable_bunny_image_ai: None,
            bunny_ai_image_blueprints: None,
            preloading_screen_enabled: None,
            preloading_screen_show_on_first_visit: None,
            preloading_screen_code: None,
            preloading_screen_logo_url: None,
            preloading_screen_code_enabled: None,
            preloading_screen_theme: None,
            preloading_screen_delay: None,
            euus_discount: None,
            south_america_discount: None,
            africa_discount: None,
            asia_oceania_discount: None,
            routing_filters: None,
            block_none_referrer: None,
            sticky_session_type: None,
            sticky_session_cookie_name: None,
            sticky_session_client_headers: None,
            user_id: None,
            cache_version: None,
            optimizer_enable_upscaling: None,
            enable_web_sockets: None,
            max_web_socket_connections: None,
            enable_extended_logging: None,
        };

        let row = PullZoneRow::from(&pz);
        assert_eq!(row.id, "1");
        assert_eq!(row.name, "-");
        assert_eq!(row.origin_url, "-");
        assert_eq!(row.origin_type, "-");
        assert_eq!(row.enabled, "-");
        assert_eq!(row.monthly_bandwidth, "-");
        assert_eq!(row.hostnames, "0");
    }

    #[test]
    fn test_enum_serde() {
        // PullZoneType
        let premium: PullZoneType = serde_json::from_str("0").unwrap();
        assert_eq!(premium, PullZoneType::Premium);
        let volume: PullZoneType = serde_json::from_str("1").unwrap();
        assert_eq!(volume, PullZoneType::Volume);

        // PullZoneOriginType
        let origin: PullZoneOriginType = serde_json::from_str("0").unwrap();
        assert_eq!(origin, PullZoneOriginType::OriginUrl);
        let storage: PullZoneOriginType = serde_json::from_str("2").unwrap();
        assert_eq!(storage, PullZoneOriginType::StorageZone);

        // EdgeRuleActionType
        let force_ssl: EdgeRuleActionType = serde_json::from_str("0").unwrap();
        assert_eq!(force_ssl, EdgeRuleActionType::ForceSSL);
        let redirect: EdgeRuleActionType = serde_json::from_str("1").unwrap();
        assert_eq!(redirect, EdgeRuleActionType::Redirect);
        let disable_rc: EdgeRuleActionType = serde_json::from_str("34").unwrap();
        assert_eq!(disable_rc, EdgeRuleActionType::DisableRequestCoalescing);

        // TriggerTypes
        let url: TriggerTypes = serde_json::from_str("0").unwrap();
        assert_eq!(url, TriggerTypes::Url);
        let remote_ip: TriggerTypes = serde_json::from_str("5").unwrap();
        assert_eq!(remote_ip, TriggerTypes::RemoteIP);

        // ShieldDDosProtectionType
        let detect: ShieldDDosProtectionType = serde_json::from_str("0").unwrap();
        assert_eq!(detect, ShieldDDosProtectionType::DetectOnly);
        let aggressive: ShieldDDosProtectionType = serde_json::from_str("2").unwrap();
        assert_eq!(aggressive, ShieldDDosProtectionType::ActiveAggressive);

        // ExecutionPhase
        let cache: ExecutionPhase = serde_json::from_str("0").unwrap();
        assert_eq!(cache, ExecutionPhase::Cache);
        let lb: ExecutionPhase = serde_json::from_str("1").unwrap();
        assert_eq!(lb, ExecutionPhase::LoadBalancer);

        // CertificateProvisionType
        let managed: CertificateProvisionType = serde_json::from_str("4").unwrap();
        assert_eq!(managed, CertificateProvisionType::Managed);

        // EdgeRuleActionType
        let strip_cookies: EdgeRuleActionType = serde_json::from_str("37").unwrap();
        assert_eq!(
            strip_cookies,
            EdgeRuleActionType::StripCookiesClientToOrigin
        );
    }

    #[test]
    fn test_deserialize_edge_rule() {
        let json = r#"{
            "Guid": "abc-123",
            "ActionType": 1,
            "ActionParameter1": "https://redirect.com",
            "ActionParameter2": null,
            "ActionParameter3": null,
            "Triggers": [
                {
                    "Type": 0,
                    "PatternMatches": ["/old-path/*"],
                    "PatternMatchingType": 0,
                    "Parameter1": null
                }
            ],
            "ExtraActions": [
                {
                    "ActionType": 5,
                    "ActionParameter1": "X-Custom",
                    "ActionParameter2": "value",
                    "ActionParameter3": null
                }
            ],
            "TriggerMatchingType": 0,
            "Description": "Redirect old paths",
            "Enabled": true,
            "OrderIndex": 0,
            "ReadOnly": false
        }"#;

        let rule: EdgeRuleV2Model = serde_json::from_str(json).unwrap();
        assert_eq!(rule.guid, Some("abc-123".to_string()));
        assert_eq!(rule.action_type, Some(EdgeRuleActionType::Redirect));
        assert_eq!(
            rule.action_parameter1,
            Some("https://redirect.com".to_string())
        );
        assert_eq!(rule.enabled, Some(true));
        assert_eq!(rule.description, Some("Redirect old paths".to_string()));

        let triggers = rule.triggers.unwrap();
        assert_eq!(triggers.len(), 1);
        assert_eq!(triggers[0].r#type, Some(TriggerTypes::Url));
        assert_eq!(
            triggers[0].pattern_matches,
            Some(vec!["/old-path/*".to_string()])
        );

        let extra = rule.extra_actions.unwrap();
        assert_eq!(extra.len(), 1);
        assert_eq!(
            extra[0].action_type,
            Some(EdgeRuleActionType::SetResponseHeader)
        );
    }

    #[test]
    fn test_deserialize_hostname_model() {
        let json = r#"{
            "Id": 42,
            "Value": "cdn.example.com",
            "ForceSSL": true,
            "IsSystemHostname": false,
            "IsManagedHostname": false,
            "HasCertificate": true,
            "Certificate": "base64cert",
            "CertificateKey": "base64key",
            "CertificateKeyType": 0,
            "CertificateProvisionType": 2
        }"#;

        let hostname: HostnameModel = serde_json::from_str(json).unwrap();
        assert_eq!(hostname.id, Some(42));
        assert_eq!(hostname.value, Some("cdn.example.com".to_string()));
        assert_eq!(hostname.force_ssl, Some(true));
        assert_eq!(hostname.certificate_key_type, Some(PrivateKeyType::Ecdsa));
        assert_eq!(
            hostname.certificate_provision_type,
            Some(CertificateProvisionType::Dns01)
        );
    }

    #[test]
    fn test_deserialize_optimizer_statistics() {
        let json = r#"{
            "RequestsOptimizedChart": {"2024-01-01": 100, "2024-01-02": 200},
            "AverageCompressionChart": {"2024-01-01": 50},
            "TrafficSavedChart": {"2024-01-01": 1024},
            "AverageProcessingTimeChart": {"2024-01-01": 15},
            "TotalRequestsOptimized": 300.0,
            "TotalTrafficSaved": 2048.0,
            "AverageProcessingTime": 12.5,
            "AverageCompressionRatio": 0.65
        }"#;

        let stats: OptimizerStatisticsModel = serde_json::from_str(json).unwrap();
        assert_eq!(stats.total_requests_optimized, Some(300.0));
        assert_eq!(stats.total_traffic_saved, Some(2048.0));
        assert_eq!(stats.average_processing_time, Some(12.5));
        assert_eq!(stats.average_compression_ratio, Some(0.65));
        assert!(stats.requests_optimized_chart.is_some());
    }

    #[test]
    fn test_deserialize_origin_shield_statistics() {
        let json = r#"{
            "ConcurrentRequestsChart": {"2024-01-01": 50},
            "QueuedRequestsChart": {"2024-01-01": 10}
        }"#;

        let stats: OriginShieldConcurrencyStatisticsModel = serde_json::from_str(json).unwrap();
        let concurrent = stats.concurrent_requests_chart.unwrap();
        assert_eq!(concurrent.get("2024-01-01"), Some(&50));
        let queued = stats.queued_requests_chart.unwrap();
        assert_eq!(queued.get("2024-01-01"), Some(&10));
    }

    #[test]
    fn test_deserialize_safehop_statistics() {
        let json = r#"{
            "RequestsRetriedChart": {"2024-01-01": 25},
            "RequestsSavedChart": {"2024-01-01": 75},
            "TotalRequestsRetried": 25.0,
            "TotalRequestsSaved": 75.0
        }"#;

        let stats: SafeHopStatisticsModel = serde_json::from_str(json).unwrap();
        assert_eq!(stats.total_requests_retried, Some(25.0));
        assert_eq!(stats.total_requests_saved, Some(75.0));
    }
}
