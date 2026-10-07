use blake3::Hash;

use crate::observation::{Observation, scope::Scope};

pub(crate) fn both(
    observation: &mut Observation,
    target: (&Scope, &Hash)
) {
    for (_, shard) in observation.shards_mut() {
        let head = shard.head();
        shard.changes_mut().retain(|key, _| *key == head);
    }
}