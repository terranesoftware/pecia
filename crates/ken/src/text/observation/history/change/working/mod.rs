pub mod operations;

use blake3::Hash;

use crate::text::observation::history::change::edit::Edit;

pub struct WorkingChange {
    edits: Vec<Edit>,
    parent: Option<Hash>
}

impl WorkingChange {
    pub fn new(parent: Option<Hash>) -> Self {
        Self {
            edits: Vec::new(),
            parent
        }
    }
    
    pub fn edits(&self) -> &[Edit] {
        &self.edits
    }
}