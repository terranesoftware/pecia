mod kind;
pub use kind::ReferenceKind;

use blake3::Hash;

/// A label given to a `Change`.
pub struct Reference {
    pub kind: ReferenceKind,
    pub name: String,
    pub change: Hash
}