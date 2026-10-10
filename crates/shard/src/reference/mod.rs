mod kind;
pub use kind::ReferenceKind;

/// A label given to a `Change`.
#[derive(Clone)]
pub struct Reference {
    pub(crate) kind: ReferenceKind,
    pub(crate) name: String
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
}