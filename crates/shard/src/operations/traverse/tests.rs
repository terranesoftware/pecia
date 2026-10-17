use crate::{Shard, ShardKey, change::Edit, operations::traverse::{Delta, Direction}};

#[test]
fn traverse() {
    let (first, second, mut shard) = init();

    let mut deltas = Vec::new();
    shard.traverse(&mut deltas, first, Direction::Older).unwrap();
    assert_eq!(
        deltas,
        vec![
            Delta {
                region: (0usize..0usize).into(),
                replacement: b"Hello World!".to_vec()
            }
        ]
    );

    shard.traverse(&mut deltas, second, Direction::Newer).unwrap();
    assert_eq!(
        deltas,
        vec![
            Delta {
                region: (0usize..0usize).into(),
                replacement: b"Hello World!".to_vec()
            },
            Delta {
                region: (5usize..11usize).into(),
                replacement: Vec::new()
            }
        ]
    );
}

fn init() -> (
    ShardKey,
    ShardKey,
    Shard
) {
    // Change tree:
    // 1. ""
    // 2. "Hello World!"
    // 3. "Hello!"
    
    let mut shard = Shard::new();
    
    shard.change(vec![
        Edit {
            at: 0,
            replaced: vec![],
            replacement: b"Hello World!".to_vec()
        }
    ], None);
    let first = shard.head;
    
    shard.change(vec![
        Edit {
            at: 5,
            replaced: b" World".to_vec(),
            replacement: vec![]
        }
    ], None);
    let second = shard.head;

    (first, second, shard)
}