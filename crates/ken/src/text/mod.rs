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