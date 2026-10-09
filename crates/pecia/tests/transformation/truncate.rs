use blake3::Hash;
use pecia::{Observation, Transformation, observation::Scope, transformation::Relation};

use crate::observation;

#[test]
fn test_truncate() {
    let mut observation = observation();

    // One for each relation variant
    /*
    truncate(&mut observation, Relation::All, target);
    truncate(&mut observation, Relation::Ancestors, target);
    truncate(&mut observation, Relation::Descendants, target);
    */
}

fn truncate(
    observation: &mut Observation,
    relation: Relation,
    target: (&Scope, &Hash)
) {
    Transformation::truncate(observation, relation, target);
}