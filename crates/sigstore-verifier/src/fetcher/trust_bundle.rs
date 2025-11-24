use crate::error::CertificateError;
use crate::parser::certificate::parse_pem_certificate;
use crate::types::certificate::{CertificateChain, FulcioInstance, TrustBundle};

/// Fetch Fulcio trust bundle for a specific Fulcio instance
///
/// # Arguments
/// * `instance` - The Fulcio instance (GitHub or PublicGood)
///
/// # Returns
/// * `CertificateChain` with intermediates and root populated (leaf is empty)
pub fn fetch_fulcio_trust_bundle(
    instance: &FulcioInstance,
) -> Result<CertificateChain, CertificateError> {
    fetch_trust_bundle_from_url(instance.trust_bundle_url())
}

/// Fetch certificate trust bundle from a custom URL
///
/// This is a generic function that can fetch certificate chains from any URL
/// that serves trust bundles. It handles two formats:
/// 1. JSON format: `{"chains": [{"certificates": ["PEM1", "PEM2", ...]}]}`
/// 2. Raw PEM format: Concatenated PEM certificates
///
/// Useful for fetching TSA certificate chains or custom certificate authorities.
///
/// # Arguments
/// * `url` - URL to fetch the trust bundle from
///
/// # Returns
/// * `CertificateChain` with intermediates and root populated (leaf is empty)
///
/// # Example
/// ```ignore
/// use sigstore_verifier::fetcher::trust_bundle::fetch_trust_bundle_from_url;
///
/// // Fetch TSA trust bundle (GitHub format - raw PEM)
/// let tsa_url = "https://timestamp.githubapp.com/api/v1/timestamp/certchain";
/// let tsa_chain = fetch_trust_bundle_from_url(tsa_url).unwrap();
/// ```
pub fn fetch_trust_bundle_from_url(url: &str) -> Result<CertificateChain, CertificateError> {
    let response = reqwest::blocking::get(url)
        .map_err(|e| CertificateError::TrustBundleFetch(e.to_string()))?;

    if !response.status().is_success() {
        return Err(CertificateError::TrustBundleFetch(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    // Get response body as text to detect format
    let body = response
        .text()
        .map_err(|e| CertificateError::TrustBundleFetch(e.to_string()))?;

    // Try to detect format: if it starts with "-----BEGIN", it's PEM format
    if body.trim().starts_with("-----BEGIN") {
        // Parse as concatenated PEM certificates
        parse_pem_chain(&body)
    } else {
        // Parse as JSON format
        let bundle: TrustBundle = serde_json::from_str(&body)
            .map_err(|e| CertificateError::TrustBundleFetch(e.to_string()))?;

        if bundle.chains.is_empty() {
            return Err(CertificateError::TrustBundleFetch(
                "No certificate chains in trust bundle".to_string(),
            ));
        }

        let chain = &bundle.chains[0];
        if chain.certificates.is_empty() {
            return Err(CertificateError::TrustBundleFetch(
                "Empty certificate chain".to_string(),
            ));
        }

        // Parse all certificates from PEM to DER
        let mut der_certs = Vec::new();
        for pem_cert in &chain.certificates {
            let der = parse_pem_certificate(pem_cert)?;
            der_certs.push(der);
        }

        if der_certs.len() < 2 {
            return Err(CertificateError::TrustBundleFetch(
                "Certificate chain too short".to_string(),
            ));
        }

        let root = der_certs.pop().unwrap();
        let intermediates = der_certs;

        Ok(CertificateChain {
            leaf: Vec::new(),
            intermediates,
            root,
        })
    }
}

/// Parse concatenated PEM certificates into a CertificateChain
///
/// Handles raw PEM format where multiple certificates are concatenated.
/// Expected order: leaf, intermediates..., root
fn parse_pem_chain(pem_data: &str) -> Result<CertificateChain, CertificateError> {
    let mut der_certs = Vec::new();

    // Parse all PEM blocks from the data
    let pem_blocks = pem::parse_many(pem_data.as_bytes())
        .map_err(|e| CertificateError::TrustBundleFetch(format!("Failed to parse PEM: {}", e)))?;

    for block in pem_blocks {
        if block.tag() != "CERTIFICATE" {
            continue; // Skip non-certificate blocks
        }
        der_certs.push(block.into_contents());
    }

    if der_certs.is_empty() {
        return Err(CertificateError::TrustBundleFetch(
            "No certificates found in PEM data".to_string(),
        ));
    }

    if der_certs.len() < 2 {
        return Err(CertificateError::TrustBundleFetch(
            "Certificate chain too short".to_string(),
        ));
    }

    // Structure: [leaf, intermediate(s), root]
    let root = der_certs.pop().unwrap();
    let leaf = der_certs.remove(0);
    let intermediates = der_certs; // Remaining middle certs

    Ok(CertificateChain {
        leaf,
        intermediates,
        root,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires network access
    fn test_fetch_github_trust_bundle() {
        let result = fetch_fulcio_trust_bundle(&FulcioInstance::GitHub);
        assert!(result.is_ok());

        let chain = result.unwrap();
        assert!(!chain.intermediates.is_empty());
        assert!(!chain.root.is_empty());
    }

    #[test]
    #[ignore] // Requires network access
    fn test_fetch_public_trust_bundle() {
        let result = fetch_fulcio_trust_bundle(&FulcioInstance::PublicGood);
        assert!(result.is_ok());

        let chain = result.unwrap();
        assert!(!chain.intermediates.is_empty());
        assert!(!chain.root.is_empty());
    }
}
