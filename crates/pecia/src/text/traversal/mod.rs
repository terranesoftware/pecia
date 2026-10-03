pub mod delta;

use blake3::Hash;

use crate::text::observation::{Observation, scope::Scope};

/// The entity that contribute to the navigation of text.
pub struct Traversal;

impl Traversal {
    pub fn route(
        destination: (Scope, Hash),
        observation: &Observation,
        origin: (Scope, Hash)
    ) {
        let shards = observation.shards();
        let origin = shards.get(&origin.0).expect("Shard doesn't exist").changes().get(&origin.1).expect("Change doesn't exist");
        let destination = shards.get(&destination.0).expect("Shard doesn't exist").changes().get(&destination.1).expect("Change doesn't exist");
    }
}