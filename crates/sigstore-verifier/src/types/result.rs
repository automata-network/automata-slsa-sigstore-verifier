use chrono::{DateTime, Utc};

use super::certificate::OidcIdentity;

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub certificate_hashes: CertificateChainHashes,
    pub signing_time: DateTime<Utc>,
    pub subject_digest: Vec<u8>,
    pub oidc_identity: Option<OidcIdentity>,
}

#[derive(Debug, Clone)]
pub struct CertificateChainHashes {
    pub leaf: [u8; 32],
    pub intermediates: Vec<[u8; 32]>,
    pub root: [u8; 32],
}

impl CertificateChainHashes {
    pub fn as_tuple(&self) -> ([u8; 32], Vec<[u8; 32]>, [u8; 32]) {
        (self.leaf, self.intermediates.clone(), self.root)
    }
}

#[derive(Debug, Clone, Default)]
pub struct VerificationOptions {
    pub expected_digest: Option<Vec<u8>>,
    pub verify_rekor: bool,
    pub allow_insecure_sct: bool,
    pub expected_issuer: Option<String>,
    pub expected_subject: Option<String>,
}
