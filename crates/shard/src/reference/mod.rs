mod kind;
pub use kind::ReferenceKind;

use blake3::Hash;

/// A label given to a `Change`.
#[derive(Clone)]
pub struct Reference {
    pub kind: ReferenceKind,
    pub name: String,
    pub change: Hash
}