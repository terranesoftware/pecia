pub mod buffers;
pub mod resource;
pub mod scope;
pub mod shard;

use indexmap::IndexMap;

use crate::text::observation::{buffers::Buffers, scope::Scope, shard::Shard};

/// Entities that contributes to the inspection of text.
pub struct Observation {
    buffers: Buffers,
    shards: IndexMap<Scope, Shard>
}

impl Observation {
    /// Returns a reference to the contained `Buffers`.
    pub fn buffers(&self) -> &Buffers {
        &self.buffers
    }

    pub fn buffers_mut(&mut self) -> &mut Buffers {
        &mut self.buffers
    }

    /// Returns a reference to the contained shards.
    pub fn shards(&self) -> &IndexMap<Scope, Shard> {
        &self.shards
    }
}