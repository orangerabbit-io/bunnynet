mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_billing_get_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/billing")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("billing.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "billing", "get"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Balance"))
        .stdout(predicate::str::contains("$42.50"))
        .stdout(predicate::str::contains("This Month Charges"))
        .stdout(predicate::str::contains("$12.75"))
        .stdout(predicate::str::contains("Bandwidth Used"))
        .stdout(predicate::str::contains("9876543"));

    mock.assert();
}

#[test]
fn test_billing_get_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/billing")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("billing.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "billing", "get"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Balance\""))
        .stdout(predicate::str::contains("\"BillingHistoryChart\""));

    mock.assert();
}

#[test]
fn test_billing_summary_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/billing/summary")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("billing_summary.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "billing", "summary"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("123"))
        .stdout(predicate::str::contains("456"))
        .stdout(predicate::str::contains("5.1234"))
        .stdout(predicate::str::contains("1000000"));

    mock.assert();
}

#[test]
fn test_billing_affiliate_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/billing/affiliate")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("billing_affiliate.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "billing", "affiliate"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Affiliate Balance"))
        .stdout(predicate::str::contains("$100.00"))
        .stdout(predicate::str::contains("Affiliate URL"))
        .stdout(predicate::str::contains("bunny.net"));

    mock.assert();
}

#[test]
fn test_billing_payment_requests_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/billing/payment-requests")
        .match_header("AccessKey", "test-key")
        .with_body(common::fixture("billing_payment_requests.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "billing", "payment-requests"])
        .env("BUNNYNET_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("456"))
        .stdout(predicate::str::contains("99.99"))
        .stdout(predicate::str::contains("Monthly invoice"))
        .stdout(predicate::str::contains("false"));

    mock.assert();
}

#[test]
fn test_billing_download_invoice() {
    let mut server = Server::new();
    let pdf_bytes = b"%PDF-1.4 dummy invoice content";
    let mock = server
        .mock("GET", "/billing/payment-request-invoice/789/pdf")
        .match_header("AccessKey", "test-key")
        .with_body(pdf_bytes.as_slice())
        .with_header("content-type", "application/pdf")
        .create();

    let tmp_dir = std::env::temp_dir();
    let output_path = tmp_dir.join("test_invoice_789.pdf");
    // Clean up from any previous run
    let _ = std::fs::remove_file(&output_path);

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "billing",
        "download-invoice",
        "789",
        "--output",
        output_path.to_str().unwrap(),
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Invoice saved to"));

    mock.assert();

    // Verify the file was written with the correct content
    let written = std::fs::read(&output_path).unwrap();
    assert_eq!(written, pdf_bytes);

    // Clean up
    let _ = std::fs::remove_file(&output_path);
}

#[test]
fn test_billing_download_summary() {
    let mut server = Server::new();
    let pdf_bytes = b"%PDF-1.4 dummy summary content";
    let mock = server
        .mock("GET", "/billing/summary/101/pdf")
        .match_header("AccessKey", "test-key")
        .with_body(pdf_bytes.as_slice())
        .with_header("content-type", "application/pdf")
        .create();

    let tmp_dir = std::env::temp_dir();
    let output_path = tmp_dir.join("test_summary_101.pdf");
    let _ = std::fs::remove_file(&output_path);

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "billing",
        "download-summary",
        "101",
        "--output",
        output_path.to_str().unwrap(),
    ])
    .env("BUNNYNET_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Summary saved to"));

    mock.assert();

    let written = std::fs::read(&output_path).unwrap();
    assert_eq!(written, pdf_bytes);

    let _ = std::fs::remove_file(&output_path);
}
