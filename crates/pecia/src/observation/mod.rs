pub mod buffers;
pub mod resource;
pub mod scope;
pub mod shard;

use indexmap::IndexMap;

use crate::observation::{buffers::Buffers, scope::Scope, shard::Shard};

/// Entities that contributes to the inspection of text.
pub struct Observation {
    buffers: Buffers,
    counter: usize,
    shards: IndexMap<Scope, Shard>
}

impl Observation {
    /// Creates an `Observation`.
    pub fn new(buffers: Buffers) -> Self {
        Self {
            buffers,
            counter: 0,
            shards: IndexMap::new()
        }
    }
    
    /// Returns a reference to the contained `Buffers`.
    pub fn buffers(&self) -> &Buffers {
        &self.buffers
    }

    pub(crate) fn buffers_mut(&mut self) -> &mut Buffers {
        &mut self.buffers
    }

    /// Returns a copy to the contained counter.
    pub fn counter(&self) -> usize {
        self.counter
    }

    pub(crate) fn counter_mut(&mut self) -> &mut usize {
        &mut self.counter
    }

    /// Returns a reference to the contained shards.
    pub fn shards(&self) -> &IndexMap<Scope, Shard> {
        &self.shards
    }

    pub(crate) fn shards_mut(&mut self) -> &mut IndexMap<Scope, Shard> {
        &mut self.shards
    }
}