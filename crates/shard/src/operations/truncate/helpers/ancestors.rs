use blake3::Hash;

use crate::{Shard, ShardError, operations::truncate::helpers::descendants::descendants_internal};

// Recursively remove all of the child's ancestors
pub(crate) fn ancestors(
    child: &Hash,
    shard: &mut Shard,
    start: &Hash
) -> Result<(), ShardError> {
    ancestors_internal(child, shard, start)?;

    // If the head got truncated, set the head to the target
    if shard.changes.get(&shard.head).is_none() {
        shard.head = *child;
    }

    Ok(())
}

fn ancestors_internal(
    child: &Hash,
    shard: &mut Shard,
    start: &Hash
) -> Result<(), ShardError> {
    // Remove the parents for this ancestor
    let parents = shard.parents.remove(child).ok_or(ShardError::InvalidHash)?;
    
    // Recursively remove all ancestors (and descendants of those ancestors)
    for parent in &parents {
        descendants_internal(parent, shard, start)?;
        ancestors_internal(parent, shard, start)?;
    }

    // Remove this ancestor
    if child != start {
        shard.changes.remove(child).unwrap();
    }

    Ok(())
}