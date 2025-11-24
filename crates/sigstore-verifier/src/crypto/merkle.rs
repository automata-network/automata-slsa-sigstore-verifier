use crate::crypto::hash::sha256;
use crate::error::TransparencyError;

pub fn verify_inclusion_proof(
    leaf_hash: &[u8],
    log_index: u64,
    tree_size: u64,
    proof_hashes: &[Vec<u8>],
    root_hash: &[u8],
) -> Result<(), TransparencyError> {
    if log_index >= tree_size {
        return Err(TransparencyError::InclusionProofFailed);
    }

    let mut computed_hash = leaf_hash.to_vec();
    let mut index = log_index;
    let mut size = tree_size;

    for proof_hash in proof_hashes {
        if size <= 1 {
            return Err(TransparencyError::InclusionProofFailed);
        }

        let (left, right) = if index % 2 == 0 && index + 1 < size {
            // Current node is left sibling
            (&computed_hash[..], &proof_hash[..])
        } else {
            // Current node is right sibling or last node
            (&proof_hash[..], &computed_hash[..])
        };

        // Hash parent: SHA256(0x01 || left || right)
        let mut parent_data = Vec::with_capacity(1 + left.len() + right.len());
        parent_data.push(0x01);
        parent_data.extend_from_slice(left);
        parent_data.extend_from_slice(right);
        computed_hash = sha256(&parent_data).to_vec();

        index /= 2;
        size = (size + 1) / 2;
    }

    if computed_hash == root_hash {
        Ok(())
    } else {
        Err(TransparencyError::InclusionProofFailed)
    }
}

pub fn compute_leaf_hash(data: &[u8]) -> [u8; 32] {
    // RFC 6962: leaf hash = SHA256(0x00 || data)
    let mut leaf_data = Vec::with_capacity(1 + data.len());
    leaf_data.push(0x00);
    leaf_data.extend_from_slice(data);
    sha256(&leaf_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_leaf_hash() {
        let data = b"test data";
        let hash = compute_leaf_hash(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_verify_inclusion_proof_simple() {
        // Test with a simple tree
        let leaf = vec![1u8; 32];
        let proof = vec![];

        // Single leaf tree
        let result = verify_inclusion_proof(&leaf, 0, 1, &proof, &leaf);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_inclusion_proof_index_out_of_bounds() {
        let leaf = vec![1u8; 32];
        let root = vec![2u8; 32];
        let proof = vec![];

        let result = verify_inclusion_proof(&leaf, 5, 3, &proof, &root);
        assert!(result.is_err());
    }
}
