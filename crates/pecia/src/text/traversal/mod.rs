pub mod delta;

use blake3::Hash;

use crate::text::observation::Observation;

/// The entity that contribute to the navigation of text.
pub struct Traversal;

impl Traversal {
    pub fn route(destination: Hash, observation: &Observation) {
        
    }
}