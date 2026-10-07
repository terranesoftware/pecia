mod helpers;

mod relativity;
pub use relativity::Relativity;

use blake3::Hash;

use crate::{observation::{Observation, scope::Scope}, transformation::{Transformation, truncate::helpers::{ancestors::ancestors, both::both, descendants::descendants}}};

impl Transformation {
    pub fn truncate(
        observation: &mut Observation,
        target: (&Scope, &Hash),
        relativity: Relativity
    ) {
        match relativity {
            Relativity::Ancestors => ancestors(observation, target),
            Relativity::Both => both(observation, target),
            Relativity::Descendants => descendants(observation, target)
        }
    }
}