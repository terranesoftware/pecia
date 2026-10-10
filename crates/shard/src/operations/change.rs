use bitcode::serialize;
use blake3::hash;
use time::Timestamp;

use crate::{Change, Reference, Shard, change::Edit, reference::ReferenceKind};

impl Shard {
    pub fn change(
        &mut self,
        edits: Vec<Edit>,
        reference: Option<String>
    ) {
        // Create the change
        let change = Change {
            edits,
            parents: vec![self.head],
            timestamp: Timestamp::now()
        };

        // Hash it
        let hash = hash(&serialize(&change).unwrap());

        // Add it as the previous head's child
        let children = self.children.get_mut(&self.head).unwrap();
        children.push(hash);
        
        // Add the new change to the tree
        self.changes.insert(hash, change);

        // Update the head
        self.head = hash;

        // If a reference was wanted, create it
        if let Some(name) = reference {
            let reference = Reference {
                kind: ReferenceKind::Change,
                name,
                change: hash
            };

            self.references.push(reference);
        };
    }
}