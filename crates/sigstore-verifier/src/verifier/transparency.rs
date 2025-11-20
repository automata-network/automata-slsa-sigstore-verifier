use crate::crypto::verify_inclusion_proof;
use crate::error::{TransparencyError, VerificationError};
use crate::parser::decode_base64;
use crate::types::SigstoreBundle;

pub fn verify_transparency_log(
    bundle: &SigstoreBundle,
    skip_verification: bool,
) -> Result<(), VerificationError> {
    if skip_verification {
        return Ok(());
    }

    let tlog_entries = bundle
        .verification_material
        .tlog_entries
        .as_ref()
        .ok_or(TransparencyError::NoRekorEntry)?;

    if tlog_entries.is_empty() {
        return Err(TransparencyError::NoRekorEntry.into());
    }

    let entry = &tlog_entries[0];

    // Verify inclusion proof if present
    if let Some(ref inclusion_proof) = entry.inclusion_proof {
        let log_index = inclusion_proof
            .log_index
            .parse::<u64>()
            .map_err(|_| TransparencyError::InvalidEntryHash)?;

        let tree_size = inclusion_proof
            .tree_size
            .parse::<u64>()
            .map_err(|_| TransparencyError::InvalidEntryHash)?;

        let root_hash = decode_base64(&inclusion_proof.root_hash)
            .map_err(|_| TransparencyError::InvalidEntryHash)?;

        let mut proof_hashes = Vec::new();
        for hash_b64 in &inclusion_proof.hashes {
            let hash = decode_base64(hash_b64)
                .map_err(|_| TransparencyError::InvalidEntryHash)?;
            proof_hashes.push(hash);
        }

        // Compute leaf hash from canonicalized body
        let canonicalized_body = decode_base64(&entry.canonicalized_body)
            .map_err(|_| TransparencyError::InvalidEntryHash)?;
        let leaf_hash = crate::crypto::compute_leaf_hash(&canonicalized_body);

        // Verify inclusion proof
        verify_inclusion_proof(&leaf_hash, log_index, tree_size, &proof_hashes, &root_hash)?;
    }

    // Verify signed entry timestamp if present
    if let Some(ref inclusion_promise) = entry.inclusion_promise {
        // TODO: Verify the signed entry timestamp signature
        // This requires fetching the Rekor public key and verifying the signature
        // For now, we just check it exists
        let _set_bytes = decode_base64(&inclusion_promise.signed_entry_timestamp)
            .map_err(|_| TransparencyError::SignedEntryTimestampInvalid)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_verification() {
        let bundle = SigstoreBundle {
            media_type: String::new(),
            verification_material: crate::types::VerificationMaterial {
                timestamp_verification_data: None,
                certificate: crate::types::Certificate {
                    raw_bytes: String::new(),
                },
                tlog_entries: None,
            },
            dsse_envelope: crate::types::DsseEnvelope {
                payload: String::new(),
                payload_type: String::new(),
                signatures: vec![],
            },
        };

        let result = verify_transparency_log(&bundle, true);
        assert!(result.is_ok());
    }
}
