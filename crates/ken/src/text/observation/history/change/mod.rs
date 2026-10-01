pub mod edit;
pub mod working;

use blake3::Hash;
use time::Timestamp;

use crate::text::observation::history::change::edit::Edit;

pub struct Change {
    edits: Vec<Edit>,
    parent: Option<Hash>,
    timestamp: Timestamp
}

impl Change {
    pub fn edits(&self) -> &[Edit] {
        &self.edits
    }

    pub fn parent(&self) -> Option<Hash> {
        self.parent
    }

    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}