use crate::language::{comprehension::spec::ComprehensionSpec, transformation::spec::TransformationSpec, traversal::spec::TraversalSpec};

pub trait LanguageKind: ComprehensionSpec + TransformationSpec + TraversalSpec {}