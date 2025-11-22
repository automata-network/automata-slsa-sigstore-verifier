use serde::{Deserialize, Serialize};

/// Sigstore TrustedRoot bundle format
/// Spec: https://github.com/sigstore/protobuf-specs/blob/main/protos/sigstore_trustroot.proto
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrustedRoot {
    pub media_type: String,
    #[serde(default)]
    pub tlogs: Vec<TransparencyLogInstance>,
    #[serde(default)]
    pub certificate_authorities: Vec<CertificateAuthority>,
    #[serde(default)]
    pub ctlogs: Vec<TransparencyLogInstance>,
    #[serde(default)]
    pub timestamp_authorities: Vec<TimestampAuthority>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateAuthority {
    pub subject: Subject,
    pub uri: String,
    pub cert_chain: CertChain,
    pub valid_for: ValidityPeriod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimestampAuthority {
    pub subject: Subject,
    pub uri: String,
    pub cert_chain: CertChain,
    pub valid_for: ValidityPeriod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub organization: String,
    pub common_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertChain {
    pub certificates: Vec<Certificate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    pub raw_bytes: String, // base64-encoded DER
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidityPeriod {
    pub start: Option<String>, // RFC3339 timestamp
    pub end: Option<String>,   // RFC3339 timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransparencyLogInstance {
    pub base_url: String,
    #[serde(default)]
    pub hash_algorithm: Option<String>,
    #[serde(default)]
    pub public_key: Option<PublicKey>,
    #[serde(default)]
    pub log_id: Option<LogId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    #[serde(default)]
    pub raw_bytes: Option<String>, // base64-encoded
    #[serde(default)]
    pub key_details: Option<String>,
    #[serde(default)]
    pub valid_for: Option<ValidityPeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogId {
    pub key_id: String, // base64-encoded
}
