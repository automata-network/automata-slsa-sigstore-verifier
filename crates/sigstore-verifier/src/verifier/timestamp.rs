use chrono::{DateTime, Utc};
use x509_parser::prelude::*;

use crate::error::{CertificateError, TimestampError, VerificationError};
use crate::parser::parse_integrated_time;
use crate::types::{SigstoreBundle, TransparencyLogEntry};

pub fn get_signing_time(bundle: &SigstoreBundle) -> Result<DateTime<Utc>, VerificationError> {
    // Check if bundle has RFC3161 timestamps
    let has_rfc3161 = bundle
        .verification_material
        .timestamp_verification_data
        .as_ref()
        .and_then(|td| td.rfc3161_timestamps.as_ref())
        .map(|ts| !ts.is_empty())
        .unwrap_or(false);

    // Check if bundle has transparency log entries
    let has_tlog = bundle
        .verification_material
        .tlog_entries
        .as_ref()
        .map(|entries| !entries.is_empty())
        .unwrap_or(false);

    // RFC3161 timestamp verification is not yet implemented
    // See RFC-3161.md for implementation requirements
    if has_rfc3161 && !has_tlog {
        return Err(TimestampError::Rfc3161NotSupported.into());
    }

    // Use integrated time from transparency log
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
