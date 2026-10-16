mod helpers;

mod relation;
pub use relation::Relation;
use slotmap::Key;

#[cfg(test)]
mod tests;

use crate::{Shard, ShardError, ShardKey, operations::truncate::helpers::{ancestors::ancestors, descendants::descendants}};

impl Shard {
    /// Modifies the shard's change tree.
    pub fn truncate(
        &mut self,
        pivot: ShardKey,
        relation: Relation
    ) -> Result<(), ShardError> {
        // Make sure the key hasn't already been removed
        self.changes.contains_key(pivot).ok_or(ShardError::InvalidKey)?;
        
        match relation {
            Relation::All => {
                ancestors(pivot, self);
                descendants(pivot, self);
            },
            Relation::Ancestors => ancestors(pivot, self),
            Relation::Descendants => descendants(pivot, self)
        }

        if !self.changes.contains_key(self.head) {
            self.head = self.changes.keys().next().unwrap_or(ShardKey::null());
        }

        Ok(())
    }
}