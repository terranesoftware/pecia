use blake3::Hash;

use crate::observation::shard::Shard;

// Recursively remove al of the parent's descendants
pub(super) fn truncate(
    parent: &Hash,
    shard: &mut Shard
) {
    let children = shard.children_mut().swap_remove(parent).expect("Change doesn't exist");
    
    for child in &children {
        truncate(child, shard);
    }

    shard.changes_mut().swap_remove(parent).unwrap();
}