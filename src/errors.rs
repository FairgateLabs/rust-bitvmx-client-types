use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitVMXError {
    #[error("Invalid variable type: {0}")]
    InvalidVariableType(String),

    #[error("Invalid witness type")]
    InvalidWitnessType,

    #[error("Invalid Conversion {0}")]
    InvalidConversion(String),

    #[error("Invalid Comms address: {0}")]
    InvalidCommsAddress(String),

    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    #[error("Participant key '{name}' not found (expected {expected_type})")]
    ParticipantKeyNotFound {
        name: String,
        expected_type: &'static str,
    },

    #[error("Participant key '{name}' has type {actual_type}, expected {expected_type}")]
    ParticipantKeyTypeMismatch {
        name: String,
        expected_type: &'static str,
        actual_type: &'static str,
    },

    #[error("Invalid merkle tree")]
    InvalidMerkleTree,

    #[error("Transaction not found in block")]
    TransactionNotFoundInBlock,

    #[error("Serialization error {0}")]
    SerdeSerializationError(#[from] serde_json::Error),
}
