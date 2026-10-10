pub mod change;
pub use change::Change;

mod operations;

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
    references: HashMap<Hash, Reference>
}

impl Shard {
    /// Creates a new `Shard`.
    pub fn new() -> Self {
        Self {
            head: Hash::from_bytes([0; 32]),
            changes: HashMap::new(),
            children: HashMap::new(),
            references: HashMap::new()
        }
    }
}