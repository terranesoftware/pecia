mod kind;
pub use kind::ReferenceKind;

use blake3::Hash;

/// A label given to a `Change`.
#[derive(Clone)]
pub struct Reference {
    pub(crate) kind: ReferenceKind,
    pub(crate) name: String,
    pub(crate) change: Hash
}

impl Reference {
    pub fn kind(&self) -> ReferenceKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn change(&self) -> Hash {
        self.change
    }
}