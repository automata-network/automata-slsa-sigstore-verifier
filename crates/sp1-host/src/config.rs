//! Configuration types for SP1 proving
//!
//! Defines configuration structures for different proving strategies and modes.

use crate::cli::{ProveArgs, ProvingMode};

/// SP1 prover configuration
#[derive(Debug, Clone)]
pub struct Sp1Config {
    pub proving_mode: ProvingMode,
    pub private_key: String
}

impl Sp1Config {
    /// Build a Sp1Config from CLI arguments
    ///
    /// # Arguments
    ///
    /// * `args` - The prove command arguments
    ///
    /// # Returns
    ///
    /// Returns a Sp1Config with the appropriate strategy and parameters.
    pub fn from_cli_args(args: &ProveArgs) -> Self {
        Sp1Config {
            proving_mode: args.mode,
            private_key: args.private_key.clone(),
        }
    }
}
