use std::collections::HashMap;

use crate::{Reference, Shard, reference::ReferenceKind};

#[test]
fn reference() {
    let mut shard = Shard::new();

    // Create a change and reference it *after* creation
    shard.change(Vec::new(), None);
    shard.reference(ReferenceKind::Change, "ReferenceTest", shard.head).unwrap();

    // Make sure the reference exists correctly
    assert_eq!(
        shard.references,
        HashMap::from([
            (
                Reference {
                    kind: ReferenceKind::Change,
                    name: "ReferenceTest".to_string()
                },
                shard.head
            )
        ])
    );
}