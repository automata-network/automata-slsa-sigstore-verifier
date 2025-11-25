//! SP1 network proving integration
//!
//! Provides functionality to generate proofs using the SP1 proving network.

use crate::cli::ProvingMode;
use sigstore_zkvm_traits::error::ZkVmError;
use sp1_sdk::{NetworkProver, SP1ProvingKey, SP1Stdin, network::FulfillmentStrategy};

/// Generate a proof using the SP1 proving network
///
/// # Arguments
///
/// * `client` - SP1 prover client
/// * `elf` - Guest program ELF (for execute in Mock mode)
/// * `pk` - SP1 proving key
/// * `stdin` - Input data for the guest program (consumed)
/// * `mode` - Proving mode (Mock, Compressed, Groth16, Plonk)
///
/// # Returns
///
/// Returns (public_values, proof_bytes) on success.
///
/// # Errors
///
/// Returns an error if:
/// - RPC URL or private key is missing/invalid
/// - Network configuration is invalid
/// - Proof request submission fails
/// - Proof generation times out
pub async fn prove_with_network(
    client: &NetworkProver,
    pk: &SP1ProvingKey,
    stdin: SP1Stdin,
    mode: ProvingMode
) -> Result<(Vec<u8>, Vec<u8>), ZkVmError> {
    println!("🔗 Connecting to SP1 network...");
    println!("🚀 Submitting proof request to SP1 network...");

    match mode {
        ProvingMode::Compressed => {
            println!("🔐 Generating Compressed proof...");
            // Note: This uses local proving. Replace with network proving when SP1 network SDK is available
            let proof = client
                .prove(pk, &stdin)
                .compressed()
                .strategy(FulfillmentStrategy::Auction)
                .run()
                .map_err(|e| {
                    ZkVmError::ProofGenerationError(format!("Failed to generate compressed proof: {}", e))
                })?;
            println!("✓ Compressed proof generated successfully!");
            Ok((proof.public_values.to_vec(), proof.bytes()))
        }
        ProvingMode::Groth16 => {
            println!("🔐 Generating Groth16 proof...");
            // Note: This uses local proving. Replace with network proving when SP1 network SDK is available
            let proof = client
                .prove(pk, &stdin)
                .groth16()
                .strategy(FulfillmentStrategy::Auction)
                .run()
                .map_err(|e| {
                    ZkVmError::ProofGenerationError(format!("Failed to generate Groth16 proof: {}", e))
                })?;
            println!("✓ Groth16 proof generated successfully!");
            Ok((proof.public_values.to_vec(), proof.bytes()))
        }
        ProvingMode::Plonk => {
            println!("🔐 Generating Plonk proof...");
            // Note: This uses local proving. Replace with network proving when SP1 network SDK is available
            let proof = client
                .prove(pk, &stdin)
                .plonk()
                .strategy(FulfillmentStrategy::Auction)
                .run()
                .map_err(|e| {
                    ZkVmError::ProofGenerationError(format!("Failed to generate Plonk proof: {}", e))
                })?;
            println!("✓ Plonk proof generated successfully!");
            Ok((proof.public_values.to_vec(), proof.bytes()))
        }
    }
}
