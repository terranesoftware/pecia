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

impl<L: LanguageKind> Language<L> {
    pub fn comprehension(&self) -> &Comprehension<L> {
        &self.comprehension
    }

    pub fn comprehension_mut(&mut self) -> &mut Comprehension<L> {
        &mut self.comprehension
    }

    pub fn transformation(&self) -> &Transformation<L> {
        &self.transformation
    }

    pub fn transformation_mut(&mut self) -> &mut Transformation<L> {
        &mut self.transformation
    }

    pub fn traversal(&self) -> &Traversal<L> {
        &self.traversal
    }

    pub fn traversal_mut(&mut self) -> &mut Traversal<L> {
        &mut self.traversal
    }
}