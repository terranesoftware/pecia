use blake3::Hash;

use crate::text::{observation::{Observation, scope::Scope, shard::change::Change}, traversal::direction::Direction};

pub(crate) fn change(
    destination: (Scope, Hash),
    direction: Direction,
    observation: &Observation
) -> &Change {
    let shard = observation.shards().get(&destination.0).expect("Shard doesn't exist");
    let head = shard.head();
    
    let relatives = match direction {
        Direction::Down => shard.changes().get(&head).unwrap().parents(),
        Direction::Up => shard.children().get(&head).unwrap()
    };
    let destination = relatives.iter().find(|hash| **hash == destination.1).unwrap();
    
    shard.changes().get(destination).unwrap()
}