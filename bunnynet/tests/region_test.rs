mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_region_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/region")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("region_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "region", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Europe (London)"))
        .stdout(predicate::str::contains("US (New York)"));

    mock.assert();
}

#[test]
fn test_region_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/region")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("region_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "region", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Id\""));

    mock.assert();
}

#[test]
fn test_region_list_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/region")
        .with_status(401)
        .with_body(r#"{"Message":"Authentication failed"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad-key", "region", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}
