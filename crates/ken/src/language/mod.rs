pub mod comprehension;
pub mod transformation;
pub mod traversal;

use crate::language::{comprehension::Comprehension, transformation::Transformation, traversal::Traversal};

/// Entities that contribute to the use of a language.
pub struct Language {
    comprehension: Comprehension,
    transformation: Transformation,
    traversal: Traversal
}