mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_api_key_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/apikey")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("api_key_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "api-key", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("key-abc-123"))
        .stdout(predicate::str::contains("key-def-456"))
        .stdout(predicate::str::contains("Page 1"));

    mock.assert();
}

#[test]
fn test_api_key_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/apikey")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("api_key_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "api-key", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Items\""))
        .stdout(predicate::str::contains("\"CurrentPage\""));

    mock.assert();
}

#[test]
fn test_api_key_list_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/apikey")
        .with_status(401)
        .with_body(r#"{"Message":"Authentication failed"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad-key", "api-key", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}
