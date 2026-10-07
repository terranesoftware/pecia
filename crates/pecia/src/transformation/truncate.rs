use blake3::Hash;

use crate::{observation::{Observation, scope::Scope, shard::Shard}, transformation::Transformation};

impl Transformation {
    pub fn truncate(
        observation: &mut Observation,
        parent: (&Scope, &Hash)
    ) {
        let shard = observation.shards_mut().get_mut(parent.0).expect("Shard doesn't exist");

        truncate_internal(parent.1, shard);
    }
}

// Recursively remove al of the parent's descendants
pub(super) fn truncate_internal(
    parent: &Hash,
    shard: &mut Shard
) {
    let children = shard.children_mut().swap_remove(parent).expect("Change doesn't exist");
    
    for child in &children {
        truncate_internal(child, shard);
    }

    shard.changes_mut().swap_remove(parent).unwrap();
}