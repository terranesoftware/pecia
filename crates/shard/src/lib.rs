pub mod change;
pub use change::Change;

mod error;
pub use error::ShardError;

pub mod operations;

pub mod reference;
pub use reference::Reference;

use slotmap::{Key, SecondaryMap, SlotMap, new_key_type};
use std::collections::HashMap;

new_key_type! { pub struct ShardKey; }

/// A fragment of a view's complete history.
#[derive(Clone)]
pub struct Shard {
    head: ShardKey,
    changes: SlotMap<ShardKey, Change>,
    children: SecondaryMap<ShardKey, Vec<ShardKey>>,
    parents: SecondaryMap<ShardKey, Vec<ShardKey>>,
    references: HashMap<Reference, ShardKey>
}

impl Shard {
    /// Creates a new `Shard`.
    pub fn new() -> Self {
        Self {
            head: ShardKey::null(),
            changes: SlotMap::with_key(),
            children: SecondaryMap::new(),
            parents: SecondaryMap::new(),
            references: HashMap::new()
        }
    }

    /// Returns a copy of the contained head key.
    pub fn head(&self) -> ShardKey {
        self.head
    }

    /// Returns a reference to the contained changes map.
    pub fn changes(&self) -> &SlotMap<ShardKey, Change> {
        &self.changes
    }

    /// Returns a reference to the contained children map.
    pub fn children(&self) -> &SecondaryMap<ShardKey, Vec<ShardKey>> {
        &self.children
    }

    /// Returns a reference to the contained parents map.
    pub fn parents(&self) -> &SecondaryMap<ShardKey, Vec<ShardKey>> {
        &self.parents
    }

    /// Returns a reference to the contained references map.
    pub fn references(&self) -> &HashMap<Reference, ShardKey> {
        &self.references
    }
}