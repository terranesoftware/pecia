mod helpers;

mod relation;
use blake3::Hash;
pub use relation::Relation;

use crate::{observation::{Observation, scope::Scope}, transformation::{Transformation, truncate::helpers::{ancestors::ancestors, descendants::descendants}}};

impl Transformation {
    pub fn truncate(
        observation: &mut Observation,
        relation: Relation,
        target: (&Scope, &Hash)
    ) {
        match relation {
            Relation::All => {
                ancestors(observation, target);
                descendants(observation, target);
            },
            Relation::Ancestors => ancestors(observation, target),
            Relation::Descendants => descendants(observation, target)
        }
    }
}