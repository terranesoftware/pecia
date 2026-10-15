use time::Timestamp;

use crate::{Change, Reference, Shard, change::Edit, reference::ReferenceKind};

impl Shard {
    /// Adds a `Change` to the `Shard`.
    pub fn change(
        &mut self,
        edits: Vec<Edit>,
        reference: Option<String>
    ) {
        // Create the change
        let change = Change {
            edits,
            timestamp: Timestamp::now()
        };

        // Add the new change to the tree
        let key = self.changes.insert(change);

        // Add head as its parent
        self.parents.insert(key, vec![self.head]);

        // Add it as the previous head's child
        let children = self.children.get_mut(self.head).unwrap();
        children.push(key);

        // Update the head
        self.head = key;

        // If a reference was wanted, create it
        if let Some(name) = reference {
            let reference = Reference {
                kind: ReferenceKind::Change,
                name
            };

            self.references.insert(reference, key);
        };
    }
}