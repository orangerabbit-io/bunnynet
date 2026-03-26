mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_country_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/country")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("country_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "country", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Germany"))
        .stdout(predicate::str::contains("United States"));

    mock.assert();
}

#[test]
fn test_country_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/country")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("country_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "country", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"IsoCode\""));

    mock.assert();
}

#[test]
fn test_country_list_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/country")
        .with_status(401)
        .with_body(r#"{"Message":"Authentication failed"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad-key", "country", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}
