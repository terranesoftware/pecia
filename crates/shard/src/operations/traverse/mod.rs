mod delta;
pub use delta::Delta;

mod direction;
pub use direction::Direction;

mod helpers;

use blake3::Hash;

use crate::{Shard, ShardError, operations::traverse::helpers::{change, create}};

impl Shard {
    pub fn traverse(
        &mut self,
        deltas: &mut Vec<Delta>,
        destination: &Hash,
        direction: Direction
    ) -> Result<(), ShardError> {
        // Get the destination change
        let change = change(destination, direction, self)?;
        
        // Loop through edits and add to/modify deltas
        for edit in &change.edits {
            create(deltas, edit);
        }

        // TODO: Coalesce diffs within each resource
        // Deferred due to it being an optimization, not a necessity

        // Update the head
        self.head = *destination;

        Ok(())
    }
}