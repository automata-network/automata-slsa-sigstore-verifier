use sigstore_verifier::{types::VerificationOptions, AttestationVerifier};
use std::path::PathBuf;

#[test]
#[ignore] // Requires network access to fetch trust bundles
fn test_verify_real_bundle() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("samples/actions-attest-build-provenance-attestation-13581567.sigstore.json");

    let verifier = AttestationVerifier::new();
    let options = VerificationOptions {
        expected_digest: None,
        verify_rekor: true,
        allow_insecure_sct: false,
        expected_issuer: None,
        expected_subject: None,
    };

    let result = verifier.verify_bundle(&path, options);
    assert!(result.is_ok(), "Verification failed: {:?}", result.err());
    
    // // This test may fail due to network or certificate expiry
    // // For now, we just verify it doesn't panic
    // match result {
    //     Ok(verification_result) => {
    //         println!("Verification succeeded!");
    //         println!(
    //             "Leaf hash: {}",
    //             hex::encode(&verification_result.certificate_hashes.leaf)
    //         );
    //         println!(
    //             "Root hash: {}",
    //             hex::encode(&verification_result.certificate_hashes.root)
    //         );
    //         println!("Signing time: {}", verification_result.signing_time);
    //     }
    //     Err(e) => {
    //         println!("Verification failed (expected for expired certs): {}", e);
    //     }
    // }
}