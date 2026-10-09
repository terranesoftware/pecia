use blake3::Hash;
use pecia::{Observation, Transformation, observation::Scope, transformation::Relation};

pub(crate) fn truncate(
    observation: &mut Observation,
    relation: Relation,
    target: (&Scope, &Hash)
) {
    Transformation::truncate(observation, relation, target);
}