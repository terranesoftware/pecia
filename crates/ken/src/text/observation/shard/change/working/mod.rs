pub mod operations;

use blake3::Hash;

use crate::text::observation::shard::change::edit::Edit;

pub struct WorkingChange {
    edits: Vec<Edit>,
    parents: Vec<Hash>
}

impl WorkingChange {
    pub fn new(parents: Vec<Hash>) -> Self {
        Self {
            edits: Vec::new(),
            parents
        }
    }
    
    pub fn edits(&self) -> &[Edit] {
        &self.edits
    }
}