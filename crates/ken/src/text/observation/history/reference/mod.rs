pub mod kind;

use blake3::Hash;

use crate::text::observation::history::reference::kind::ReferenceKind;

pub struct Reference {
    kind: ReferenceKind,
    name: String,
    node: Hash
}

impl Reference {
    pub fn kind(&self) -> ReferenceKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn node(&self) -> Hash {
        self.node
    }
}