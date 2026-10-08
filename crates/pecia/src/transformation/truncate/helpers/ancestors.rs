use blake3::Hash;

use crate::observation::{Observation, Scope, shard::Shard};

// Recursively remove all of the child's parents
pub(crate) fn ancestors(
    observation: &mut Observation,
    child: (&Scope, &Hash)
) {
    let shard = observation
        .shards_mut()
        .get_mut(child.0)
        .expect("Shard doesn't exist");

    ancestors_internal(child.1, shard);

    // If the head got truncated, set the head to the target
    if shard.changes()
        .get(&shard.head())
        .is_none()
    {
        *shard.head_mut() = *child.1;
    }
}

fn ancestors_internal(
    child: &Hash,
    shard: &mut Shard
) {
    let child = shard
        .changes_mut()
        .swap_remove(child)
        .expect("Change doesn't exist");

    for parent in child.parents() {
        ancestors_internal(parent, shard);
    }
}