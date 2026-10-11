use blake3::Hash;

use crate::{Shard, ShardError};

// Recursively remove all of the parent's descendants
pub(crate) fn descendants(
    parent: &Hash,
    shard: &mut Shard
) -> Result<(), ShardError> {
    descendants_internal(parent, shard)?;

    // If the head got truncated, set the head to the target
    if shard.changes.get(&shard.head).is_none() {
        shard.head = *parent;
    }

    Ok(())
}

fn descendants_internal(
    parent: &Hash,
    shard: &mut Shard
) -> Result<(), ShardError> {
    // Remove the children for this descendant
    let children = shard.children.remove(parent).ok_or(ShardError::InvalidHash)?;
    
    // Recursively remove all descendants
    for child in &children {
        descendants_internal(child, shard)?;
    }

    // Remove this descendant
    shard.changes.remove(parent).unwrap();

    Ok(())
}