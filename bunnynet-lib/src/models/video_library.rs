use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use tabled::Tabled;

// --- Enums (serde_repr) ---

/// DRM version: Basic (0), Enterprise (1), BasicV2 (2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum DrmVersion {
    Basic = 0,
    Enterprise = 1,
    BasicV2 = 2,
}

/// Encoding tier: Free (0), Premium (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum EncodingTier {
    Free = 0,
    Premium = 1,
}

/// Pull zone type for video library: Premium (0), Volume (1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum VideoLibraryPullZoneType {
    Premium = 0,
    Volume = 1,
}

/// Widevine minimum client security level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum WidevineMinClientSecurityLevel {
    None = 0,
    L1 = 1,
    L2 = 2,
    L3 = 3,
}

// --- Nested models ---

/// Apple FairPlay DRM configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AppleFairPlayDrm {
    pub enabled: Option<bool>,
    pub certificate_id: Option<i64>,
    pub certificate_expiration_date: Option<String>,
    pub provider: Option<String>,
}

/// Google Widevine DRM configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GoogleWidevineDrm {
    pub enabled: Option<bool>,
    pub certificate_id: Option<i64>,
    pub certificate_expiration_date: Option<String>,
    pub provider: Option<String>,
    #[serde(rename = "SdOnlyForL3")]
    pub sd_only_for_l3: Option<bool>,
    pub min_client_security_level: Option<WidevineMinClientSecurityLevel>,
}

// --- Main VideoLibrary model ---

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VideoLibrary {
    pub id: i64,
    pub name: Option<String>,
    pub video_count: Option<i64>,
    pub traffic_usage: Option<i64>,
    pub storage_usage: Option<i64>,
    pub date_created: Option<String>,
    pub date_modified: Option<String>,
    pub replication_regions: Option<Vec<String>>,
    #[serde(rename = "ApiKey")]
    pub api_key: Option<String>,
    #[serde(rename = "ReadOnlyApiKey")]
    pub read_only_api_key: Option<String>,
    pub has_watermark: Option<bool>,
    pub watermark_position_left: Option<i32>,
    pub watermark_position_top: Option<i32>,
    pub watermark_width: Option<i32>,
    pub pull_zone_id: Option<i64>,
    pub storage_zone_id: Option<i64>,
    pub watermark_height: Option<i32>,
    pub enabled_resolutions: Option<String>,
    #[serde(rename = "ViAiPublisherId")]
    pub vi_ai_publisher_id: Option<String>,
    #[serde(rename = "VastTagUrl")]
    pub vast_tag_url: Option<String>,
    #[serde(rename = "WebhookUrl")]
    pub webhook_url: Option<String>,
    pub captions_font_size: Option<i32>,
    pub captions_font_color: Option<String>,
    pub captions_background: Option<String>,
    #[serde(rename = "UILanguage")]
    pub ui_language: Option<String>,
    pub allow_early_play: Option<bool>,
    pub player_token_authentication_enabled: Option<bool>,
    pub allowed_referrers: Option<Vec<String>>,
    pub blocked_referrers: Option<Vec<String>>,
    pub block_none_referrer: Option<bool>,
    #[serde(rename = "EnableMP4Fallback")]
    pub enable_mp4_fallback: Option<bool>,
    pub keep_original_files: Option<bool>,
    pub allow_direct_play: Option<bool>,
    #[serde(rename = "EnableDRM")]
    pub enable_drm: Option<bool>,
    pub drm_version: Option<DrmVersion>,
    pub apple_fair_play_drm: Option<AppleFairPlayDrm>,
    pub google_widevine_drm: Option<GoogleWidevineDrm>,
    pub bitrate240p: Option<i32>,
    pub bitrate360p: Option<i32>,
    pub bitrate480p: Option<i32>,
    pub bitrate720p: Option<i32>,
    pub bitrate1080p: Option<i32>,
    pub bitrate1440p: Option<i32>,
    pub bitrate2160p: Option<i32>,
    #[serde(rename = "ApiAccessKey")]
    pub api_access_key: Option<String>,
    pub show_heatmap: Option<bool>,
    pub enable_content_tagging: Option<bool>,
    pub pull_zone_type: Option<VideoLibraryPullZoneType>,
    #[serde(rename = "CustomHTML")]
    pub custom_html: Option<String>,
    pub controls: Option<String>,
    pub playback_speeds: Option<String>,
    pub player_key_color: Option<String>,
    pub font_family: Option<String>,
    pub watermark_version: Option<i64>,
    pub enable_transcribing: Option<bool>,
    pub enable_transcribing_title_generation: Option<bool>,
    pub enable_transcribing_description_generation: Option<bool>,
    pub enable_transcribing_chapters_generation: Option<bool>,
    pub enable_transcribing_moments_generation: Option<bool>,
    pub transcribing_caption_languages: Option<Vec<String>>,
    pub enable_captions_in_playlist: Option<bool>,
    pub remember_player_position: Option<bool>,
    pub enable_multi_audio_track_support: Option<bool>,
    pub use_separate_audio_stream: Option<bool>,
    #[serde(rename = "JitEncodingEnabled")]
    pub jit_encoding_enabled: Option<bool>,
    pub encoding_tier: Option<EncodingTier>,
    pub output_codecs: Option<String>,
    pub drm_base_price_override: Option<f64>,
    pub drm_cost_per_license_override: Option<f64>,
    pub transcribing_price_override: Option<f64>,
    pub premium_encoding_price_override: Option<f64>,
    pub monthly_charges_transcribing: Option<f64>,
    pub monthly_charges_premium_encoding: Option<f64>,
    pub monthly_charges_enterprise_drm: Option<f64>,
    pub feature_flags: Option<String>,
    pub player_version: Option<i32>,
    pub remove_metadata_from_fallback_videos: Option<bool>,
    pub scale_video_using_both_dimensions: Option<bool>,
    pub expose_originals: Option<bool>,
    pub expose_video_metadata: Option<bool>,
}

// --- Statistics models ---

/// Video library DRM statistics model
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VideoLibraryDrmStatisticsModel {
    pub total_licenses_issued: Option<i64>,
    pub licenses_issued_chart: Option<HashMap<String, f64>>,
}

/// Video library transcription statistics model
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VideoLibraryTranscriptionStatisticsModel {
    pub total_transcription_seconds: Option<i64>,
    pub transcription_seconds_chart: Option<HashMap<String, f64>>,
}

// --- Language model ---

/// Language entry returned by /videolibrary/languages
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VideoLibraryLanguage {
    pub short_code: Option<String>,
    pub name: Option<String>,
    pub supported_by_tts: Option<bool>,
}

// --- Display row for table output ---

#[derive(Debug, Tabled)]
pub struct VideoLibraryRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "API KEY")]
    pub api_key: String,
    #[tabled(rename = "PULL ZONE ID")]
    pub pull_zone_id: String,
    #[tabled(rename = "STORAGE ZONE ID")]
    pub storage_zone_id: String,
    #[tabled(rename = "DRM ENABLED")]
    pub drm_enabled: String,
}

impl From<&VideoLibrary> for VideoLibraryRow {
    fn from(vl: &VideoLibrary) -> Self {
        VideoLibraryRow {
            id: vl.id.to_string(),
            name: vl.name.clone().unwrap_or_else(|| "-".to_string()),
            api_key: vl
                .api_key
                .as_ref()
                .map(|k| {
                    if k.len() > 8 {
                        format!("{}...", &k[..8])
                    } else {
                        k.clone()
                    }
                })
                .unwrap_or_else(|| "-".to_string()),
            pull_zone_id: vl
                .pull_zone_id
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            storage_zone_id: vl
                .storage_zone_id
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            drm_enabled: vl
                .enable_drm
                .map(|e| if e { "Yes" } else { "No" }.to_string())
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

/// Language row for table output
#[derive(Debug, Tabled)]
pub struct VideoLibraryLanguageRow {
    #[tabled(rename = "CODE")]
    pub code: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "TTS SUPPORTED")]
    pub tts_supported: String,
}

impl From<&VideoLibraryLanguage> for VideoLibraryLanguageRow {
    fn from(lang: &VideoLibraryLanguage) -> Self {
        VideoLibraryLanguageRow {
            code: lang.short_code.clone().unwrap_or_else(|| "-".to_string()),
            name: lang.name.clone().unwrap_or_else(|| "-".to_string()),
            tts_supported: lang
                .supported_by_tts
                .map(|v| if v { "Yes" } else { "No" }.to_string())
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_video_library() {
        let json = r##"{
            "Id": 5001,
            "Name": "my-video-lib",
            "VideoCount": 42,
            "TrafficUsage": 1073741824,
            "StorageUsage": 536870912,
            "DateCreated": "2024-01-15T10:30:00Z",
            "DateModified": "2024-06-20T14:45:00Z",
            "ReplicationRegions": ["DE", "NY"],
            "ApiKey": "abc123-api-key",
            "ReadOnlyApiKey": "ro-key-123",
            "HasWatermark": false,
            "WatermarkPositionLeft": 70,
            "WatermarkPositionTop": 70,
            "WatermarkWidth": 15,
            "PullZoneId": 2001,
            "StorageZoneId": 3001,
            "WatermarkHeight": 10,
            "EnabledResolutions": "720p,1080p,1440p",
            "ViAiPublisherId": null,
            "VastTagUrl": null,
            "WebhookUrl": null,
            "CaptionsFontSize": 20,
            "CaptionsFontColor": "#fff",
            "CaptionsBackground": "#000",
            "UILanguage": "en",
            "AllowEarlyPlay": true,
            "PlayerTokenAuthenticationEnabled": false,
            "AllowedReferrers": ["example.com"],
            "BlockedReferrers": [],
            "BlockNoneReferrer": false,
            "EnableMP4Fallback": true,
            "KeepOriginalFiles": true,
            "AllowDirectPlay": true,
            "EnableDRM": false,
            "DrmVersion": 0,
            "AppleFairPlayDrm": {
                "Enabled": false,
                "CertificateId": null,
                "CertificateExpirationDate": null,
                "Provider": null
            },
            "GoogleWidevineDrm": {
                "Enabled": false,
                "CertificateId": null,
                "CertificateExpirationDate": null,
                "Provider": null,
                "SdOnlyForL3": false,
                "MinClientSecurityLevel": null
            },
            "Bitrate240p": 600,
            "Bitrate360p": 800,
            "Bitrate480p": 1400,
            "Bitrate720p": 2800,
            "Bitrate1080p": 5000,
            "Bitrate1440p": 8000,
            "Bitrate2160p": 13000,
            "ApiAccessKey": "abc123-api-key",
            "ShowHeatmap": true,
            "EnableContentTagging": true,
            "PullZoneType": 0,
            "CustomHTML": null,
            "Controls": "play-large,play,progress,current-time,mute,volume,captions,settings,pip,airplay,fullscreen",
            "PlaybackSpeeds": "0.5,0.75,1,1.25,1.5,2",
            "PlayerKeyColor": "#E83C5C",
            "FontFamily": "Arial",
            "WatermarkVersion": 0,
            "EnableTranscribing": false,
            "EnableTranscribingTitleGeneration": false,
            "EnableTranscribingDescriptionGeneration": false,
            "EnableTranscribingChaptersGeneration": false,
            "EnableTranscribingMomentsGeneration": false,
            "TranscribingCaptionLanguages": [],
            "EnableCaptionsInPlaylist": false,
            "RememberPlayerPosition": false,
            "EnableMultiAudioTrackSupport": false,
            "UseSeparateAudioStream": false,
            "JitEncodingEnabled": false,
            "EncodingTier": 0,
            "OutputCodecs": "x264",
            "DrmBasePriceOverride": null,
            "DrmCostPerLicenseOverride": null,
            "TranscribingPriceOverride": null,
            "PremiumEncodingPriceOverride": null,
            "MonthlyChargesTranscribing": 0.0,
            "MonthlyChargesPremiumEncoding": 0.0,
            "MonthlyChargesEnterpriseDrm": 0.0,
            "FeatureFlags": null,
            "PlayerVersion": 2,
            "RemoveMetadataFromFallbackVideos": false,
            "ScaleVideoUsingBothDimensions": true,
            "ExposeOriginals": false,
            "ExposeVideoMetadata": false
        }"##;

        let vl: VideoLibrary = serde_json::from_str(json).unwrap();
        assert_eq!(vl.id, 5001);
        assert_eq!(vl.name.as_deref(), Some("my-video-lib"));
        assert_eq!(vl.video_count, Some(42));
        assert_eq!(vl.pull_zone_id, Some(2001));
        assert_eq!(vl.storage_zone_id, Some(3001));
        assert_eq!(vl.enable_drm, Some(false));
        assert_eq!(vl.drm_version, Some(DrmVersion::Basic));
        assert_eq!(vl.enable_mp4_fallback, Some(true));
        assert_eq!(vl.encoding_tier, Some(EncodingTier::Free));
        assert_eq!(vl.player_version, Some(2));
        assert_eq!(vl.bitrate720p, Some(2800));
        assert_eq!(vl.bitrate1080p, Some(5000));

        // Nested: AppleFairPlayDrm
        let afp = vl.apple_fair_play_drm.unwrap();
        assert_eq!(afp.enabled, Some(false));
        assert!(afp.certificate_id.is_none());

        // Nested: GoogleWidevineDrm
        let gwd = vl.google_widevine_drm.unwrap();
        assert_eq!(gwd.enabled, Some(false));
        assert_eq!(gwd.sd_only_for_l3, Some(false));
        assert!(gwd.min_client_security_level.is_none());

        // Row conversion
        let row = VideoLibraryRow::from(&serde_json::from_str::<VideoLibrary>(json).unwrap());
        assert_eq!(row.id, "5001");
        assert_eq!(row.name, "my-video-lib");
        assert_eq!(row.api_key, "abc123-a...");
        assert_eq!(row.pull_zone_id, "2001");
        assert_eq!(row.storage_zone_id, "3001");
        assert_eq!(row.drm_enabled, "No");
    }

    #[test]
    fn test_deserialize_drm_statistics() {
        let json = r#"{
            "TotalLicensesIssued": 1500,
            "LicensesIssuedChart": {"2024-01-01": 100.0, "2024-01-02": 200.0}
        }"#;

        let stats: VideoLibraryDrmStatisticsModel = serde_json::from_str(json).unwrap();
        assert_eq!(stats.total_licenses_issued, Some(1500));
        assert!(stats.licenses_issued_chart.is_some());
        assert_eq!(stats.licenses_issued_chart.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_deserialize_transcription_statistics() {
        let json = r#"{
            "TotalTranscriptionSeconds": 36000,
            "TranscriptionSecondsChart": {"2024-01-01": 5000.0, "2024-01-02": 7000.0}
        }"#;

        let stats: VideoLibraryTranscriptionStatisticsModel =
            serde_json::from_str(json).unwrap();
        assert_eq!(stats.total_transcription_seconds, Some(36000));
        assert!(stats.transcription_seconds_chart.is_some());
        assert_eq!(
            stats.transcription_seconds_chart.as_ref().unwrap().len(),
            2
        );
    }

    #[test]
    fn test_deserialize_language() {
        let json = r#"{"ShortCode":"en","Name":"English","SupportedByTts":true}"#;
        let lang: VideoLibraryLanguage = serde_json::from_str(json).unwrap();
        assert_eq!(lang.short_code.as_deref(), Some("en"));
        assert_eq!(lang.name.as_deref(), Some("English"));
        assert_eq!(lang.supported_by_tts, Some(true));

        let row = VideoLibraryLanguageRow::from(&lang);
        assert_eq!(row.code, "en");
        assert_eq!(row.name, "English");
        assert_eq!(row.tts_supported, "Yes");
    }

    #[test]
    fn test_drm_version_enum() {
        let json = "0";
        let v: DrmVersion = serde_json::from_str(json).unwrap();
        assert_eq!(v, DrmVersion::Basic);

        let json = "1";
        let v: DrmVersion = serde_json::from_str(json).unwrap();
        assert_eq!(v, DrmVersion::Enterprise);

        let json = "2";
        let v: DrmVersion = serde_json::from_str(json).unwrap();
        assert_eq!(v, DrmVersion::BasicV2);
    }

    #[test]
    fn test_encoding_tier_enum() {
        let json = "0";
        let v: EncodingTier = serde_json::from_str(json).unwrap();
        assert_eq!(v, EncodingTier::Free);

        let json = "1";
        let v: EncodingTier = serde_json::from_str(json).unwrap();
        assert_eq!(v, EncodingTier::Premium);
    }

    #[test]
    fn test_widevine_min_client_security_level() {
        for (i, expected) in [
            (0, WidevineMinClientSecurityLevel::None),
            (1, WidevineMinClientSecurityLevel::L1),
            (2, WidevineMinClientSecurityLevel::L2),
            (3, WidevineMinClientSecurityLevel::L3),
        ] {
            let json = i.to_string();
            let v: WidevineMinClientSecurityLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(v, expected);
        }
    }
}
