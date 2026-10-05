pub mod observation;
pub mod transformation;
pub mod traversal;

use crate::{observation::Observation, transformation::Transformation, traversal::Traversal};

/// Entities that contribute to the use of text.
pub struct Text {
    observation: Observation,
    transformation: Transformation,
    traversal: Traversal
}

impl Text {
    /// Returns a reference to the contained `Observation`.
    pub fn observation(&self) -> &Observation {
        &self.observation
    }

    /// Returns a reference to the contained `Transformation`.
    pub fn transformation(&self) -> &Transformation {
        &self.transformation
    }

    /// Returns a reference to the contained `Traversal`.
    pub fn traversal(&self) -> &Traversal {
        &self.traversal
    }
}