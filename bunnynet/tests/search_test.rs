mod common;

use mockito::{Matcher, Server};
use predicates::prelude::*;

#[test]
fn test_search_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/search")
        .match_header("AccessKey", "test-key")
        .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
            "search".into(),
            "example".into(),
        )]))
        .with_body(common::fixture("search_results.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "search", "example"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("my-zone"))
        .stdout(predicate::str::contains("example.com"));

    mock.assert();
}

#[test]
fn test_search_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/search")
        .match_header("AccessKey", "test-key")
        .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
            "search".into(),
            "example".into(),
        )]))
        .with_body(common::fixture("search_results.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "search", "example"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Query\""))
        .stdout(predicate::str::contains("\"SearchResults\""));

    mock.assert();
}

#[test]
fn test_search_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/search")
        .match_query(Matcher::Any)
        .with_status(401)
        .with_body(r#"{"Message":"Authentication failed"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad-key", "search", "example"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}
