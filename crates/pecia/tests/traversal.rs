use blake3::Hash;
use pecia::{Delta, Observation, Traversal, observation::Scope, traversal::direction::Direction};

use crate::observation;

#[test]
fn test_route() {
    let observation = observation();

    // route(deltas, destination, direction, observation);
}

fn route(
    deltas: &mut Vec<Delta>,
    destination: (&Scope, &Hash),
    direction: Direction,
    observation: &Observation
) {
    Traversal::route(deltas, destination, direction, observation);
}