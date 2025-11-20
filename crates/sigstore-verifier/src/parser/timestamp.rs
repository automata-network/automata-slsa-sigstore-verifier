use asn1_rs::{FromDer, Sequence};
use chrono::{DateTime, Utc};

use crate::error::TimestampError;

#[derive(Debug, Clone)]
pub struct Rfc3161TimestampInfo {
    pub signing_time: DateTime<Utc>,
    pub raw_bytes: Vec<u8>,
}

pub fn parse_rfc3161_timestamp(der: &[u8]) -> Result<Rfc3161TimestampInfo, TimestampError> {
    // RFC 3161 TimeStampToken is a CMS SignedData structure
    // For now, we'll do basic ASN.1 parsing to extract the time
    // A full implementation would verify the signature as well

    let (_, _sequence) = Sequence::from_der(der)
        .map_err(|e| TimestampError::Rfc3161Parse(e.to_string()))?;

    // TODO: Proper ASN.1 parsing of TimeStampToken
    // This is a simplified placeholder that extracts integrated time instead
    // A complete implementation should:
    // 1. Parse ContentInfo
    // 2. Extract SignedData
    // 3. Extract EncapsulatedContentInfo
    // 4. Parse TSTInfo
    // 5. Extract genTime field

    Err(TimestampError::Rfc3161Parse(
        "RFC3161 timestamp parsing not yet fully implemented".to_string(),
    ))
}

pub fn parse_integrated_time(time_str: &str) -> Result<DateTime<Utc>, TimestampError> {
    let timestamp = time_str
        .parse::<i64>()
        .map_err(|_| TimestampError::InvalidIntegratedTime)?;

    DateTime::from_timestamp(timestamp, 0)
        .ok_or(TimestampError::InvalidIntegratedTime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integrated_time() {
        let time_str = "1732068373";
        let result = parse_integrated_time(time_str);
        assert!(result.is_ok());

        let dt = result.unwrap();
        assert_eq!(dt.timestamp(), 1732068373);
    }

    #[test]
    fn test_parse_integrated_time_invalid() {
        let result = parse_integrated_time("not_a_number");
        assert!(result.is_err());
    }
}
