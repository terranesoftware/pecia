use crate::{Shard, ShardError, ShardKey};

// Recursively remove all of the parent's descendants
pub(crate) fn descendants(
    parent: ShardKey,
    shard: &mut Shard,
    start: ShardKey
) -> Result<(), ShardError> {
    descendants_internal(parent, shard, start)?;

    // If the head got truncated, set the head to the target
    if shard.changes.get(shard.head).is_none() {
        shard.head = parent;
    }

    Ok(())
}

pub(super) fn descendants_internal(
    parent: ShardKey,
    shard: &mut Shard,
    start: ShardKey
) -> Result<(), ShardError> {
    // Remove the children for this descendant
    let children = shard.children.remove(parent).ok_or(ShardError::InvalidHash)?;
    
    // Recursively remove all descendants
    for child in children {
        descendants_internal(child, shard, start)?;
    }

    // Remove this descendant
    if parent != start {
        shard.changes.remove(parent).unwrap();
    }

    Ok(())
}