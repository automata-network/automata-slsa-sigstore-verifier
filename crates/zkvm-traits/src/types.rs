use serde::{Deserialize, Serialize};
use sigstore_verifier::types::certificate::OidcIdentity;
use sigstore_verifier::types::result::{CertificateChainHashes, VerificationOptions};

/// Input data for the zkVM prover
///
/// This structure contains all the necessary data for the guest program
/// to perform sigstore bundle verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProverInput {
    /// Sigstore attestation bundle in JSON format
    pub bundle_json: Vec<u8>,

    /// Options for verification (expected digest, issuer, subject, etc.)
    pub verification_options: VerificationOptions,

    /// Trust bundle containing Fulcio certificate chain in PEM format
    pub trust_bundle_pem: Vec<u8>,

    /// Optional TSA certificate chain in PEM format for RFC3161 timestamp verification
    pub tsa_cert_chain_pem: Option<Vec<u8>>,
}

/// Output data from the zkVM prover
///
/// This structure contains the verification result that was computed
/// inside the guest program and committed to the public output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProverOutput {
    /// Hashes of the certificate chain (leaf, intermediates, root)
    pub certificate_hashes: CertificateChainHashes,

    /// Signing time as Unix timestamp (seconds since epoch)
    pub signing_time: i64,

    /// Digest of the signed subject (artifact)
    pub subject_digest: Vec<u8>,

    /// Optional OIDC identity extracted from the certificate
    pub oidc_identity: Option<OidcIdentity>,
}

impl ProverInput {
    /// Create a new ProverInput with the given parameters
    pub fn new(
        bundle_json: Vec<u8>,
        verification_options: VerificationOptions,
        trust_bundle_pem: Vec<u8>,
        tsa_cert_chain_pem: Option<Vec<u8>>,
    ) -> Self {
        Self {
            bundle_json,
            verification_options,
            trust_bundle_pem,
            tsa_cert_chain_pem,
        }
    }
}

impl ProverOutput {
    /// Create a new ProverOutput with the given parameters
    pub fn new(
        certificate_hashes: CertificateChainHashes,
        signing_time: i64,
        subject_digest: Vec<u8>,
        oidc_identity: Option<OidcIdentity>,
    ) -> Self {
        Self {
            certificate_hashes,
            signing_time,
            subject_digest,
            oidc_identity,
        }
    }
}
