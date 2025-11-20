use chrono::{DateTime, Utc};
use x509_parser::prelude::*;

use crate::error::{CertificateError, TimestampError, VerificationError};
use crate::parser::{decode_base64, parse_integrated_time};
use crate::types::{SigstoreBundle, TransparencyLogEntry};

pub fn get_signing_time(bundle: &SigstoreBundle) -> Result<DateTime<Utc>, VerificationError> {
    // Try RFC3161 timestamp first
    if let Some(ref timestamp_data) = bundle.verification_material.timestamp_verification_data {
        if let Some(ref rfc3161_timestamps) = timestamp_data.rfc3161_timestamps {
            if !rfc3161_timestamps.is_empty() {
                // For now, we'll use integrated time as fallback
                // TODO: Implement full RFC3161 parsing
                // let timestamp_der = decode_base64(&rfc3161_timestamps[0].signed_timestamp)?;
                // return parse_rfc3161_timestamp(&timestamp_der)
                //     .map(|info| info.signing_time)
                //     .map_err(|e| e.into());
            }
        }
    }

    // Fall back to integrated time from transparency log
    if let Some(ref tlog_entries) = bundle.verification_material.tlog_entries {
        if let Some(entry) = tlog_entries.first() {
            return get_integrated_time(entry).map_err(|e| e.into());
        }
    }

    Err(TimestampError::NoTimestamp.into())
}

fn get_integrated_time(entry: &TransparencyLogEntry) -> Result<DateTime<Utc>, TimestampError> {
    parse_integrated_time(&entry.integrated_time)
}

pub fn verify_signing_time_in_validity(
    signing_time: &DateTime<Utc>,
    cert: &X509Certificate,
) -> Result<(), CertificateError> {
    let validity = cert.validity();
    let not_before = validity.not_before.timestamp();
    let not_after = validity.not_after.timestamp();
    let signing_timestamp = signing_time.timestamp();

    if signing_timestamp < not_before || signing_timestamp > not_after {
        return Err(CertificateError::SigningTimeOutsideValidity {
            signing_time: signing_time.to_rfc3339(),
            not_before: validity.not_before.to_string(),
            not_after: validity.not_after.to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_get_integrated_time() {
        let entry = TransparencyLogEntry {
            log_index: Some("123".to_string()),
            log_id: None,
            kind_version: None,
            integrated_time: "1732068373".to_string(),
            inclusion_promise: None,
            inclusion_proof: None,
            canonicalized_body: String::new(),
        };

        let result = get_integrated_time(&entry);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().timestamp(), 1732068373);
    }
}
