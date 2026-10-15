use slotmap::Key;

use crate::{Shard, ShardKey};

#[test]
fn change() {
    let mut shard = Shard::new();

    shard.change(Vec::new(), None);
    let first = shard.head;
    shard.change(Vec::new(), Some("Second"));
    let second = shard.head;

    // Make sure neither of the keys are null
    assert!(first != ShardKey::null());
    assert!(second != ShardKey::null());

    // Make sure the parent/child relationship is there
    assert_eq!(
        shard.children.get(first).unwrap(),
        &[second]
    );
    assert_eq!(
        shard.parents.get(first).unwrap(),
        &[]
    );
    assert_eq!(
        shard.children.get(second).unwrap(),
        &[]
    );
    assert_eq!(
        shard.parents.get(second).unwrap(),
        &[first]
    );
}