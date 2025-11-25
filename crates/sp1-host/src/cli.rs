//! Command-line interface definitions for sp1-host
//!
//! Defines all CLI commands, subcommands, and arguments using clap.

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "sp1-host",
    author,
    version,
    about = "SP1 zkVM host program for Sigstore attestation verification",
    long_about = "Generate zero-knowledge proofs of Sigstore attestation bundle verification using SP1 zkVM"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display the SP1 program verifying key hash
    #[command(name = "verifying-key")]
    VerifyingKey,

    /// Generate a proof of attestation verification
    Prove(ProveArgs),
}

#[derive(Args, Debug)]
pub struct ProveArgs {
    /// Path to the Sigstore attestation bundle JSON file
    #[arg(long = "bundle", value_name = "PATH", required = true)]
    pub bundle_path: PathBuf,

    /// Path to the trusted root JSONL file
    #[arg(long = "trust-roots", value_name = "PATH", required = true)]
    pub trust_roots_path: PathBuf,

    /// Path to write the proof artifact JSON file
    #[arg(long = "output", value_name = "PATH")]
    pub output_path: Option<PathBuf>,

    /// SP1 network private key (hex-encoded)
    #[arg(
        long = "network-private-key",
        env = "SP1_NETWORK_PRIVATE_KEY",
        value_name = "WALLET_KEY",
        hide_env_values = true
    )]
    pub private_key: String,

    /// Proving mode
    #[arg(
        long = "mode",
        value_enum,
        default_value = "groth16",
        value_name = "MODE"
    )]
    pub mode: ProvingMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ProvingMode {
    /// Compressed SNARK proof
    #[value(name = "compressed")]
    Compressed,

    /// Groth16 proof (optimized for on-chain verification)
    #[value(name = "groth16")]
    Groth16,

    /// Plonk proof
    #[value(name = "plonk")]
    Plonk,
}
