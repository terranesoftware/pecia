use crate::language::{comprehension::Comprehension, transformation::Transformation, traversal::Traversal};

pub mod comprehension;
pub mod transformation;
pub mod traversal;

pub struct Language {
    comprehension: Comprehension,
    transformation: Transformation,
    traversal: Traversal
}