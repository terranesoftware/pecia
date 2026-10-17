mod helpers;

mod operation;
pub use operation::Operation;

#[cfg(test)]
mod tests;

use crate::{Shard, ShardError, ShardKey, operations::relocate::helpers::check};

impl Shard {
    /// Applies an `Operation` to a `Change` (and to its descendants, transitively)
    pub fn relocate(
        &mut self,
        destinations: &[ShardKey],
        operation: Operation,
        source: ShardKey
    ) -> Result<(), ShardError> {
        // Check for key validity and cycles
        check(destinations, self, source)?;

        // For every destination hash, push on the source as a child
        for parent in destinations {
            self.children.get_mut(*parent).unwrap().push(source);
        }

        // Match on the operation
        match operation {
            Operation::Copy => self.parents.get_mut(source).unwrap().extend(destinations), // Just add all the destination hashes as new parents
            Operation::Cut => {
                // Get the parents of this source as well as replacing it with its new parents
                let parents = self.parents.insert(source, destinations.to_vec()).unwrap();

                // For every parent, get its children and remove the source from it
                for parent in parents {
                    self.children.get_mut(parent).unwrap().extract_if(.., |hash| *hash == source).next().unwrap();
                }
            }
        }

        Ok(())
    }
}