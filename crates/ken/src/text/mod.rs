use crate::text::{observation::Observation, transformation::Transformation, traversal::Traversal};

pub mod observation;
pub mod transformation;
pub mod traversal;

/// Entities that contribute to the use of text.
pub struct Text {
    observation: Observation,
    transformation: Transformation,
    traversal: Traversal
}

impl Text {
    pub fn observation(&self) -> &Observation {
        &self.observation
    }

    pub fn transformation(&self) -> &Transformation {
        &self.transformation
    }

    pub fn traversal(&self) -> &Traversal {
        &self.traversal
    }
}