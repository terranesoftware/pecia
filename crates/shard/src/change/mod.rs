mod edit;
pub use edit::Edit;

use blake3::Hash;
use serde::Serialize;
use time::Timestamp;

/// A group of modifications that happened in the same instant.
#[derive(Clone, Serialize)]
pub struct Change {
    pub edits: Vec<Edit>,
    pub parents: Vec<Hash>,
    pub timestamp: Timestamp
}