use blake3::Hash;

use crate::{Shard, ShardError};

// Recursively remove all of the child's ancestors
pub(crate) fn ancestors(
    child: &Hash,
    shard: &mut Shard
) -> Result<(), ShardError> {
    ancestors_internal(child, shard)?;

    // If the head got truncated, set the head to the target
    if shard.changes.get(&shard.head).is_none() {
        shard.head = *child;
    }

    Ok(())
}

fn ancestors_internal(
    child: &Hash,
    shard: &mut Shard
) -> Result<(), ShardError> {
    // Remove the parents for this ancestor
    let parents = shard.parents.remove(child).ok_or(ShardError::InvalidHash)?;
    
    // Recursively remove all ancestors
    for parent in &parents {
        ancestors_internal(parent, shard)?;
    }

    // Remove this ancestor
    shard.changes.remove(child).unwrap();

    Ok(())
}