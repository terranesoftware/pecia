use crate::{Shard, ShardKey};

// Recursively remove all of the parent's descendants
pub(crate) fn descendants(
    parent: ShardKey,
    shard: &mut Shard
) {
    let mut removed = Vec::new();
    descendants_internal(parent, &mut removed, shard);
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

    // Remove this descendant from its parents' maps
    let parents = shard.parents.remove(parent).unwrap();
    for inner in parents {
        // If it is still in the map, remove the descendant
        if let Some(children) = shard.children.get_mut(inner) {
            let index = children.iter().position(|child| child == &parent).unwrap();
            children.swap_remove(index);
        }
    }

    // Remove this descendant
    shard.changes.remove(parent).unwrap();
}