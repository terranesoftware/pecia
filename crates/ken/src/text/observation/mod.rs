pub mod buffer;
pub mod scope;
pub mod shard;

use indexmap::IndexMap;

use crate::text::observation::{buffer::Buffer, scope::Scope, shard::Shard};

/// Entities that contributes to the inspection of text.
pub struct Observation {
    buffers: Vec<Buffer>,
    shards: IndexMap<Scope, Shard>
}

impl Observation {
    pub fn buffers(&self) -> &[Buffer] {
        &self.buffers
    }

    pub fn shards(&self) -> &IndexMap<Scope, Shard> {
        &self.shards
    }
}