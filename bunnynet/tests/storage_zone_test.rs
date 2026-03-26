mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_storage_zone_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/storagezone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
            "page".to_string(),
            "1".to_string(),
        )]))
        .with_body(common::fixture("storage_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "storage-zone", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("zone-alpha"))
        .stdout(predicate::str::contains("zone-beta"))
        .stdout(predicate::str::contains("DE"))
        .stdout(predicate::str::contains("NY"))
        .stdout(predicate::str::contains("Page 1"));

    mock.assert();
}

#[test]
fn test_storage_zone_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/storagezone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
            "page".to_string(),
            "1".to_string(),
        )]))
        .with_body(common::fixture("storage_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "storage-zone", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Items\""))
        .stdout(predicate::str::contains("\"CurrentPage\""));

    mock.assert();
}

#[test]
fn test_storage_zone_list_with_pagination() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/storagezone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "2".to_string()),
            mockito::Matcher::UrlEncoded("perPage".to_string(), "10".to_string()),
        ]))
        .with_body(common::fixture("storage_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "storage-zone",
        "list",
        "--page",
        "2",
        "--per-page",
        "10",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("zone-alpha"));

    mock.assert();
}

#[test]
fn test_storage_zone_get_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/storagezone/100")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("storage_zone_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "storage-zone", "get", "100"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("100"))
        .stdout(predicate::str::contains("zone-alpha"))
        .stdout(predicate::str::contains("DE"))
        .stdout(predicate::str::contains("Standard"))
        .stdout(predicate::str::contains("NY, LA"));

    mock.assert();
}

#[test]
fn test_storage_zone_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/storagezone")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Name":"new-zone","Region":"LA"}"#.to_string(),
        ))
        .with_body(common::fixture("storage_zone_create.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "storage-zone",
        "create",
        "new-zone",
        "--region",
        "LA",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("new-zone"))
    .stdout(predicate::str::contains("300"));

    mock.assert();
}

#[test]
fn test_storage_zone_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/storagezone/100")
        .match_header("AccessKey", "test-key")
        .with_status(204)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "storage-zone", "delete", "100"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Storage zone 100 deleted"));

    mock.assert();
}

#[test]
fn test_storage_zone_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/storagezone")
        .match_query(mockito::Matcher::Any)
        .with_status(401)
        .with_body(r#"{"Message":"Authentication failed"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad-key", "storage-zone", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}

// --- Task 15: Storage Zone Actions ---

#[test]
fn test_storage_zone_check_availability() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/storagezone/checkavailability")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Name":"my-new-zone"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "storage-zone",
        "check-availability",
        "my-new-zone",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("my-new-zone"))
    .stdout(predicate::str::contains("available"));

    mock.assert();
}

#[test]
fn test_storage_zone_reset_password() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/storagezone/100/resetPassword")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "storage-zone",
        "reset-password",
        "100",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains(
        "Password reset for storage zone 100",
    ));

    mock.assert();
}

#[test]
fn test_storage_zone_reset_read_only_password() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/storagezone/resetReadOnlyPassword")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::UrlEncoded(
            "id".to_string(),
            "100".to_string(),
        ))
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "storage-zone",
        "reset-read-only-password",
        "--id",
        "100",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains(
        "Read-only password reset for storage zone 100",
    ));

    mock.assert();
}

#[test]
fn test_storage_zone_statistics_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/storagezone/100/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("storage_zone_statistics.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "storage-zone", "statistics", "100"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Storage Zone ID"))
        .stdout(predicate::str::contains("100"))
        .stdout(predicate::str::contains("Storage Data Points"))
        .stdout(predicate::str::contains("3"));

    mock.assert();
}

#[test]
fn test_storage_zone_statistics_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/storagezone/100/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("storage_zone_statistics.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "storage-zone",
        "statistics",
        "100",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"StorageUsedChart\""))
    .stdout(predicate::str::contains("\"FileCountChart\""));

    mock.assert();
}

#[test]
fn test_storage_zone_statistics_with_date_range() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/storagezone/100/statistics")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("dateFrom".to_string(), "2024-06-01".to_string()),
            mockito::Matcher::UrlEncoded("dateTo".to_string(), "2024-06-30".to_string()),
        ]))
        .with_body(common::fixture("storage_zone_statistics.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "storage-zone",
        "statistics",
        "100",
        "--date-from",
        "2024-06-01",
        "--date-to",
        "2024-06-30",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Storage Zone ID"));

    mock.assert();
}
