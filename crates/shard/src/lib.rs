pub mod change;
pub use change::Change;

pub mod reference;
pub use reference::Reference;

use blake3::Hash;
use indexmap::IndexMap;

/// A fragment of a view's complete history.
pub struct Shard {
    pub head: Hash,
    pub changes: IndexMap<Hash, Change>,
    pub children: IndexMap<Hash, Vec<Hash>>,
    pub references: Vec<Reference>
}