mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_api_key_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/apikey")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()),
            mockito::Matcher::UrlEncoded("perPage".to_string(), "1000".to_string()),
        ]))
        .with_body(common::fixture("api_key_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "api-key", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("key-abc-123"))
        .stdout(predicate::str::contains("key-def-456"));

    mock.assert();
}

#[test]
fn test_api_key_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/apikey")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()),
            mockito::Matcher::UrlEncoded("perPage".to_string(), "1000".to_string()),
        ]))
        .with_body(common::fixture("api_key_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "api-key", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("key-abc-123"))
        .stdout(predicate::str::contains("key-def-456"));

    mock.assert();
}

#[test]
fn test_api_key_list_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/apikey")
        .match_query(mockito::Matcher::Any)
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
