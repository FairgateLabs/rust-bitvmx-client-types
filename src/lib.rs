//! `rust-bitvmx-client-types` — the BitVMX wire contract, published verbatim from
//! `rust-bitvmx-client` on every release tag. See README.md before hand-editing anything here.

pub mod errors;
pub mod program;
pub mod spv_proof;
pub mod types;

// Flat re-export of the contract, for union's convenience.
pub use errors::BitVMXError;
pub use program::participant::*;
pub use program::protocols::union::common::*;
pub use program::protocols::union::transport::*;
pub use program::protocols::union::types::*;
pub use program::variables::*;
pub use spv_proof::*;
pub use types::*;

// Upstream types the contract references but does not own.
pub use bitcoin_coordinator::{FullBlock, OutputPatternFilter, TransactionStatus};
pub use bitvmx_broker::identification::identifier::PubkHash;
pub use bitvmx_wallet::wallet::Destination;
pub use key_manager::{lamport, musig2, winternitz};
pub use protocol_builder::{scripts, types as protocol_types};
