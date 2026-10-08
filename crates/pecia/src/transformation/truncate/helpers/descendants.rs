use blake3::Hash;

use crate::observation::{Observation, scope::Scope, shard::Shard};

// Recursively remove al of the parent's descendants
pub(crate) fn descendants(
    observation: &mut Observation,
    parent: (&Scope, &Hash)
) {
    let shard = observation
        .shards_mut()
        .get_mut(parent.0)
        .expect("Shard doesn't exist");
                    
    descendants_internal(parent.1, shard);

    // If the head got truncated, set the head to the target
    if shard
        .changes()
        .get(&shard.head())
        .is_none()
    {
        *shard.head_mut() = *parent.1;
    }
}

fn descendants_internal(
    parent: &Hash,
    shard: &mut Shard
) {
    let children = shard
        .children_mut()
        .swap_remove(parent)
        .expect("Change doesn't exist");
    
    for child in &children {
        descendants_internal(child, shard);
    }

    shard
        .changes_mut()
        .swap_remove(parent)
        .unwrap();
}