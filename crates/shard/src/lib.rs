pub mod change;
pub use change::Change;

pub mod reference;
pub use reference::Reference;

use blake3::Hash;
use indexmap::IndexMap;

/// A fragment of a view's complete history.
#[derive(Clone)]
pub struct Shard {
    head: Hash,
    changes: IndexMap<Hash, Change>,
    children: IndexMap<Hash, Vec<Hash>>,
    references: Vec<Reference>
}

impl Shard {
    /// Creates a new `Shard`.
    pub fn new() -> Self {
        Self {
            head: Hash::from_bytes([0; 32]),
            changes: IndexMap::new(),
            children: IndexMap::new(),
            references: Vec::new()
        }
    }
}