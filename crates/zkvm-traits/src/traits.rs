use async_trait::async_trait;
use crate::{error::ZkVmError, types::ProverInput};

/// Trait for zkVM provers that generate proofs of sigstore verification
///
/// This trait defines the common interface that all zkVM implementations
/// (RISC0, SP1, etc.) must implement to generate zero-knowledge proofs
/// that verify sigstore attestation bundles.
#[async_trait]
pub trait ZkVmProver: Sized {
    /// Configuration type specific to this zkVM prover
    ///
    /// Each zkVM implementation will have its own configuration type
    /// that specifies proving strategy, network settings, etc.
    type Config;

    /// Create a new prover instance
    ///
    /// # Returns
    /// A new instance of the prover, ready to generate proofs
    fn new() -> Result<Self, ZkVmError>;

    /// Generate a zero-knowledge proof for the given input
    ///
    /// This method takes the prover input (sigstore bundle, verification options,
    /// trust bundles) and generates a proof that the verification succeeded.
    ///
    /// # Arguments
    /// * `config` - zkVM-specific configuration for proof generation
    /// * `input` - The input data containing the bundle and verification parameters
    ///
    /// # Returns
    /// A tuple of (public_output, proof_bytes) where:
    /// - `public_output`: The serialized ProverOutput containing verification results
    /// - `proof_bytes`: The zkVM proof that can be verified on-chain
    async fn prove(
        &self,
        config: &Self::Config,
        input: &ProverInput,
    ) -> Result<(Vec<u8>, Vec<u8>), ZkVmError>;

    /// Get the program identifier required for on-chain proof verification
    ///
    /// Different zkVMs use different identifiers:
    /// - RISC0: Returns the ImageID (computed from the guest ELF)
    /// - SP1: Returns the verifying key hash as bytes32 string
    ///
    /// # Returns
    /// The program identifier as a hex-encoded string
    fn program_identifier(&self) -> Result<String, ZkVmError>;

    /// Get the zkVM circuit version used for proof generation
    ///
    /// This is a static method that returns the version of the zkVM
    /// circuit/prover being used.
    ///
    /// # Returns
    /// The circuit version as a string (e.g., "v1.0.0")
    fn circuit_version() -> String;

    /// Get the guest program ELF binary
    ///
    /// Returns a reference to the compiled guest program that will be
    /// executed inside the zkVM to perform verification.
    ///
    /// # Returns
    /// A static reference to the ELF binary bytes
    fn elf(&self) -> &'static [u8];
}
