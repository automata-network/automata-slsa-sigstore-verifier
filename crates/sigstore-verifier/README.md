# Sigstore Verifier

A Rust library for verifying software build attestations according to the Sigstore specification.

## Features

- Verifies Sigstore bundles (format v0.3+)
- Supports both GitHub Fulcio and public Sigstore instances
- Validates DSSE envelope signatures
- Verifies certificate chains with automatic trust bundle fetching
- Supports RFC3161 timestamps and Rekor integrated time
- Merkle tree inclusion proof verification
- Returns SHA256 hashes of the entire certificate chain

## Verification Workflow

The library performs the following verification steps:

1. **Subject Digest Validation**: Checks that the attestation subject digest is not zero and optionally matches an expected value
2. **Timestamp Verification**: Extracts signing time from RFC3161 timestamps or Rekor integrated time, then verifies the signing time falls within the certificate's validity period
3. **Certificate Chain Verification**: Fetches the issuer certificate chain from Fulcio and verifies the entire chain up to the root
4. **Signature Verification**: Verifies the DSSE envelope signature using the public key extracted from the leaf certificate
5. **Transparency Log Verification**: Verifies the Rekor merkle tree inclusion proof (optional)

## Usage

```rust
use std::path::Path;
use sigstore_verifier::{AttestationVerifier, Options};

let verifier = AttestationVerifier::new();

let options = Options {
    expected_digest: None,
    verify_rekor: true,
    allow_insecure_sct: false,
    expected_issuer: None,
    expected_subject: None,
};

let result = verifier.verify_bundle(
    Path::new("path/to/bundle.sigstore.json"),
    options
)?;

println!("Leaf cert hash: {}", hex::encode(&result.certificate_hashes.leaf));
println!("Root cert hash: {}", hex::encode(&result.certificate_hashes.root));
println!("Signing time: {}", result.signing_time);
println!("Subject digest: {}", hex::encode(&result.subject_digest));
```

## Return Value

On successful verification, the library returns a `VerificationResult` containing:

```rust
pub struct VerificationResult {
    pub certificate_hashes: CertificateChainHashes,
    pub signing_time: DateTime<Utc>,
    pub subject_digest: Vec<u8>,
    pub oidc_identity: Option<OidcIdentity>,
}

pub struct CertificateChainHashes {
    pub leaf: [u8; 32],
    pub intermediates: Vec<[u8; 32]>,
    pub root: [u8; 32],
}
```

The certificate hashes can be used to verify the trust chain and track which certificates were used for signing.

## Supported Signature Algorithms

- ECDSA with secp256r1 (P-256)
- ECDSA with secp384r1 (P-384)

The library automatically detects the signature algorithm from the certificate's Subject Public Key Info.

## Limitations

- RFC3161 timestamp parsing is not yet fully implemented (falls back to integrated time)
- Rekor signed entry timestamp verification is not yet fully implemented
- OIDC identity extraction from certificate extensions is not yet implemented
- Certificate revocation checking is not implemented

## Testing

Run the test suite:

```bash
cargo test --package sigstore-verifier
```

Run integration tests (requires network access):

```bash
cargo test --package sigstore-verifier -- --ignored
```

## License

See the repository root for license information.
