use x509_parser::prelude::*;

use crate::crypto::{sha256, PublicKey};
use crate::error::CertificateError;
use crate::parser::{decode_base64, parse_der_certificate};
use crate::types::{CertificateChain, CertificateChainHashes, SigstoreBundle};

/// Verify the certificate chain using provided trust bundle
///
/// # Arguments
///
/// * `bundle` - The Sigstore bundle containing the leaf certificate
/// * `trust_bundle` - The trust bundle (intermediates and root) for verification
///
/// # Returns
///
/// Returns the complete certificate chain and SHA256 hashes of all certificates
pub fn verify_certificate_chain(
    bundle: &SigstoreBundle,
    trust_bundle: &CertificateChain,
) -> Result<(CertificateChain, CertificateChainHashes), CertificateError> {
    // Parse leaf certificate from bundle
    let leaf_der = decode_base64(&bundle.verification_material.certificate.raw_bytes)
        .map_err(|e| CertificateError::ParseError(e.to_string()))?;

    // Create complete chain with leaf from bundle
    let chain = CertificateChain {
        leaf: leaf_der.clone(),
        intermediates: trust_bundle.intermediates.clone(),
        root: trust_bundle.root.clone(),
    };

    // Parse all certificates
    let leaf_x509 = parse_der_certificate(&chain.leaf)?;
    let mut intermediate_x509 = Vec::new();
    for der in &chain.intermediates {
        intermediate_x509.push(parse_der_certificate(der)?);
    }
    let root_x509 = parse_der_certificate(&chain.root)?;

    // Verify certificate signatures
    // 1. Verify leaf signed by first intermediate
    verify_cert_signature(&leaf_x509, &intermediate_x509[0])?;

    // 2. Verify intermediate chain
    for i in 0..intermediate_x509.len() - 1 {
        verify_cert_signature(&intermediate_x509[i], &intermediate_x509[i + 1])?;
    }

    // 3. Verify last intermediate signed by root
    if let Some(last_intermediate) = intermediate_x509.last() {
        verify_cert_signature(last_intermediate, &root_x509)?;
    }

    // 4. Verify root is self-signed
    verify_cert_signature(&root_x509, &root_x509)?;

    // Compute SHA256 hashes of all certificates
    let leaf_hash = sha256(&chain.leaf);
    let intermediate_hashes: Vec<[u8; 32]> = chain
        .intermediates
        .iter()
        .map(|der| sha256(der))
        .collect();
    let root_hash = sha256(&chain.root);

    let hashes = CertificateChainHashes {
        leaf: leaf_hash,
        intermediates: intermediate_hashes,
        root: root_hash,
    };

    Ok((chain, hashes))
}

fn verify_cert_signature(
    cert: &X509Certificate,
    issuer: &X509Certificate,
) -> Result<(), CertificateError> {
    let public_key = PublicKey::from_certificate(issuer)
        .map_err(|e| CertificateError::ChainVerificationFailed(e.to_string()))?;

    let signature = &cert.signature_value.data;
    let tbs_certificate = cert.tbs_certificate.as_ref();

    public_key
        .verify_signature(tbs_certificate, signature)
        .map_err(|e| CertificateError::ChainVerificationFailed(e.to_string()))?;

    Ok(())
}
