pub mod scope;
pub mod shard;

use indexmap::IndexMap;

use crate::text::observation::history::{scope::Scope, shard::Shard};

pub struct History(IndexMap<Scope, Shard>);

impl History {
    pub fn new() -> Self {
        Self(IndexMap::new())
    }

    pub fn shards(&self) -> &IndexMap<Scope, Shard> {
        &self.0
    }
}