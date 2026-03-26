mod common;

use mockito::Server;
use predicates::prelude::*;

// --- Task 16: DNS Zone CRUD ---

#[test]
fn test_dns_zone_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()),
        ]))
        .with_body(common::fixture("dns_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "dns-zone", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("example.com"))
        .stdout(predicate::str::contains("test.org"))
        .stdout(predicate::str::contains("ns1.bunny.net"))
        .stdout(predicate::str::contains("Page 1"));

    mock.assert();
}

#[test]
fn test_dns_zone_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()),
        ]))
        .with_body(common::fixture("dns_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "dns-zone", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Items\""))
        .stdout(predicate::str::contains("\"CurrentPage\""));

    mock.assert();
}

#[test]
fn test_dns_zone_list_with_pagination() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "2".to_string()),
            mockito::Matcher::UrlEncoded("perPage".to_string(), "10".to_string()),
        ]))
        .with_body(common::fixture("dns_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "list",
        "--page",
        "2",
        "--per-page",
        "10",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("example.com"));

    mock.assert();
}

#[test]
fn test_dns_zone_list_with_search() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()),
            mockito::Matcher::UrlEncoded("search".to_string(), "example".to_string()),
        ]))
        .with_body(common::fixture("dns_zone_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "list",
        "--search",
        "example",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("example.com"));

    mock.assert();
}

#[test]
fn test_dns_zone_get_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone/500")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("dns_zone_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "dns-zone", "get", "500"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("500"))
        .stdout(predicate::str::contains("example.com"))
        .stdout(predicate::str::contains("ns1.bunny.net"))
        .stdout(predicate::str::contains("ns2.bunny.net"))
        .stdout(predicate::str::contains("admin@example.com"))
        .stdout(predicate::str::contains("Enabled"));

    mock.assert();
}

#[test]
fn test_dns_zone_get_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone/500")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("dns_zone_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "dns-zone", "get", "500"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Domain\""))
        .stdout(predicate::str::contains("\"Records\""));

    mock.assert();
}

#[test]
fn test_dns_zone_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dnszone")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Domain":"newzone.com"}"#.to_string(),
        ))
        .with_body(common::fixture("dns_zone_create.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "create",
        "newzone.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("newzone.com"))
    .stdout(predicate::str::contains("600"));

    mock.assert();
}

#[test]
fn test_dns_zone_update() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dnszone/500")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"SoaEmail":"new@example.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "update",
        "500",
        "--soa-email",
        "new@example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("DNS zone 500 updated"));

    mock.assert();
}

#[test]
fn test_dns_zone_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/dnszone/500")
        .match_header("AccessKey", "test-key")
        .with_status(204)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "dns-zone", "delete", "500"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("DNS zone 500 deleted"));

    mock.assert();
}

#[test]
fn test_dns_zone_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone")
        .match_query(mockito::Matcher::Any)
        .with_status(401)
        .with_body(r#"{"Message":"Authentication failed"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad-key", "dns-zone", "list"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}

// --- Task 17: DNS Records ---

#[test]
fn test_dns_record_add() {
    let mut server = Server::new();
    let mock = server
        .mock("PUT", "/dnszone/500/records")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Type":0,"Name":"www","Value":"1.2.3.4"}"#.to_string(),
        ))
        .with_body(common::fixture("dns_record_add.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "record",
        "add",
        "500",
        "--type",
        "A",
        "--name",
        "www",
        "--value",
        "1.2.3.4",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("DNS record added"))
    .stdout(predicate::str::contains("2001"));

    mock.assert();
}

#[test]
fn test_dns_record_add_with_options() {
    let mut server = Server::new();
    let mock = server
        .mock("PUT", "/dnszone/500/records")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Type":4,"Name":"mail","Value":"mail.example.com","Priority":10,"Ttl":3600}"#
                .to_string(),
        ))
        .with_body(common::fixture("dns_record_add.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "record",
        "add",
        "500",
        "--type",
        "MX",
        "--name",
        "mail",
        "--value",
        "mail.example.com",
        "--priority",
        "10",
        "--ttl",
        "3600",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("DNS record added"));

    mock.assert();
}

#[test]
fn test_dns_record_add_json() {
    let mut server = Server::new();
    let mock = server
        .mock("PUT", "/dnszone/500/records")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("dns_record_add.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "dns-zone",
        "record",
        "add",
        "500",
        "--type",
        "A",
        "--name",
        "www",
        "--value",
        "1.2.3.4",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"Id\""))
    .stdout(predicate::str::contains("2001"));

    mock.assert();
}

#[test]
fn test_dns_record_update() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dnszone/500/records/2001")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Value":"5.6.7.8"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "record",
        "update",
        "500",
        "2001",
        "--value",
        "5.6.7.8",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("DNS record 2001 updated in zone 500"));

    mock.assert();
}

#[test]
fn test_dns_record_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/dnszone/500/records/2001")
        .match_header("AccessKey", "test-key")
        .with_status(204)
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "record",
        "delete",
        "500",
        "2001",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("DNS record 2001 deleted from zone 500"));

    mock.assert();
}

// --- Task 18: DNS Zone Actions ---

#[test]
fn test_dns_zone_export() {
    let zone_file = "$ORIGIN example.com.\n@ 300 IN A 1.2.3.4\nwww 300 IN CNAME example.com.\n";
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone/500/export")
        .match_header("AccessKey", "test-key")
        .with_body(zone_file)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "dns-zone", "export", "500"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("$ORIGIN example.com"))
        .stdout(predicate::str::contains("IN A 1.2.3.4"));

    mock.assert();
}

#[test]
fn test_dns_zone_export_json() {
    let zone_file = "$ORIGIN example.com.\n@ 300 IN A 1.2.3.4\n";
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone/500/export")
        .match_header("AccessKey", "test-key")
        .with_body(zone_file)
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "dns-zone",
        "export",
        "500",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("zone_file"));

    mock.assert();
}

#[test]
fn test_dns_zone_import() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dnszone/500/import")
        .match_header("AccessKey", "test-key")
        .match_header("Content-Type", "text/plain")
        .with_body(common::fixture("dns_zone_import_result.json"))
        .with_header("content-type", "application/json")
        .create();

    // Create a temp zone file
    let tmp_dir = std::env::temp_dir();
    let zone_file = tmp_dir.join("test_zone_import.txt");
    std::fs::write(&zone_file, "@ 300 IN A 1.2.3.4\nwww 300 IN CNAME @\n").unwrap();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "import",
        "500",
        "--file",
        zone_file.to_str().unwrap(),
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Records Successful"))
    .stdout(predicate::str::contains("10"))
    .stdout(predicate::str::contains("Records Failed"))
    .stdout(predicate::str::contains("2"));

    // Clean up
    let _ = std::fs::remove_file(&zone_file);

    mock.assert();
}

#[test]
fn test_dns_zone_statistics_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone/500/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("dns_zone_statistics.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "statistics",
        "500",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("DNS Zone ID"))
    .stdout(predicate::str::contains("500"))
    .stdout(predicate::str::contains("Total Queries Served"))
    .stdout(predicate::str::contains("12345"));

    mock.assert();
}

#[test]
fn test_dns_zone_statistics_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone/500/statistics")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("dns_zone_statistics.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "dns-zone",
        "statistics",
        "500",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"TotalQueriesServed\""))
    .stdout(predicate::str::contains("\"QueriesServedChart\""));

    mock.assert();
}

#[test]
fn test_dns_zone_statistics_with_date_range() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone/500/statistics")
        .match_header("AccessKey", "test-key")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("dateFrom".to_string(), "2024-06-01".to_string()),
            mockito::Matcher::UrlEncoded("dateTo".to_string(), "2024-06-30".to_string()),
        ]))
        .with_body(common::fixture("dns_zone_statistics.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "statistics",
        "500",
        "--date-from",
        "2024-06-01",
        "--date-to",
        "2024-06-30",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("DNS Zone ID"));

    mock.assert();
}

#[test]
fn test_dns_zone_check_availability() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dnszone/checkavailability")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Name":"newdomain.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "check-availability",
        "newdomain.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("newdomain.com"))
    .stdout(predicate::str::contains("available"));

    mock.assert();
}

#[test]
fn test_dns_zone_dnssec_enable() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dnszone/500/dnssec")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "dnssec",
        "enable",
        "500",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("DNSSEC enabled for DNS zone 500"));

    mock.assert();
}

#[test]
fn test_dns_zone_dnssec_disable() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/dnszone/500/dnssec")
        .match_header("AccessKey", "test-key")
        .with_status(204)
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "dnssec",
        "disable",
        "500",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("DNSSEC disabled for DNS zone 500"));

    mock.assert();
}

#[test]
fn test_dns_zone_certificate_issue() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dnszone/500/certificate/issue")
        .match_header("AccessKey", "test-key")
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "certificate",
        "issue",
        "500",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Certificate issue triggered for zone 500"));

    mock.assert();
}

#[test]
fn test_dns_zone_certificate_issue_with_domain() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dnszone/500/certificate/issue")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Domain":"sub.example.com"}"#.to_string(),
        ))
        .with_status(200)
        .with_body("")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "certificate",
        "issue",
        "500",
        "--domain",
        "sub.example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Certificate issue triggered for zone 500"));

    mock.assert();
}

#[test]
fn test_dns_record_scan_by_zone_id() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dnszone/records/scan")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"ZoneId":500}"#.to_string(),
        ))
        .with_body(common::fixture("dns_record_scan_trigger.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "record",
        "scan",
        "--zone-id",
        "500",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Record scan triggered"))
    .stdout(predicate::str::contains("abc-123-def"));

    mock.assert();
}

#[test]
fn test_dns_record_scan_by_domain() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dnszone/records/scan")
        .match_header("AccessKey", "test-key")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"Domain":"example.com"}"#.to_string(),
        ))
        .with_body(common::fixture("dns_record_scan_trigger.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "record",
        "scan",
        "--domain",
        "example.com",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Record scan triggered"));

    mock.assert();
}

#[test]
fn test_dns_record_scan_results() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone/500/records/scan")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("dns_record_scan_results.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "dns-zone",
        "record",
        "scan-results",
        "500",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("abc-123-def"))
    .stdout(predicate::str::contains("example.com"))
    .stdout(predicate::str::contains("Completed"))
    .stdout(predicate::str::contains("2"));

    mock.assert();
}

#[test]
fn test_dns_record_scan_results_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/dnszone/500/records/scan")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("dns_record_scan_results.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "dns-zone",
        "record",
        "scan-results",
        "500",
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"JobId\""))
    .stdout(predicate::str::contains("\"Records\""));

    mock.assert();
}
