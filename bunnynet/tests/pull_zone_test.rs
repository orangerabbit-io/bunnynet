mod common;

use mockito::Server;
use predicates::prelude::*;

// --- Task 19: CRUD tests ---

#[test]
fn test_pull_zone_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
            "page".to_string(),
            "1".to_string(),
        )]))
        .with_body(common::fixture("pull_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "pull-zone", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("my-cdn"))
        .stdout(predicate::str::contains("static-assets"))
        .stdout(predicate::str::contains("Page 1"));

    mock.assert();
}

#[test]
fn test_pull_zone_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
            "page".to_string(),
            "1".to_string(),
        )]))
        .with_body(common::fixture("pull_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "pull-zone", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Items\""))
        .stdout(predicate::str::contains("\"CurrentPage\""));

    mock.assert();
}

#[test]
fn test_pull_zone_list_with_search_and_pagination() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "2".to_string()),
            mockito::Matcher::UrlEncoded("perPage".to_string(), "10".to_string()),
            mockito::Matcher::UrlEncoded("search".to_string(), "cdn".to_string()),
        ]))
        .with_body(common::fixture("pull_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "list",
        "--page",
        "2",
        "--per-page",
        "10",
        "--search",
        "cdn",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success();

    mock.assert();
}

#[test]
fn test_pull_zone_get_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone/1001")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("pull_zone_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "pull-zone", "get", "1001"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("1001"))
        .stdout(predicate::str::contains("my-cdn"))
        .stdout(predicate::str::contains("origin.example.com"))
        .stdout(predicate::str::contains("cdn.example.com"));

    mock.assert();
}

#[test]
fn test_pull_zone_get_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone/1001")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("pull_zone_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "pull-zone",
        "get",
        "1001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"Id\": 1001"))
    .stdout(predicate::str::contains("\"OriginUrl\""));

    mock.assert();
}

#[test]
fn test_pull_zone_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Name":"new-cdn"}"#.to_string(),
        ))
        .with_body(common::fixture("pull_zone_create.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "create",
        "new-cdn",
        "--origin-url",
        "https://new-origin.example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("new-cdn"))
    .stdout(predicate::str::contains("2001"));

    mock.assert();
}

#[test]
fn test_pull_zone_update() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"OriginUrl":"https://new-origin.example.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "update",
        "1001",
        "--origin-url",
        "https://new-origin.example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("updated"));

    mock.assert();
}

#[test]
fn test_pull_zone_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/pullzone/1001")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "pull-zone", "delete", "1001"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    mock.assert();
}

#[test]
fn test_pull_zone_purge_cache() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001/purgeCache")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "pull-zone", "purge-cache", "1001"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("purged"));

    mock.assert();
}

#[test]
fn test_pull_zone_check_availability() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/checkavailability")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "check-availability",
        "my-new-zone",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("available"));

    mock.assert();
}

// --- Task 20: Hostname + Certificate + Edge Rules ---

#[test]
fn test_pull_zone_hostname_add() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001/addHostname")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Hostname":"cdn2.example.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "hostname",
        "add",
        "1001",
        "--hostname",
        "cdn2.example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("cdn2.example.com"))
    .stdout(predicate::str::contains("added"));

    mock.assert();
}

#[test]
fn test_pull_zone_hostname_remove() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/pullzone/1001/removeHostname")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Hostname":"old.example.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "hostname",
        "remove",
        "1001",
        "--hostname",
        "old.example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("removed"));

    mock.assert();
}

#[test]
fn test_pull_zone_hostname_set_force_ssl() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001/setForceSSL")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Hostname":"cdn.example.com","ForceSSL":true}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "hostname",
        "set-force-ssl",
        "1001",
        "--hostname",
        "cdn.example.com",
        "--force-ssl",
        "true",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("enabled"));

    mock.assert();
}

#[test]
fn test_pull_zone_hostname_set_private_key_type() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001/updatePrivateKeyType")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "hostname",
        "set-private-key-type",
        "1001",
        "--hostname",
        "cdn.example.com",
        "--key-type",
        "1",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Rsa"));

    mock.assert();
}

#[test]
fn test_pull_zone_certificate_remove() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/pullzone/1001/removeCertificate")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Hostname":"cdn.example.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "certificate",
        "remove",
        "1001",
        "--hostname",
        "cdn.example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("removed"));

    mock.assert();
}

#[test]
fn test_pull_zone_certificate_load_free() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone/loadFreeCertificate")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::UrlEncoded(
            "hostname".to_string(),
            "cdn.example.com".to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "certificate",
        "load-free",
        "cdn.example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("loaded"));

    mock.assert();
}

#[test]
fn test_pull_zone_edge_rule_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/pullzone/1001/edgerules/abc-123")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "edge-rule",
        "delete",
        "1001",
        "abc-123",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("deleted"));

    mock.assert();
}

#[test]
fn test_pull_zone_edge_rule_set_enabled() {
    let mut server = Server::new();
    let mock = server
        .mock(
            "POST",
            "/pullzone/1001/edgerules/abc-123/setEdgeRuleEnabled",
        )
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Id":"abc-123","Value":true}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "edge-rule",
        "set-enabled",
        "1001",
        "abc-123",
        "--enabled",
        "true",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("enabled"));

    mock.assert();
}

// --- Task 21: Referrer + Blocked IP + Stats ---

#[test]
fn test_pull_zone_referrer_add_allowed() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001/addAllowedReferrer")
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
        "pull-zone",
        "referrer",
        "add-allowed",
        "1001",
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
fn test_pull_zone_referrer_remove_blocked() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001/removeBlockedReferrer")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Hostname":"spam.example.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "referrer",
        "remove-blocked",
        "1001",
        "--hostname",
        "spam.example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("blocked referrer removed"));

    mock.assert();
}

#[test]
fn test_pull_zone_blocked_ip_add() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001/addBlockedIp")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"BlockedIp":"1.2.3.4"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "blocked-ip",
        "add",
        "1001",
        "--ip",
        "1.2.3.4",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("blocked IP added"));

    mock.assert();
}

#[test]
fn test_pull_zone_blocked_ip_remove() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001/removeBlockedIp")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"BlockedIp":"1.2.3.4"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "blocked-ip",
        "remove",
        "1001",
        "--ip",
        "1.2.3.4",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("blocked IP removed"));

    mock.assert();
}

#[test]
fn test_pull_zone_reset_security_key() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001/resetSecurityKey")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "reset-security-key",
        "1001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("reset"));

    mock.assert();
}

#[test]
fn test_pull_zone_reset_security_key_with_value() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pullzone/1001/resetSecurityKey")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"SecurityKey":"my-custom-key"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "reset-security-key",
        "1001",
        "--security-key",
        "my-custom-key",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("reset"));

    mock.assert();
}

#[test]
fn test_pull_zone_optimizer_statistics_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone/1001/optimizer/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("pull_zone_optimizer_stats.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "optimizer-statistics",
        "1001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("300"))
    .stdout(predicate::str::contains("2048"))
    .stdout(predicate::str::contains("12.50"));

    mock.assert();
}

#[test]
fn test_pull_zone_optimizer_statistics_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone/1001/optimizer/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("pull_zone_optimizer_stats.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "pull-zone",
        "optimizer-statistics",
        "1001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("TotalRequestsOptimized"));

    mock.assert();
}

#[test]
fn test_pull_zone_origin_shield_statistics() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone/1001/originshield/queuestatistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("pull_zone_origin_shield_stats.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "origin-shield-statistics",
        "1001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("1001"))
    .stdout(predicate::str::contains("2"));

    mock.assert();
}

#[test]
fn test_pull_zone_safehop_statistics() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone/1001/safehop/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("pull_zone_safehop_stats.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "safehop-statistics",
        "1001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("55"))
    .stdout(predicate::str::contains("155"));

    mock.assert();
}

#[test]
fn test_pull_zone_statistics_with_date_params() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone/1001/optimizer/statistics")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("dateFrom".to_string(), "2024-01-01".to_string()),
            mockito::Matcher::UrlEncoded("dateTo".to_string(), "2024-01-31".to_string()),
            mockito::Matcher::UrlEncoded("hourly".to_string(), "true".to_string()),
        ]))
        .with_body(common::fixture("pull_zone_optimizer_stats.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "pull-zone",
        "optimizer-statistics",
        "1001",
        "--date-from",
        "2024-01-01",
        "--date-to",
        "2024-01-31",
        "--hourly",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success();

    mock.assert();
}

#[test]
fn test_pull_zone_list_default_page_is_1() {
    // Verify that the default page is 1 (not 0), since Bunny returns
    // a plain array for page=0 but PaginatedList for page>=1
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/pullzone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
            "page".to_string(),
            "1".to_string(),
        )]))
        .with_body(common::fixture("pull_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "pull-zone", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success();

    // The mock asserts page=1 was sent. If it wasn't, mock.assert() would fail.
    mock.assert();
}
