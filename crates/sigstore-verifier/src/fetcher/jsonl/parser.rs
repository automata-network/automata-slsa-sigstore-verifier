use base64::prelude::*;
use chrono::DateTime;
use crate::fetcher::jsonl::types::{CertChain as JsonlCertChain, TrustedRoot};
use crate::types::certificate::{CertificateChain, FulcioInstance};
use crate::VerificationError;

/// Parse RFC3339 timestamp string to Unix timestamp in seconds.
fn parse_rfc3339_timestamp(s: &str) -> Result<i64, VerificationError> {
    let dt = DateTime::parse_from_rfc3339(s).map_err(|e| {
        VerificationError::InvalidBundleFormat(format!("Invalid RFC3339 timestamp: {}", e))
    })?;
    Ok(dt.timestamp())
}

/// Load and parse Sigstore TrustedRoot bundles from JSONL format.
/// Each line in the input should be a valid JSON object representing a TrustedRoot.
///
/// # Arguments
/// * `content` - JSONL content where each line is a separate trust bundle
///
/// # Returns
/// Vector of parsed TrustedRoot objects, one per line
pub fn load_trusted_root_from_jsonl(content: &str) -> Result<Vec<TrustedRoot>, VerificationError> {
    let mut roots = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let root: TrustedRoot = serde_json::from_str(line).map_err(|e| {
            VerificationError::InvalidBundleFormat(format!(
                "Failed to parse JSONL line {}: {}",
                line_num + 1,
                e
            ))
        })?;

        roots.push(root);
    }

    if roots.is_empty() {
        return Err(VerificationError::InvalidBundleFormat(
            "No trust bundles found in JSONL content".to_string(),
        ));
    }

    Ok(roots)
}

/// Select appropriate certificate authority from trust bundles based on instance and timestamp.
/// Validates that the certificate was valid at the time of signing.
/// When multiple CAs match, selects the one with the latest start date to ensure the most
/// recent/specific certificate is used.
///
/// # Arguments
/// * `roots` - Parsed trust root bundles
/// * `instance` - Fulcio instance (GitHub or PublicGood)
/// * `timestamp` - Signature timestamp in Unix seconds
///
/// # Returns
/// Certificate chain for the matching authority
pub fn select_certificate_authority(
    roots: &[TrustedRoot],
    instance: &FulcioInstance,
    timestamp: i64,
) -> Result<CertificateChain, VerificationError> {
    let expected_uri = instance.trust_bundle_url();
    let mut best_match: Option<(&JsonlCertChain, i64)> = None;

    for root in roots {
        for ca in &root.certificate_authorities {
            // Match by URI (primary method)
            if ca.uri.contains(expected_uri.trim_start_matches("https://").split('/').next().unwrap()) {
                // Validate timestamp falls within validity period
                if let Some(start_str) = &ca.valid_for.start {
                    let start = parse_rfc3339_timestamp(start_str)?;
                    if timestamp < start {
                        continue; // Not yet valid
                    }

                    // Check end time if present
                    if let Some(end_str) = &ca.valid_for.end {
                        let end = parse_rfc3339_timestamp(end_str)?;
                        if timestamp > end {
                            continue; // Expired
                        }
                    }
                    // No end time means ongoing/current certificate

                    // Keep track of the best match (most recent start date)
                    match best_match {
                        None => best_match = Some((&ca.cert_chain, start)),
                        Some((_, best_start)) if start > best_start => {
                            best_match = Some((&ca.cert_chain, start));
                        }
                        _ => {} // Keep existing best match
                    }
                }
            }
        }
    }

    match best_match {
        Some((cert_chain, _)) => extract_cert_chain_from_authority(cert_chain),
        None => Err(VerificationError::InvalidBundleFormat(format!(
            "No valid certificate authority found for instance {:?} at timestamp {}",
            instance, timestamp
        ))),
    }
}

/// Select appropriate timestamp authority from trust bundles based on instance and timestamp.
/// Validates that the TSA certificate was valid at the time of signing.
/// When multiple TSAs match, selects the one with the latest start date to ensure the most
/// recent/specific certificate is used.
///
/// # Arguments
/// * `roots` - Parsed trust root bundles
/// * `instance` - Fulcio instance (GitHub or PublicGood) - used to determine TSA endpoint
/// * `timestamp` - Signature timestamp in Unix seconds
///
/// # Returns
/// Certificate chain for the matching timestamp authority
pub fn select_timestamp_authority(
    roots: &[TrustedRoot],
    instance: &FulcioInstance,
    timestamp: i64,
) -> Result<CertificateChain, VerificationError> {
    // Map Fulcio instance to expected TSA URI
    let expected_tsa_domain = match instance {
        FulcioInstance::GitHub => "timestamp.githubapp.com",
        FulcioInstance::PublicGood => "timestamp.sigstore.dev",
    };

    let mut best_match: Option<(&JsonlCertChain, i64)> = None;

    for root in roots {
        for tsa in &root.timestamp_authorities {
            // Match by URI
            if tsa.uri.contains(expected_tsa_domain) {
                // Validate timestamp falls within validity period
                if let Some(start_str) = &tsa.valid_for.start {
                    let start = parse_rfc3339_timestamp(start_str)?;
                    if timestamp < start {
                        continue; // Not yet valid
                    }

                    // Check end time if present
                    if let Some(end_str) = &tsa.valid_for.end {
                        let end = parse_rfc3339_timestamp(end_str)?;
                        if timestamp > end {
                            continue; // Expired
                        }
                    }
                    // No end time means ongoing/current certificate

                    // Keep track of the best match (most recent start date)
                    match best_match {
                        None => best_match = Some((&tsa.cert_chain, start)),
                        Some((_, best_start)) if start > best_start => {
                            best_match = Some((&tsa.cert_chain, start));
                        }
                        _ => {} // Keep existing best match
                    }
                }
            }
        }
    }

    match best_match {
        Some((cert_chain, _)) => extract_tsa_cert_chain_from_authority(cert_chain),
        None => Err(VerificationError::InvalidBundleFormat(format!(
            "No valid timestamp authority found for instance {:?} at timestamp {}",
            instance, timestamp
        ))),
    }
}

/// Convert JSONL cert chain to verifier's CertificateChain format for Fulcio CAs.
/// Decodes base64-encoded DER certificates.
/// For Fulcio chains, the leaf certificate is in the bundle, not in the trust bundle.
///
/// # Arguments
/// * `cert_chain` - JSONL certificate chain with base64 rawBytes
///
/// # Returns
/// CertificateChain with leaf=empty, intermediates, and root
fn extract_cert_chain_from_authority(
    cert_chain: &JsonlCertChain,
) -> Result<CertificateChain, VerificationError> {
    if cert_chain.certificates.is_empty() {
        return Err(VerificationError::InvalidBundleFormat(
            "Certificate chain is empty".to_string(),
        ));
    }

    // Decode all certificates from base64 to DER
    let mut der_certs: Vec<Vec<u8>> = Vec::new();
    for cert in &cert_chain.certificates {
        let der = BASE64_STANDARD.decode(&cert.raw_bytes).map_err(|e| {
            VerificationError::InvalidBundleFormat(format!("Failed to decode certificate: {}", e))
        })?;
        der_certs.push(der);
    }

    // For Fulcio chains: leaf is in the bundle (not in trust bundle)
    // Trust bundle contains: [intermediate L2, intermediate L1, root]
    // We return: leaf=empty, intermediates=[0..n-1], root=last

    if der_certs.len() == 1 {
        // Single certificate - treat as root (self-signed)
        Ok(CertificateChain {
            leaf: Vec::new(),
            intermediates: Vec::new(),
            root: der_certs[0].clone(),
        })
    } else {
        // Multiple certificates: [intermediates...], root
        let root = der_certs.last().unwrap().clone();
        let intermediates = der_certs[..der_certs.len() - 1].to_vec();

        Ok(CertificateChain {
            leaf: Vec::new(),
            intermediates,
            root,
        })
    }
}

/// Convert JSONL cert chain to verifier's CertificateChain format for TSAs.
/// Decodes base64-encoded DER certificates.
/// For TSA chains, the leaf certificate (TSA signing cert) is in the trust bundle.
///
/// # Arguments
/// * `cert_chain` - JSONL certificate chain with base64 rawBytes
///
/// # Returns
/// CertificateChain with leaf, intermediates, and root
fn extract_tsa_cert_chain_from_authority(
    cert_chain: &JsonlCertChain,
) -> Result<CertificateChain, VerificationError> {
    if cert_chain.certificates.is_empty() {
        return Err(VerificationError::InvalidBundleFormat(
            "Certificate chain is empty".to_string(),
        ));
    }

    // Decode all certificates from base64 to DER
    let mut der_certs: Vec<Vec<u8>> = Vec::new();
    for cert in &cert_chain.certificates {
        let der = BASE64_STANDARD.decode(&cert.raw_bytes).map_err(|e| {
            VerificationError::InvalidBundleFormat(format!("Failed to decode certificate: {}", e))
        })?;
        der_certs.push(der);
    }

    // For TSA chains: [TSA signing cert (leaf), TSA intermediate, root]
    // We return: leaf=cert[0], intermediates=cert[1..n-1], root=cert[last]

    if der_certs.len() == 1 {
        // Single self-signed TSA cert
        Ok(CertificateChain {
            leaf: der_certs[0].clone(),
            intermediates: Vec::new(),
            root: der_certs[0].clone(),
        })
    } else if der_certs.len() == 2 {
        // TSA leaf + root, no intermediates
        Ok(CertificateChain {
            leaf: der_certs[0].clone(),
            intermediates: Vec::new(),
            root: der_certs[1].clone(),
        })
    } else {
        // TSA leaf + intermediates + root
        let leaf = der_certs[0].clone();
        let root = der_certs.last().unwrap().clone();
        let intermediates = der_certs[1..der_certs.len() - 1].to_vec();

        Ok(CertificateChain {
            leaf,
            intermediates,
            root,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_empty_jsonl() {
        let result = load_trusted_root_from_jsonl("");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_invalid_json() {
        let result = load_trusted_root_from_jsonl("not a json");
        assert!(result.is_err());
    }
}
