use crate::{Shard, ShardKey};

// Recursively remove all of the parent's descendants
pub(crate) fn descendants(
    parent: ShardKey,
    shard: &mut Shard
) {
    let mut removed = Vec::new();
    descendants_internal(parent, &mut removed, shard);

    // If the head got truncated, set the head to the target
    if shard.changes.get(shard.head).is_none() {
        shard.head = parent;
    }
}

pub(super) fn descendants_internal(
    parent: ShardKey,
    removed: &mut Vec<ShardKey>,
    shard: &mut Shard
) {
    // We've already removed this
    if removed.contains(&parent) {
        return;
    }

    // Add it to the list
    removed.push(parent);

    // Remove the children for this descendant
    let children = shard.children.remove(parent).unwrap();
    
    // Recursively remove all descendants
    for child in children {
        descendants_internal(child, removed, shard);
    }

    // Remove this descendant (from all maps)
    shard.changes.remove(parent).unwrap();
    shard.parents.remove(parent).unwrap();
}