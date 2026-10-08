use std::path::PathBuf;

use blake3::Hash;
use pecia::{Observation, Transformation, observation::{Resource, Scope}, transformation::Relation};

use crate::observation;

#[test]
fn test_truncate() {
    let mut observation = observation();

    // Placeholder for now
    let scope = Scope::Directory(Resource::Directory(PathBuf::from("/")));
    let hash = Hash::from([0; 32]);
    let target = (&scope, &hash);
    
    // One for each relation variant
    truncate(&mut observation, Relation::All, target);
    truncate(&mut observation, Relation::Ancestors, target);
    truncate(&mut observation, Relation::Descendants, target);
}

fn truncate(
    observation: &mut Observation,
    relation: Relation,
    target: (&Scope, &Hash)
) {
    Transformation::truncate(observation, relation, target);
}