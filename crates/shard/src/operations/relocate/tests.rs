use crate::{Shard, ShardKey, operations::{relocate::Operation, traverse::Direction}};

#[test]
fn relocate() {
    let (a, b, c, d, mut shard) = init();

    copy(b, c, d, &mut shard);
    cut(a, b, c, d, &mut shard);
}

fn init() -> (
    ShardKey,
    ShardKey,
    ShardKey,
    ShardKey,
    Shard
) {
    // Current DAG:
    //    A
    //  /  \  ↑ Ancestors
    // B    C
    //      | ↓ Descendants
    //      D
    
    let mut shard = Shard::new();
    
    shard.change(Vec::new(), Some("A".to_string()));
    let a = shard.head();
    
    shard.change(Vec::new(), Some("B".to_string()));
    let b = shard.head();
    
    shard.traverse(&mut Vec::new(), a, Direction::Older).unwrap();
    shard.change(Vec::new(), Some("C".to_string()));
    let c = shard.head();

    shard.change(Vec::new(), Some("D".to_string()));
    let d = shard.head();

    (a, b, c, d, shard)
}

fn copy(
    b: ShardKey,
    c: ShardKey,
    d: ShardKey,
    shard: &mut Shard
) {
    // Supposed DAG:
    //    A
    //  /  \  ↑ Ancestors
    // B    C
    //  \  /  ↓ Descendants
    //   D

    shard.relocate(&[b], Operation::Copy, d).unwrap();

    // Check if C still has only D as a child
    assert_eq!(
        shard.children.get(c).unwrap(),
        &[d]
    );

    // Make sure B also has D as a child
    assert_eq!(
        shard.children.get(b).unwrap(),
        &[d]
    );
}

fn cut(
    a: ShardKey,
    b: ShardKey,
    c: ShardKey,
    d: ShardKey,
    shard: &mut Shard
) {
    // Supposed DAG:
    //    A
    //     \    ↑ Ancestors
    //      C
    //     / \  ↓ Descendants
    //   D —— B

    shard.relocate(&[c], Operation::Cut, b).unwrap();

    // A
    assert_eq!(
        shard.children.get(a).unwrap(),
        &[c]
    );
    assert_eq!(
        shard.parents.get(a).unwrap(),
        &[]
    );

    // B
    assert_eq!(
        shard.children.get(b).unwrap(),
        &[d]
    );
    assert_eq!(
        shard.parents.get(b).unwrap(),
        &[c]
    );

    // C
    assert_eq!(
        shard.children.get(c).unwrap(),
        &[d, b]
    );
    assert_eq!(
        shard.parents.get(c).unwrap(),
        &[a]
    );

    // D
    assert_eq!(
        shard.children.get(d).unwrap(),
        &[]
    );
    assert_eq!(
        shard.parents.get(d).unwrap(),
        &[c, b]
    );
}