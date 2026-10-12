use pecia::{Shard, shard::{ShardError, operations::{relocate::Operation, traverse::Direction, truncate::Relation}, reference::ReferenceKind}};

fn main() -> Result<(), ShardError> {
    let mut shard = Shard::new();

    let edits = Vec::new();
    shard.change(edits, None);
    shard.reference(ReferenceKind::Change, "Empty Change".to_string(), shard.head())?;
    let first = shard.head();

    shard.change(Vec::new(), Some("Second Change".to_string()));
    let second = shard.head();

    let mut deltas = Vec::new();
    shard.traverse(&mut deltas, &first, Direction::Older)?;

    shard.relocate(&[first], Operation::Copy, &second)?;

    shard.truncate(Relation::Ancestors, &second)?;
    
    Ok(())
}