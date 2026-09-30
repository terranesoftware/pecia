pub mod change;
pub mod reference;

use blake3::Hash;
use indexmap::IndexMap;

use crate::text::observation::history::{change::Change, reference::Reference};

pub struct History {
    changes: IndexMap<Hash, Change>,
    references: Vec<Reference>
}

impl History {
    pub fn changes(&self) -> &IndexMap<Hash, Change> {
        &self.changes
    }

    pub fn references(&self) -> &[Reference] {
        &self.references
    }
}