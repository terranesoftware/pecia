use crate::{observation::Observation, transformation::Transformation};

impl Transformation {
    pub fn squash(observation: &mut Observation) {
        for (_, shard) in observation.shards_mut() {
            let head = shard.head();
            shard.changes_mut().retain(|key, _| *key == head);
        }
    }
}