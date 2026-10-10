use blake3::Hash;

use crate::{Reference, Shard, ShardError, reference::ReferenceKind};

impl Shard {
    /// Tags a `Change` with a `Reference`.
    pub fn reference(
        &mut self,
        kind: ReferenceKind,
        name: String,
        change: Hash
    ) -> Result<(), ShardError> {
        // Make sure the hash is valid
        if !self.changes.contains_key(&change) {
            return Err(ShardError::InvalidHash);
        }
        
        // Create the reference
        let reference = Reference {
            kind,
            name
        };

        // Add it to the map
        self.references.insert(reference, change);

        Ok(())
    }
}