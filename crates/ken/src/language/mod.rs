pub mod comprehension;
pub mod transformation;
pub mod traversal;

use languages::traits::Language as LanguageKind;

use crate::language::{comprehension::Comprehension, transformation::Transformation, traversal::Traversal};

/// Entities that contribute to the use of a language.
pub struct Language<L: LanguageKind> {
    comprehension: Comprehension<L>,
    transformation: Transformation<L>,
    traversal: Traversal<L>
}

impl<L: LanguageKind> Language<L> {
    pub fn comprehension(&self) -> &Comprehension<L> {
        &self.comprehension
    }

    pub fn transformation(&self) -> &Transformation<L> {
        &self.transformation
    }

    pub fn traversal(&self) -> &Traversal<L> {
        &self.traversal
    }
}