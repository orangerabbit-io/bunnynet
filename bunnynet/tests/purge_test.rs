mod common;

use mockito::{Matcher, Server};
use predicates::prelude::*;

#[test]
fn test_purge_url_table() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/purge")
        .match_header("AccessKey", "test-key")
        .match_query(Matcher::Any)
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "purge",
        "url",
        "https://example.com/path",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains(
        "Purge queued for https://example.com/path",
    ));

    mock.assert();
}

#[test]
fn test_purge_url_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/purge")
        .match_header("AccessKey", "test-key")
        .match_query(Matcher::Any)
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "purge",
        "url",
        "https://example.com/path",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\""))
    .stdout(predicate::str::contains("\"purged\""));

    mock.assert();
}

#[test]
fn test_purge_url_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/purge")
        .match_query(Matcher::Any)
        .with_status(401)
        .with_body(r#"{"Message":"Authentication failed"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "bad-key",
        "purge",
        "url",
        "https://example.com/path",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .failure()
    .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}
