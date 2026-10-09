mod kind;
pub use kind::ReferenceKind;

use blake3::Hash;

/// A label given to a `Change`.
pub struct Reference {
    kind: ReferenceKind,
    name: String,
    change: Hash
}

impl Reference {
    /// Returns a copy of the contained `ReferenceKind`.
    pub fn kind(&self) -> ReferenceKind {
        self.kind
    }

    /// Returns a reference to the contained name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a copy of the contained change hash.
    pub fn change(&self) -> Hash {
        self.change
    }
}