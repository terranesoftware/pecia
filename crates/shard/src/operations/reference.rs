use blake3::Hash;

use crate::{Reference, Shard, reference::ReferenceKind};

impl Shard {
    /// Tags a `Change` with a `Reference`.
    pub fn reference(
        &mut self,
        kind: ReferenceKind,
        name: String,
        change: Hash
    ) {
        // Create the reference
        let reference = Reference {
            kind,
            name
        };

        // Add it to the map
        self.references.insert(change, reference);
    }
}