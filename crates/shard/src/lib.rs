pub mod change;
pub use change::Change;

mod error;
pub use error::ShardError;

pub mod operations;

pub mod reference;
pub use reference::Reference;

use blake3::Hash;
use std::collections::HashMap;

/// A fragment of a view's complete history.
#[derive(Clone)]
pub struct Shard {
    head: Hash,
    changes: HashMap<Hash, Change>,
    children: HashMap<Hash, Vec<Hash>>,
    parents: HashMap<Hash, Vec<Hash>>,
    references: HashMap<Reference, Hash>
}

impl Shard {
    /// Creates a new `Shard`.
    pub fn new() -> Self {
        Self {
            head: Hash::from_bytes([0; 32]),
            changes: HashMap::new(),
            children: HashMap::new(),
            parents: HashMap::new(),
            references: HashMap::new()
        }
    }

    /// Returns a copy of the contained head hash.
    pub fn head(&self) -> Hash {
        self.head
    }

    /// Returns a reference to the contained changes map.
    pub fn changes(&self) -> &HashMap<Hash, Change> {
        &self.changes
    }

    /// Returns a reference to the contained children map.
    pub fn children(&self) -> &HashMap<Hash, Vec<Hash>> {
        &self.children
    }

    /// Returns a reference to the contained parents map.
    pub fn parents(&self) -> &HashMap<Hash, Vec<Hash>> {
        &self.parents
    }

    /// Returns a reference to the contained references map.
    pub fn references(&self) -> &HashMap<Reference, Hash> {
        &self.references
    }
}