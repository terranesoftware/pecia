mod edit;
pub use edit::Edit;

use blake3::Hash;
use time::Timestamp;

pub struct Change {
    edits: Vec<Edit>,
    parents: Vec<Hash>,
    timestamp: Timestamp
}

impl Change {
    pub fn edits(&self) -> &[Edit] {
        &self.edits
    }

    pub fn parents(&self) -> &[Hash] {
        &self.parents
    }

    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}