use crate::{Shard, ShardKey, operations::truncate::helpers::descendants::descendants_internal};

// Recursively remove all of the child's ancestors
pub(crate) fn ancestors(
    child: ShardKey,
    shard: &mut Shard
) {
    let mut removed = Vec::new();
    ancestors_internal(child, &mut removed, shard);
}

fn ancestors_internal(
    child: ShardKey,
    removed: &mut Vec<ShardKey>,
    shard: &mut Shard
) {
    // We've already removed this
    if removed.contains(&child) {
        return;
    }
    
    // Add it to the list
    removed.push(child);

    // Remove the parents for this ancestor
    let parents = shard.parents.remove(child).unwrap();
    
    // Recursively remove all ancestors (and descendants of those ancestors)
    for parent in parents {
        ancestors_internal(parent, removed, shard);
        descendants_internal(parent, removed, shard);
    }

    // Remove this ancestor from its childrens' maps
    let children = shard.children.remove(child).unwrap();
    for inner in children {
        // If it is still in the map, remove the ancestor
        if let Some(parents) = shard.parents.get_mut(inner) {
            let index = parents.iter().position(|parent| parent == &child).unwrap();
            parents.swap_remove(index);
        }
    }

    // Remove this ancestor
    shard.changes.remove(child).unwrap();
}