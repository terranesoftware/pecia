pub mod change;
pub mod reference;

use blake3::Hash;
use indexmap::IndexMap;

use crate::text::observation::shard::{change::Change, reference::Reference};

/// A fragment of a view's complete history.
pub struct Shard {
    head: Hash,
    changes: IndexMap<Hash, Change>,
    references: Vec<Reference>
}

impl Shard {
    /// Returns a copy of the contained head.
    pub fn head(&self) -> Hash {
        self.head
    }
    
    /// Returns a reference to the contained changes.
    pub fn changes(&self) -> &IndexMap<Hash, Change> {
        &self.changes
    }

    /// Returns a mutable reference to the contained changes.
    pub(crate) fn changes_mut(&mut self) -> &mut IndexMap<Hash, Change> {
        &mut self.changes
    }

    /// Returns a reference to the contained references.
    pub fn references(&self) -> &[Reference] {
        &self.references
    }
}