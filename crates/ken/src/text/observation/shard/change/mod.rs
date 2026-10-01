pub mod edit;
pub mod working;

use blake3::Hash;
use time::Timestamp;

use crate::text::observation::shard::change::edit::Edit;

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