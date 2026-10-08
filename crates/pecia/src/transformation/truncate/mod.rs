mod helpers;
mod relation;
pub use relation::Relation;

use blake3::Hash;

use crate::{observation::{Observation, Scope}, transformation::{Transformation, truncate::helpers::{ancestors::ancestors, descendants::descendants}}};

impl Transformation {
    // Might change this doc comment
    /// Modifies the shard's change tree.
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