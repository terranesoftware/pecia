pub trait TraversalSpec {
    type Grammar;
    type Index;
}

/// Entities that contribute to the navigation of a language.
pub struct Traversal<T: TraversalSpec> {
    grammar: T::Grammar,
    indexes: Vec<T::Index>
}