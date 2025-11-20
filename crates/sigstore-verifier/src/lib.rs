pub mod crypto;
pub mod error;
pub mod fetcher;
pub mod parser;
pub mod types;
pub mod verifier;

use std::path::Path;

use error::VerificationError;
use parser::{parse_bundle_from_bytes, parse_bundle_from_path, parse_dsse_payload};
use types::{VerificationOptions, VerificationResult};
use verifier::{
    get_signing_time, verify_certificate_chain, verify_dsse_signature,
    verify_signing_time_in_validity, verify_subject_digest, verify_transparency_log,
};

/// Main attestation verifier
#[derive(Debug, Clone, Default)]
pub struct AttestationVerifier {}

impl AttestationVerifier {
    /// Create a new verifier instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Verify a sigstore bundle from a file path
    ///
    /// # Arguments
    ///
    /// * `bundle_path` - Path to the sigstore bundle JSON file
    /// * `options` - Verification options
    ///
    /// # Returns
    ///
    /// On success, returns `VerificationResult` containing:
    /// - Certificate chain hashes (leaf, intermediates, root)
    /// - Signing time
    /// - Subject digest
    /// - OIDC identity (if present)
    pub fn verify_bundle(
        &self,
        bundle_path: &Path,
        options: VerificationOptions,
    ) -> Result<VerificationResult, VerificationError> {
        let bundle = parse_bundle_from_path(bundle_path)?;
        self.verify_bundle_internal(&bundle, options)
    }

    /// Verify a sigstore bundle from raw JSON bytes
    ///
    /// # Arguments
    ///
    /// * `bundle_json` - Raw JSON bytes of the sigstore bundle
    /// * `options` - Verification options
    ///
    /// # Returns
    ///
    /// On success, returns `VerificationResult` containing:
    /// - Certificate chain hashes (leaf, intermediates, root)
    /// - Signing time
    /// - Subject digest
    /// - OIDC identity (if present)
    pub fn verify_bundle_bytes(
        &self,
        bundle_json: &[u8],
        options: VerificationOptions,
    ) -> Result<VerificationResult, VerificationError> {
        let bundle = parse_bundle_from_bytes(bundle_json)?;
        self.verify_bundle_internal(&bundle, options)
    }

    fn verify_bundle_internal(
        &self,
        bundle: &types::SigstoreBundle,
        options: VerificationOptions,
    ) -> Result<VerificationResult, VerificationError> {
        // Step 1: Parse and verify subject digest
        let statement = parse_dsse_payload(&bundle.dsse_envelope)?;
        let subject_digest = verify_subject_digest(&statement, options.expected_digest.as_deref())?;

        // Step 2: Get signing time (from RFC3161 timestamp or integrated time)
        let signing_time = get_signing_time(bundle)?;

        // Step 3: Verify certificate chain and get hashes
        let (chain, certificate_hashes) = verify_certificate_chain(bundle)?;

        // Step 3b: Verify signing time is within certificate validity period
        let leaf_cert = parser::parse_der_certificate(&chain.leaf)
            .map_err(|e| VerificationError::InvalidBundleFormat(e.to_string()))?;
        verify_signing_time_in_validity(&signing_time, &leaf_cert)?;

        // Step 4: Verify DSSE signature
        verify_dsse_signature(&bundle.dsse_envelope, &chain)?;

        // Step 5: Verify transparency log (if enabled)
        verify_transparency_log(bundle, !options.verify_rekor)?;

        // TODO: Extract OIDC identity from certificate extensions
        let oidc_identity = None;

        Ok(VerificationResult {
            certificate_hashes,
            signing_time,
            subject_digest,
            oidc_identity,
        })
    }
}
