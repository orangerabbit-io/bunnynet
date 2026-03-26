mod common;

use mockito::{Matcher, Server};
use predicates::prelude::*;

#[test]
fn test_statistics_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/statistics")
        .match_header("AccessKey", "test-key")
        .match_query(Matcher::Any)
        .with_body(common::fixture("statistics.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "statistics"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total Bandwidth"))
        .stdout(predicate::str::contains("1234567890"))
        .stdout(predicate::str::contains("Total Requests"))
        .stdout(predicate::str::contains("5000000"))
        .stdout(predicate::str::contains("Cache Hit Rate"))
        .stdout(predicate::str::contains("95.50%"))
        .stdout(predicate::str::contains("Avg Origin Response Time"))
        .stdout(predicate::str::contains("250ms"));

    mock.assert();
}

#[test]
fn test_statistics_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/statistics")
        .match_header("AccessKey", "test-key")
        .match_query(Matcher::Any)
        .with_body(common::fixture("statistics.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "statistics"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"TotalBandwidthUsed\""))
        .stdout(predicate::str::contains("\"BandwidthUsedChart\""))
        .stdout(predicate::str::contains("\"GeoTrafficDistribution\""));

    mock.assert();
}

#[test]
fn test_statistics_with_flags() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/statistics")
        .match_header("AccessKey", "test-key")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("pullZone".into(), "123".into()),
            Matcher::UrlEncoded("hourly".into(), "true".into()),
            Matcher::UrlEncoded("loadBandwidthUsed".into(), "true".into()),
        ]))
        .with_body(common::fixture("statistics.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "statistics",
        "--pull-zone",
        "123",
        "--hourly",
        "--load-bandwidth-used",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success();

    mock.assert();
}
