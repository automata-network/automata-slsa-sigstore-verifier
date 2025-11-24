use std::fmt;

/// Error types for zkVM operations
#[derive(Debug)]
pub enum ZkVmError {
    /// Error during proof generation
    ProofGenerationError(String),

    /// Error during serialization/deserialization
    SerializationError(String),

    /// Invalid input provided to the prover
    InvalidInput(String),

    /// Error from the underlying zkVM implementation
    ZkVmImplementationError(String),

    /// Generic error
    Other(String),
}

impl fmt::Display for ZkVmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkVmError::ProofGenerationError(msg) => write!(f, "Proof generation error: {}", msg),
            ZkVmError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            ZkVmError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ZkVmError::ZkVmImplementationError(msg) => write!(f, "zkVM implementation error: {}", msg),
            ZkVmError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ZkVmError {}

impl From<anyhow::Error> for ZkVmError {
    fn from(err: anyhow::Error) -> Self {
        ZkVmError::Other(err.to_string())
    }
}
