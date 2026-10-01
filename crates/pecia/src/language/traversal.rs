use languages::traits::traversal::Traversal as TraversalSpec;

/// Entities that contribute to the navigation of a language.
pub struct Traversal<T: TraversalSpec> {
    grammar: T::Grammar,
    indexes: Vec<T::Index>
}

impl<T: TraversalSpec> Traversal<T> {
    pub fn grammar(&self) -> &T::Grammar {
        &self.grammar
    }

    pub fn indexes(&self) -> &[T::Index] {
        &self.indexes
    }
}