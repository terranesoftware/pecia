use std::collections::HashSet;

use crate::{Shard, ShardError, ShardKey};

pub(crate) fn check(
    destinations: &[ShardKey],
    shard: &Shard,
    source: ShardKey
) -> Result<(), ShardError> {
    // Make sure hashes are valid
    if !shard.changes.contains_key(source) || !destinations.iter().all(|hash| shard.changes.contains_key(*hash)) {
        return Err(ShardError::InvalidKey);
    }

    // Check for cycles
    let mut stack = vec![source];
    let mut visited = HashSet::new();
    
    while let Some(change) = stack.pop() {
        // Already been here
        if !visited.insert(change) {
            continue;
        }
        
        // Reachable, error
        if destinations.contains(&change) {
            return Err(ShardError::Cycle);
        }
        
        // Extend with descendants
        stack.extend(shard.children.get(change).unwrap());
    }

    Ok(())
}