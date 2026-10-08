pub mod buffers;
pub use buffers::{BufferKey, Buffers};

mod resource;
pub use resource::Resource;

mod scope;
pub use scope::Scope;

pub mod shard;

use indexmap::IndexMap;

use crate::observation::shard::Shard;

/// Entities that contributes to the inspection of text.
pub struct Observation {
    buffers: Buffers,
    shards: IndexMap<Scope, Shard>
}

impl Observation {
    /// Creates an `Observation`.
    pub fn new(buffers: Buffers) -> Self {
        Self {
            buffers,
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

    /// Returns a reference to the contained shards.
    pub fn shards(&self) -> &IndexMap<Scope, Shard> {
        &self.shards
    }

    pub(crate) fn shards_mut(&mut self) -> &mut IndexMap<Scope, Shard> {
        &mut self.shards
    }
}