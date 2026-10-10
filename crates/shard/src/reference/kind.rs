/// The kinds of possible referents.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ReferenceKind {
    Branch,
    Change
}