#[cfg(test)]
mod tests;

use crate::{Reference, Shard, ShardError, ShardKey, reference::ReferenceKind};

impl Shard {
    /// Tags a `Change` with a `Reference`.
    pub fn reference(
        &mut self,
        kind: ReferenceKind,
        name: String,
        change: ShardKey
    ) -> Result<(), ShardError> {
        // Make sure the key is valid
        if !self.changes.contains_key(change) {
            return Err(ShardError::InvalidKey);
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