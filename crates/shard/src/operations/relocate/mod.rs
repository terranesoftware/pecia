mod operation;
pub use operation::Operation;

use blake3::Hash;

use crate::{Shard, ShardError};

impl Shard {
    pub fn relocate(
        &mut self,
        destination: Vec<Hash>,
        operation: Operation,
        source: &Hash
    ) -> Result<(), ShardError> {
        // Make sure hashes are valid
        if !self.changes.contains_key(source) || !destination.iter().all(|hash| self.changes.contains_key(hash)) {
            return Err(ShardError::InvalidHash);
        }

        // For every destination hash, push on the source as a child
        for parent in &destination {
            self.children.get_mut(&parent).unwrap().push(*source);
        }

        // Match on the operation
        match operation {
            Operation::Copy => self.parents.get_mut(source).unwrap().extend(destination), // Just add all the destination hashes as new parents
            Operation::Cut => {
                // Get the parents of this source as well as replacing it with its new parents
                let parents = self.parents.insert(*source, destination).unwrap();

                // For every parent, get its children and remove the source from it
                for parent in parents {
                    self.children.get_mut(&parent).unwrap().extract_if(.., |hash| hash == source).next().unwrap();
                }
            }
        }

        Ok(())
    }
}