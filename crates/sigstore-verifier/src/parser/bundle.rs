use std::path::Path;

use base64::prelude::*;
use crate::error::VerificationError;
use crate::parser::rfc3161::parse_rfc3161_timestamp;
use crate::parser::timestamp::parse_integrated_time;
use crate::types::bundle::{DsseEnvelope, SigstoreBundle};
use crate::types::dsse::Statement;

pub fn parse_bundle_from_path(path: &Path) -> Result<SigstoreBundle, VerificationError> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| VerificationError::InvalidBundleFormat(e.to_string()))?;
    parse_bundle_from_str(&contents)
}

pub fn parse_bundle_from_bytes(bytes: &[u8]) -> Result<SigstoreBundle, VerificationError> {
    let bundle: SigstoreBundle = serde_json::from_slice(bytes)?;
    validate_bundle(&bundle)?;
    Ok(bundle)
}

pub fn parse_bundle_from_str(json: &str) -> Result<SigstoreBundle, VerificationError> {
    let bundle: SigstoreBundle = serde_json::from_str(json)?;
    validate_bundle(&bundle)?;
    Ok(bundle)
}

fn validate_bundle(bundle: &SigstoreBundle) -> Result<(), VerificationError> {
    if !bundle
        .media_type
        .starts_with("application/vnd.dev.sigstore.bundle")
    {
        return Err(VerificationError::InvalidBundleFormat(format!(
            "Unsupported media type: {}",
            bundle.media_type
        )));
    }

    if bundle.dsse_envelope.signatures.is_empty() {
        return Err(VerificationError::InvalidBundleFormat(
            "No signatures in DSSE envelope".to_string(),
        ));
    }

    Ok(())
}

pub fn parse_dsse_payload(envelope: &DsseEnvelope) -> Result<Statement, VerificationError> {
    let payload_bytes = BASE64_STANDARD.decode(&envelope.payload)?;
    let statement: Statement = serde_json::from_slice(&payload_bytes)?;
    Ok(statement)
}

pub fn decode_base64(input: &str) -> Result<Vec<u8>, VerificationError> {
    BASE64_STANDARD.decode(input).map_err(|e| e.into())
}

/// Extract timestamp from a Sigstore bundle in Unix seconds.
/// This extracts the genTime from the RFC 3161 timestamp token.
///
/// # Arguments
/// * `bundle` - Parsed Sigstore bundle
///
/// # Returns
/// Unix timestamp in seconds
pub fn extract_bundle_timestamp(bundle: &SigstoreBundle) -> Result<i64, VerificationError> {
    if let Some(timestamp_data) = bundle
        .verification_material
        .timestamp_verification_data
        .as_ref()
    {
        if let Some(rfc3161_timestamps) = timestamp_data.rfc3161_timestamps.as_ref() {
            if !rfc3161_timestamps.is_empty() {
                let signed_timestamp = &rfc3161_timestamps[0].signed_timestamp;
                let timestamp_der = decode_base64(signed_timestamp)?;

                let parsed_timestamp = parse_rfc3161_timestamp(&timestamp_der).map_err(|e| {
                    VerificationError::InvalidBundleFormat(format!("Failed to parse timestamp: {}", e))
                })?;

                return Ok(parsed_timestamp.tst_info.gen_time.timestamp());
            }
        }
    }

    if let Some(tlog_entries) = bundle.verification_material.tlog_entries.as_ref() {
        if let Some(entry) = tlog_entries.first() {
            let dt = parse_integrated_time(&entry.integrated_time).map_err(|e| {
                VerificationError::InvalidBundleFormat(format!("Failed to parse integrated time: {}", e))
            })?;
            return Ok(dt.timestamp());
        }
    }

    Err(VerificationError::InvalidBundleFormat(
        "No RFC 3161 timestamp or transparency log integrated time found".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_bundle_invalid_media_type() {
        use crate::types::bundle::{Certificate, Signature, VerificationMaterial};

        let mut bundle = SigstoreBundle {
            media_type: "invalid".to_string(),
            verification_material: VerificationMaterial {
                timestamp_verification_data: None,
                certificate: Certificate {
                    raw_bytes: String::new(),
                },
                tlog_entries: None,
            },
            dsse_envelope: DsseEnvelope {
                payload: String::new(),
                payload_type: String::new(),
                signatures: vec![Signature {
                    sig: String::new(),
                }],
            },
        };

        let result = validate_bundle(&bundle);
        assert!(result.is_err());

        bundle.media_type = "application/vnd.dev.sigstore.bundle.v0.3+json".to_string();
        assert!(validate_bundle(&bundle).is_ok());
    }
}
