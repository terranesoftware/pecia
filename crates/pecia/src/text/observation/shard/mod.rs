pub mod change;
pub mod reference;

use blake3::Hash;
use indexmap::IndexMap;

use crate::text::observation::shard::{change::Change, reference::Reference};

/// A fragment of a view's complete history.
pub struct Shard {
    changes: IndexMap<Hash, Change>,
    references: Vec<Reference>
}

impl Shard {
    /// Returns a reference to the contained changes.
    pub fn changes(&self) -> &IndexMap<Hash, Change> {
        &self.changes
    }

    /// Returns a reference to the contained references.
    pub fn references(&self) -> &[Reference] {
        &self.references
    }
}