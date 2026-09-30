pub mod edit;

use blake3::Hash;
use time::Timestamp;

use crate::text::observation::history::change::edit::Edit;

pub struct Change {
    edits: Vec<Edit>,
    parent: Hash,
    timestamp: Timestamp
}

impl Change {
    pub fn edits(&self) -> &[Edit] {
        &self.edits
    }

    pub fn parent(&self) -> Hash {
        self.parent
    }

    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}