pub mod comprehension;
pub mod transformation;
pub mod traversal;

use crate::language::{comprehension::{Comprehension, ComprehensionSpec}, transformation::{Transformation, TransformationSpec}, traversal::{Traversal, TraversalSpec}};

pub trait LanguageKind: ComprehensionSpec + TransformationSpec + TraversalSpec {}

/// Entities that contribute to the use of a language.
pub struct Language<L: LanguageKind> {
    comprehension: Comprehension<L>,
    transformation: Transformation<L>,
    traversal: Traversal<L>
}