mod edit;
pub use edit::Edit;

use blake3::Hash;
use time::Timestamp;

pub struct Change {
    pub edits: Vec<Edit>,
    pub parents: Vec<Hash>,
    pub timestamp: Timestamp
}