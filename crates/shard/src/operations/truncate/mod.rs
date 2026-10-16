mod helpers;

mod relation;
pub use relation::Relation;

#[cfg(test)]
mod tests;

use crate::{Shard, ShardError, ShardKey, operations::truncate::helpers::{ancestors::ancestors, descendants::descendants}};

impl Shard {
    /// Modifies the shard's change tree.
    pub fn truncate(
        &mut self,
        relation: Relation,
        pivot: ShardKey
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

        Ok(())
    }
}