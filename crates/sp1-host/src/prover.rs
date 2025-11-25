//! SP1 zkVM prover implementation
//!
//! Implements the ZkVmProver trait for SP1, providing proof generation
//! capabilities for Sigstore attestation verification.

use crate::config::Sp1Config;
use crate::proving::network::prove_with_network;
use async_trait::async_trait;
use sigstore_zkvm_traits::error::ZkVmError;
use sigstore_zkvm_traits::traits::ZkVmProver;
use sigstore_zkvm_traits::types::ProverInput;
use sp1_sdk::{EnvProver, HashableKey, Prover, ProverClient, SP1Stdin};
use sugstore_sp1_methods::{vk, SP1_SIGSTORE_ELF};

pub struct Sp1Prover {
    elf: &'static [u8],
}

#[async_trait]
impl ZkVmProver for Sp1Prover {
    type Config = Sp1Config;

    fn new() -> Result<Self, ZkVmError> {
        Ok(Sp1Prover {
            elf: SP1_SIGSTORE_ELF,
        })
    }

    async fn prove(
        &self,
        config: &Self::Config,
        input: &ProverInput,
    ) -> Result<(Vec<u8>, Vec<u8>), ZkVmError> {
        // Serialize input to bytes
        let input_bytes = input
            .encode_input()
            .map_err(|e| ZkVmError::InvalidInput(format!("Failed to encode ProverInput: {}", e)))?;

        // Log verifying key hash
        let vk = vk(self.elf);
        let vk_hash = vk.bytes32();
        println!("Verifying Key Hash: {}", vk_hash);
        println!("SP1 Version: {}", Self::circuit_version());

        // Build stdin with input bytes
        let mut stdin = SP1Stdin::new();
        stdin.write_vec(input_bytes.clone());

        // Check for DEV_MODE
        if std::env::var("DEV_MODE").is_ok() || std::env::var("SP1_DEV_MODE").is_ok() {
            println!("⚠ Running in DEV_MODE - no proof will be generated");
            let client = EnvProver::new();
            let (public_values, _) = client.execute(self.elf, &stdin).run().map_err(|e| {
                ZkVmError::ProofGenerationError(format!("Failed to execute guest program: {}", e))
            })?;
            return Ok((public_values.to_vec(), vec![]));
        }

        // Set up SP1 environment variables
        std::env::set_var("SP1_PROVER", "network");

        // Get private key from config or environment
        let sp1_network_key = config.private_key.as_str();
        std::env::set_var("NETWORK_PRIVATE_KEY", sp1_network_key);

        let client = ProverClient::builder()
            .network_for(sp1_sdk::network::NetworkMode::Mainnet)
            .build();

        // Get proving key for proof generation
        let (pk, _) = client.setup(self.elf);
        prove_with_network(&client, &pk, stdin, config.proving_mode).await
    }

    fn program_identifier(&self) -> Result<String, ZkVmError> {
        let vk = vk(self.elf);
        Ok(format!("{}", vk.bytes32()))
    }

    fn circuit_version() -> String {
        sp1_sdk::SP1_CIRCUIT_VERSION.to_string()
    }

    fn elf(&self) -> &'static [u8] {
        self.elf
    }
}
