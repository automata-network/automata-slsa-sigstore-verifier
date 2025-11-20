use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigstoreBundle {
    pub media_type: String,
    pub verification_material: VerificationMaterial,
    pub dsse_envelope: DsseEnvelope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMaterial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_verification_data: Option<TimestampVerificationData>,
    pub certificate: Certificate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tlog_entries: Option<Vec<TransparencyLogEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimestampVerificationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfc3161_timestamps: Option<Vec<Rfc3161Timestamp>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rfc3161Timestamp {
    pub signed_timestamp: String, // Base64-encoded
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    pub raw_bytes: String, // Base64-encoded DER certificate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransparencyLogEntry {
    pub log_index: Option<String>,
    pub log_id: Option<LogId>,
    pub kind_version: Option<KindVersion>,
    pub integrated_time: String,
    pub inclusion_promise: Option<InclusionPromise>,
    pub inclusion_proof: Option<InclusionProof>,
    pub canonicalized_body: String, // Base64-encoded
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogId {
    pub key_id: String, // Base64-encoded
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KindVersion {
    pub kind: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InclusionPromise {
    pub signed_entry_timestamp: String, // Base64-encoded
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InclusionProof {
    pub log_index: String,
    pub root_hash: String,   // Base64-encoded
    pub tree_size: String,
    pub hashes: Vec<String>, // Base64-encoded
    pub checkpoint: Option<Checkpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checkpoint {
    pub envelope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DsseEnvelope {
    pub payload: String,      // Base64-encoded
    pub payload_type: String,
    pub signatures: Vec<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub sig: String, // Base64-encoded
}
