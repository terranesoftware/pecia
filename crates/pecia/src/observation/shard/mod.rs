pub mod change;
pub mod reference;

use blake3::Hash;
use indexmap::IndexMap;

use crate::observation::shard::{change::Change, reference::Reference};

/// A fragment of a view's complete history.
pub struct Shard {
    head: Hash,
    changes: IndexMap<Hash, Change>,
    children: IndexMap<Hash, Vec<Hash>>,
    references: Vec<Reference>
}

impl Shard {
    /// Returns a copy of the contained head.
    pub fn head(&self) -> Hash {
        self.head
    }

    /// Returns a mutable reference to the contained head.
    pub(crate) fn head_mut(&mut self) -> &mut Hash {
        &mut self.head
    }
    
    /// Returns a reference to the contained changes.
    pub fn changes(&self) -> &IndexMap<Hash, Change> {
        &self.changes
    }
    
    /// Returns a mutable reference to the contained changes.
    pub(crate) fn changes_mut(&mut self) -> &mut IndexMap<Hash, Change> {
        &mut self.changes
    }

    /// Returns a reference to the contained children.
    pub fn children(&self) -> &IndexMap<Hash, Vec<Hash>> {
        &self.children
    }
    
    /// Returns a mutable reference to the contained children.
    pub(crate) fn children_mut(&mut self) -> &mut IndexMap<Hash, Vec<Hash>> {
        &mut self.children
    }

    /// Returns a reference to the contained references.
    pub fn references(&self) -> &[Reference] {
        &self.references
    }
}