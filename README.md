# pecia

**Text, and only text.**

## Buffers

```rust
use pecia::{Buffer, buffers::BufferError};

fn buffers() -> Result<(), BufferError> {
    let mut vector = Buffer::vector();
    
    vector.edit(0..0, "Hello World!".as_bytes())?;
    let len = vector.len();

    let text = vector.read(0..12)?;

    vector.clear();

    Ok(())
}
```

## Shard

```rust
use pecia::{Shard, shard::{ShardError, operations::{relocate::Operation, traverse::Direction, truncate::Relation}, reference::ReferenceKind}};

fn shard() -> Result<(), ShardError> {
    let mut shard = Shard::new();
    
    let edits = Vec::new();
    shard.change(edits, None);
    let first = shard.head();
    
    shard.reference(ReferenceKind::Change, "Empty Change", first)?;
    
    shard.change(Vec::new(), Some("Second Change"));
    let second = shard.head();
    
    let mut deltas = Vec::new();
    shard.traverse(&mut deltas, first, Direction::Older)?;
    
    shard.relocate(&[first], Operation::Copy, second)?;
    
    shard.truncate(second, Relation::Ancestors)?;
    
    Ok(())
}
```