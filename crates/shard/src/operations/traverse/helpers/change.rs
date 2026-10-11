use blake3::Hash;

use crate::{Change, Shard, ShardError, operations::traverse::Direction};

pub(crate) fn change<'a>(
    destination: &Hash,
    direction: Direction,
    shard: &'a mut Shard
) -> Result<&'a Change, ShardError> {
    // Get the head
    let head = shard.head;
    
    // Get the head's relevant relatives and find the change associated with the destination
    let relatives = match direction {
        Direction::Newer => shard.children.get(&head).unwrap(),
        Direction::Older => &shard.parents.get(&head).unwrap()
    };
    if !relatives.contains(destination) {
        return Err(ShardError::NotARelative);
    }

    // Return a reference to the change
    Ok(shard.changes.get(destination).unwrap())
}