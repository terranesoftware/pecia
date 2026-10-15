/// The kinds of possible referents.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReferenceKind {
    Branch,
    Change
}