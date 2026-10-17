use std::collections::HashMap;

use slotmap::Key;

use crate::{Shard, ShardKey, operations::{relocate::Operation, traverse::Direction, truncate::Relation}};

#[test]
fn truncate() {
    let (a, b, c, d, shard) = init();

    // Ancestors
    ancestors_top(a, b, c, d, shard.clone());
    ancestors_middle(b, c, d, shard.clone());
    ancestors_bottom(d, shard.clone());
    
    // Descendants
    descendants_top(a, shard.clone());
    descendants_middle(a, b, c, shard.clone());
    descendants_bottom(a, b, c, d, shard);
}

fn init() -> (
    ShardKey,
    ShardKey,
    ShardKey,
    ShardKey,
    Shard
) {
    // Change tree:
    //    A
    //  /  \  ↑ Ancestors
    // B    C
    //  \  /  ↓ Descendants
    //   D
    
    let mut shard = Shard::new();
        
    shard.change(Vec::new(), Some("A"));
    let a = shard.head;

    shard.change(Vec::new(), Some("B"));
    let b = shard.head;

    // Go back to A to create C
    shard.traverse(&mut Vec::new(), a, Direction::Older).unwrap();
    shard.change(Vec::new(), Some("C"));
    let c = shard.head;

    // Create D and then copy it over to B
    shard.change(Vec::new(), Some("D"));
    let d = shard.head;
    shard.relocate(&[b], Operation::Copy, d).unwrap();

    (a, b, c, d, shard)
}

fn ancestors_top(
    a: ShardKey,
    b: ShardKey,
    c: ShardKey,
    d: ShardKey,
    mut shard: Shard
) {
    // Truncate from the top
    shard.truncate(a, Relation::Ancestors).unwrap();

    assert_eq!(shard.head, d);
    assert_eq!(
        shard.changes.keys().collect::<Vec<_>>(),
        vec![b, c, d]
    );
    assert_eq!(
        shard.parents.iter().collect::<HashMap<_, _>>(),
        HashMap::from([
            (d, &vec![c, b]),
            (c, &vec![]),
            (b, &vec![])
        ])
    );
    assert_eq!(
        shard.children.iter().collect::<HashMap<_, _>>(),
        HashMap::from([
            (d, &vec![]),
            (c, &vec![d]),
            (b, &vec![d])
        ])
    );
}

fn ancestors_middle(
    b: ShardKey,
    c: ShardKey,
    d: ShardKey,
    mut shard: Shard
) {
    // Truncate from the middle
    shard.truncate(b, Relation::Ancestors).unwrap();

    assert_eq!(shard.head, d);
    assert_eq!(
        shard.changes.keys().collect::<Vec<_>>(),
        vec![c, d]
    );
    assert_eq!(
        shard.parents.iter().collect::<HashMap<_, _>>(),
        HashMap::from([
            (c, &vec![]),
            (d, &vec![c])
        ])
    );
    assert_eq!(
        shard.children.iter().collect::<HashMap<_, _>>(),
        HashMap::from([
            (c, &vec![d]),
            (d, &vec![])
        ])
    );
}

fn ancestors_bottom(
    d: ShardKey,
    mut shard: Shard
) {
    // Truncate from the bottom
    shard.truncate(d, Relation::Ancestors).unwrap();

    assert_eq!(shard.head, ShardKey::null());
    assert_eq!(
        shard.changes.iter().collect::<HashMap<_, _>>(),
        HashMap::from([])
    );
    assert_eq!(
        shard.parents.iter().collect::<HashMap<_, _>>(),
        HashMap::from([])
    );
    assert_eq!(
        shard.children.iter().collect::<HashMap<_, _>>(),
        HashMap::from([])
    );
}

fn descendants_top(
    a: ShardKey,
    mut shard: Shard
) {
    // Truncate from the top
    shard.truncate(a, Relation::Descendants).unwrap();

    assert_eq!(shard.head, ShardKey::null());
    assert_eq!(
        shard.changes.iter().collect::<HashMap<_, _>>(),
        HashMap::from([])
    );
    assert_eq!(
        shard.parents.iter().collect::<HashMap<_, _>>(),
        HashMap::from([])
    );
    assert_eq!(
        shard.children.iter().collect::<HashMap<_, _>>(),
        HashMap::from([])
    );
}

fn descendants_middle(
    a: ShardKey,
    b: ShardKey,
    c: ShardKey,
    mut shard: Shard
) {
    // Truncate from the middle
    shard.truncate(c, Relation::Descendants).unwrap();

    assert_eq!(shard.head, a);
    assert_eq!(
        shard.changes.keys().collect::<Vec<_>>(),
        vec![a, b]
    );
    assert_eq!(
        shard.parents.iter().collect::<HashMap<_, _>>(),
        HashMap::from([
            (a, &vec![]),
            (b, &vec![a])
        ])
    );
    assert_eq!(
        shard.children.iter().collect::<HashMap<_, _>>(),
        HashMap::from([
            (a, &vec![b]),
            (b, &vec![])
        ])
    );
}

fn descendants_bottom(
    a: ShardKey,
    b: ShardKey,
    c: ShardKey,
    d: ShardKey,
    mut shard: Shard
) {
    // Truncate from the bottom
    shard.truncate(d, Relation::Descendants).unwrap();
    
    assert_eq!(shard.head, a);
    assert_eq!(
        shard.changes.keys().collect::<Vec<_>>(),
        vec![a, b, c]
    );
    assert_eq!(
        shard.parents.iter().collect::<HashMap<_, _>>(),
        HashMap::from([
            (a, &vec![]),
            (b, &vec![a]),
            (c, &vec![a])
        ])
    );
    assert_eq!(
        shard.children.iter().collect::<HashMap<_, _>>(),
        HashMap::from([
            (a, &vec![b, c]),
            (b, &vec![]),
            (c, &vec![])
        ])
    );
}