use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct CertificateChain {
    pub leaf: Vec<u8>,          // DER-encoded
    pub intermediates: Vec<Vec<u8>>, // DER-encoded
    pub root: Vec<u8>,          // DER-encoded
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustBundle {
    pub chains: Vec<CertChain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertChain {
    pub certificates: Vec<String>, // PEM-encoded certificates
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FulcioInstance {
    GitHub,
    PublicGood,
}

impl FulcioInstance {
    pub fn trust_bundle_url(&self) -> &'static str {
        match self {
            FulcioInstance::GitHub => "https://fulcio.githubapp.com/api/v2/trustBundle",
            FulcioInstance::PublicGood => "https://fulcio.sigstore.dev/api/v2/trustBundle",
        }
    }

    pub fn from_issuer_cn(cn: &str) -> Option<Self> {
        match cn {
            "Fulcio Intermediate l2" => Some(FulcioInstance::GitHub),
            "sigstore-intermediate" => Some(FulcioInstance::PublicGood),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OidcIdentity {
    pub issuer: Option<String>,
    pub subject: Option<String>,
    pub workflow_ref: Option<String>,
    pub repository: Option<String>,
    pub event_name: Option<String>,
}
