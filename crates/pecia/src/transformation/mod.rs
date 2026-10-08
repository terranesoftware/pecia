mod close;
mod edit;
mod open;
mod point;
mod truncate;
pub use truncate::Relation;

/// The entity that contributes to the manipulation of text.
#[derive(Clone, Copy)]
pub struct Transformation;