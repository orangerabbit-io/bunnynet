mod common;

use mockito::Server;
use predicates::prelude::*;

// --- Task 22: CRUD + Languages tests ---

#[test]
fn test_video_library_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()),
        ]))
        .with_body(common::fixture("video_library_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "video-library", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("my-videos"))
        .stdout(predicate::str::contains("live-streams"))
        .stdout(predicate::str::contains("Page 1"));

    mock.assert();
}

#[test]
fn test_video_library_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()),
        ]))
        .with_body(common::fixture("video_library_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "video-library",
        "list",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"Items\""))
    .stdout(predicate::str::contains("\"CurrentPage\""));

    mock.assert();
}

#[test]
fn test_video_library_list_with_search_and_pagination() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "2".to_string()),
            mockito::Matcher::UrlEncoded("perPage".to_string(), "10".to_string()),
            mockito::Matcher::UrlEncoded("search".to_string(), "videos".to_string()),
        ]))
        .with_body(common::fixture("video_library_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "list",
        "--page",
        "2",
        "--per-page",
        "10",
        "--search",
        "videos",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success();

    mock.assert();
}

#[test]
fn test_video_library_get_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary/5001")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("video_library_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "video-library", "get", "5001"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("5001"))
        .stdout(predicate::str::contains("my-videos"))
        .stdout(predicate::str::contains("2001"))
        .stdout(predicate::str::contains("3001"));

    mock.assert();
}

#[test]
fn test_video_library_get_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary/5001")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("video_library_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "video-library",
        "get",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"Id\": 5001"))
    .stdout(predicate::str::contains("\"Name\""));

    mock.assert();
}

#[test]
fn test_video_library_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/videolibrary")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Name":"new-video-lib"}"#.to_string(),
        ))
        .with_body(common::fixture("video_library_create.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "create",
        "new-video-lib",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("new-video-lib"))
    .stdout(predicate::str::contains("6001"));

    mock.assert();
}

#[test]
fn test_video_library_create_with_regions() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/videolibrary")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Name":"new-video-lib","ReplicationRegions":["DE","NY"]}"#.to_string(),
        ))
        .with_body(common::fixture("video_library_create.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "create",
        "new-video-lib",
        "--replication-regions",
        "DE,NY",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("6001"));

    mock.assert();
}

#[test]
fn test_video_library_update() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/videolibrary/5001")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "video-library", "update", "5001"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("updated"));

    mock.assert();
}

#[test]
fn test_video_library_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/videolibrary/5001")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "delete",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("deleted"));

    mock.assert();
}

#[test]
fn test_video_library_languages_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary/languages")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("video_library_languages.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "video-library", "languages"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("English"))
        .stdout(predicate::str::contains("German"))
        .stdout(predicate::str::contains("Japanese"));

    mock.assert();
}

#[test]
fn test_video_library_languages_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary/languages")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("video_library_languages.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "video-library",
        "languages",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"ShortCode\""))
    .stdout(predicate::str::contains("\"English\""));

    mock.assert();
}

// --- Task 23: Actions tests ---

#[test]
fn test_video_library_add_allowed_referrer() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/videolibrary/5001/addAllowedReferrer")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Hostname":"example.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "add-allowed-referrer",
        "5001",
        "--hostname",
        "example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("allowed referrer added"));

    mock.assert();
}

#[test]
fn test_video_library_remove_allowed_referrer() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/videolibrary/5001/removeAllowedReferrer")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Hostname":"example.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "remove-allowed-referrer",
        "5001",
        "--hostname",
        "example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("allowed referrer removed"));

    mock.assert();
}

#[test]
fn test_video_library_add_blocked_referrer() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/videolibrary/5001/addBlockedReferrer")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Hostname":"evil.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "add-blocked-referrer",
        "5001",
        "--hostname",
        "evil.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("blocked referrer added"));

    mock.assert();
}

#[test]
fn test_video_library_remove_blocked_referrer() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/videolibrary/5001/removeBlockedReferrer")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Hostname":"evil.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "remove-blocked-referrer",
        "5001",
        "--hostname",
        "evil.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("blocked referrer removed"));

    mock.assert();
}

#[test]
fn test_video_library_reset_api_key() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/videolibrary/5001/resetApiKey")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "reset-api-key",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("API key reset"));

    mock.assert();
}

#[test]
fn test_video_library_reset_read_only_api_key() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/videolibrary/5001/resetReadOnlyApiKey")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "reset-read-only-api-key",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Read-only API key reset"));

    mock.assert();
}

#[test]
fn test_video_library_watermark_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/videolibrary/5001/watermark")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "watermark",
        "delete",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("deleted"));

    mock.assert();
}

#[test]
fn test_video_library_watermark_add() {
    let mut server = Server::new();
    let mock = server
        .mock("PUT", "/videolibrary/5001/watermark")
        .match_header("AccessKey", "test-key")
        .match_header("Content-Type", "image/png")
        .with_status(200)
        .with_body("{}")
        .create();

    // Create a temp file for upload
    let tmp_dir = std::env::temp_dir();
    let tmp_file = tmp_dir.join("test_watermark.png");
    std::fs::write(&tmp_file, b"fake-png-data").unwrap();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "watermark",
        "add",
        "5001",
        "--file",
        tmp_file.to_str().unwrap(),
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("uploaded"));

    mock.assert();
    std::fs::remove_file(&tmp_file).ok();
}

#[test]
fn test_video_library_live_thumbnail_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/videolibrary/5001/live/thumbnail")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "live-thumbnail",
        "delete",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("deleted"));

    mock.assert();
}

#[test]
fn test_video_library_live_watermark_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/videolibrary/5001/live/watermark")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "live-watermark",
        "delete",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("deleted"));

    mock.assert();
}

#[test]
fn test_video_library_drm_statistics_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary/5001/drm/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("video_library_drm_stats.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "drm-statistics",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("1500"))
    .stdout(predicate::str::contains("Data Points"));

    mock.assert();
}

#[test]
fn test_video_library_drm_statistics_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary/5001/drm/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("video_library_drm_stats.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "video-library",
        "drm-statistics",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"TotalLicensesIssued\""));

    mock.assert();
}

#[test]
fn test_video_library_drm_statistics_with_dates() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary/5001/drm/statistics")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded(
                "dateFrom".to_string(),
                "2024-01-01".to_string(),
            ),
            mockito::Matcher::UrlEncoded(
                "dateTo".to_string(),
                "2024-01-31".to_string(),
            ),
        ]))
        .with_body(common::fixture("video_library_drm_stats.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "drm-statistics",
        "5001",
        "--date-from",
        "2024-01-01",
        "--date-to",
        "2024-01-31",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success();

    mock.assert();
}

#[test]
fn test_video_library_transcribing_statistics_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary/5001/transcribing/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("video_library_transcribing_stats.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "video-library",
        "transcribing-statistics",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("36000"))
    .stdout(predicate::str::contains("Data Points"));

    mock.assert();
}

#[test]
fn test_video_library_transcribing_statistics_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary/5001/transcribing/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("video_library_transcribing_stats.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "video-library",
        "transcribing-statistics",
        "5001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"TotalTranscriptionSeconds\""));

    mock.assert();
}

// --- Auth error test ---

#[test]
fn test_video_library_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/videolibrary")
        .match_header("AccessKey", "bad-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()),
        ]))
        .with_status(401)
        .with_body("Unauthorized")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad-key", "video-library", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}
