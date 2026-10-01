use time::Timestamp;

use crate::text::observation::history::shard::change::{Change, edit::Edit, working::WorkingChange};

impl WorkingChange {
    pub fn add(&mut self, edits: Vec<Edit>) {
        self.edits.extend(edits);
    }

    pub fn complete(self, timestamp: Timestamp) -> Change {
        Change {
            edits: self.edits,
            parents: self.parents,
            timestamp
        }
    }
}