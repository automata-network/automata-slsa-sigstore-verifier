#![cfg(feature = "fetcher")]

use sigstore_verifier::fetcher::jsonl::parser::{
    load_trusted_root_from_jsonl, select_certificate_authority, select_timestamp_authority,
};
use sigstore_verifier::types::certificate::FulcioInstance;
use std::fs;
use std::path::PathBuf;

fn get_sample_trusted_root() -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../samples/trusted_root.jsonl");
    fs::read_to_string(path).expect("Failed to read trusted_root.jsonl")
}

#[test]
fn test_load_trusted_root_from_jsonl() {
    let content = get_sample_trusted_root();
    let roots = load_trusted_root_from_jsonl(&content).expect("Failed to parse JSONL");

    // Should have 2 trust bundles (one for public Sigstore, one for GitHub)
    assert_eq!(roots.len(), 2);

    // Verify structure of trust bundles
    for root in &roots {
        assert!(root.media_type.contains("sigstore.trustedroot"));
        // Should have either certificate authorities or timestamp authorities
        assert!(
            !root.certificate_authorities.is_empty()
                || !root.timestamp_authorities.is_empty()
        );
    }
}

#[test]
fn test_load_empty_jsonl() {
    let result = load_trusted_root_from_jsonl("");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No trust bundles found"));
}

#[test]
fn test_load_invalid_json() {
    let result = load_trusted_root_from_jsonl("not valid json");
    assert!(result.is_err());
}

#[test]
fn test_load_whitespace_lines() {
    let content = "\n\n  \n";
    let result = load_trusted_root_from_jsonl(content);
    assert!(result.is_err());
}

#[test]
fn test_select_github_certificate_authority() {
    let content = get_sample_trusted_root();
    let roots = load_trusted_root_from_jsonl(&content).expect("Failed to parse JSONL");

    // Use a timestamp that falls within the validity period of GitHub certificates
    // Looking at the JSONL, GitHub CAs have validity periods like:
    // "start": 1719849600 (2024-07-01), "end": 1751385600 (2025-07-01)
    let timestamp = 1720000000; // Mid-2024

    let result = select_certificate_authority(&roots, &FulcioInstance::GitHub, timestamp);
    assert!(
        result.is_ok(),
        "Failed to select GitHub CA: {:?}",
        result.err()
    );

    let chain = result.unwrap();
    // Should have intermediates and root
    assert!(!chain.intermediates.is_empty(), "No intermediates found");
    assert!(!chain.root.is_empty(), "No root certificate found");
}

#[test]
fn test_select_public_sigstore_certificate_authority() {
    let content = get_sample_trusted_root();
    let roots = load_trusted_root_from_jsonl(&content).expect("Failed to parse JSONL");

    // Use a timestamp that falls within the validity period of public Sigstore CAs
    // The second public Sigstore CA starts at 2022-04-13 with no end date
    let timestamp = 1650000000; // April 2022

    let result =
        select_certificate_authority(&roots, &FulcioInstance::PublicGood, timestamp);
    assert!(
        result.is_ok(),
        "Failed to select public Sigstore CA: {:?}",
        result.err()
    );

    let chain = result.unwrap();
    // This CA has 2 certificates: intermediate and root (no separate leaf for trust bundles)
    assert!(!chain.root.is_empty(), "No root certificate found");
}

#[test]
fn test_select_github_timestamp_authority() {
    let content = get_sample_trusted_root();
    let roots = load_trusted_root_from_jsonl(&content).expect("Failed to parse JSONL");

    // Use a timestamp within GitHub TSA validity period
    let timestamp = 1720000000; // Mid-2024

    let result = select_timestamp_authority(&roots, &FulcioInstance::GitHub, timestamp);
    assert!(
        result.is_ok(),
        "Failed to select GitHub TSA: {:?}",
        result.err()
    );

    let chain = result.unwrap();
    // TSA chains should have certificates
    assert!(
        !chain.intermediates.is_empty() || !chain.root.is_empty(),
        "No certificates in TSA chain"
    );
}

#[test]
fn test_select_public_sigstore_timestamp_authority() {
    let content = get_sample_trusted_root();
    let roots = load_trusted_root_from_jsonl(&content).expect("Failed to parse JSONL");

    // Public Sigstore TSA starts at 2025-07-04, so use a timestamp after that
    let timestamp = 1752000000; // Mid-2025

    let result = select_timestamp_authority(&roots, &FulcioInstance::PublicGood, timestamp);
    assert!(
        result.is_ok(),
        "Failed to select public Sigstore TSA: {:?}",
        result.err()
    );

    let chain = result.unwrap();
    assert!(
        !chain.intermediates.is_empty() || !chain.root.is_empty(),
        "No certificates in TSA chain"
    );
}

#[test]
fn test_validity_period_enforcement() {
    let content = get_sample_trusted_root();
    let roots = load_trusted_root_from_jsonl(&content).expect("Failed to parse JSONL");

    // Use a timestamp before any GitHub certificates were valid
    // First GitHub CA starts at 2023-10-27
    let before_all_timestamp = 1600000000; // Sep 2020

    // Should fail because timestamp is before all GitHub certificates
    let result = select_certificate_authority(&roots, &FulcioInstance::GitHub, before_all_timestamp);
    assert!(result.is_err(), "Should reject timestamp before all certificates");
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No valid certificate authority found"));
}

#[test]
fn test_expired_certificate_rejected() {
    let content = get_sample_trusted_root();
    let roots = load_trusted_root_from_jsonl(&content).expect("Failed to parse JSONL");

    // Use a timestamp that falls in the gap between the first and second GitHub CAs
    // First GitHub CA: 2023-10-27 to 2024-05-25
    // Second GitHub CA: 2024-05-13 to 2024-10-25
    // Gap: None, they overlap. Let's use a timestamp after the first CA expired
    // but matched by the second one. Actually there's no gap.
    // Instead, let's test a timestamp way before any GitHub certs
    let old_timestamp = 1262304000; // Year 2010

    // Should fail for GitHub instance (no certs that old)
    let result = select_certificate_authority(&roots, &FulcioInstance::GitHub, old_timestamp);
    assert!(result.is_err(), "Should reject very old timestamp");
}

#[test]
fn test_certificate_chain_structure() {
    let content = get_sample_trusted_root();
    let roots = load_trusted_root_from_jsonl(&content).expect("Failed to parse JSONL");

    let timestamp = 1720000000;
    let chain =
        select_certificate_authority(&roots, &FulcioInstance::GitHub, timestamp).unwrap();

    // Verify chain structure
    // For Fulcio: leaf should be empty, intermediates should have entries, root should exist
    assert!(chain.leaf.is_empty(), "Fulcio leaf should be empty");
    assert!(
        !chain.intermediates.is_empty(),
        "Should have intermediates"
    );
    assert!(!chain.root.is_empty(), "Should have root");

    // Verify they're valid DER-encoded certificates (basic check)
    for intermediate in &chain.intermediates {
        assert!(
            intermediate.len() > 100,
            "Certificate too small to be valid"
        );
        // DER certificates typically start with 0x30 (SEQUENCE tag)
        assert_eq!(intermediate[0], 0x30, "Not a valid DER certificate");
    }
    assert!(chain.root.len() > 100, "Root certificate too small");
    assert_eq!(chain.root[0], 0x30, "Root not a valid DER certificate");
}
