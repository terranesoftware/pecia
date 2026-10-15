mod helpers;

mod relation;
pub use relation::Relation;

use crate::{Shard, ShardError, ShardKey, operations::truncate::helpers::{ancestors::ancestors, descendants::descendants}};

impl Shard {
    /// Modifies the shard's change tree.
    pub fn truncate(
        &mut self,
        relation: Relation,
        target: ShardKey
    ) -> Result<(), ShardError> {
        match relation {
            Relation::All => {
                ancestors(target, self, target)?;
                descendants(target, self, target)?;

                Ok(())
            },
            Relation::Ancestors => ancestors(target, self, target),
            Relation::Descendants => descendants(target, self, target)
        }
    }
}

#[test]
fn test_truncate() {
    use crate::operations::{relocate::Operation, traverse::Direction};
    
    let mut shard = Shard::new();
    
    // Change tree:
    //    A
    //  /  \  ↑ Ancestors
    // B    C
    //  \  /  ↓ Descendants
    //   D

    // A
    shard.change(Vec::new(), Some("A".to_string()));
    let a = shard.head();

    // B
    shard.change(Vec::new(), Some("B".to_string()));
    let b = shard.head();

    // Go back to A to create C
    shard.traverse(&mut Vec::new(), a, Direction::Older).unwrap();
    shard.change(Vec::new(), Some("C".to_string()));
    let c = shard.head();

    // Create D and then copy it over to B
    shard.change(Vec::new(), Some("D".to_string()));
    let d = shard.head();
    shard.relocate(&[b], Operation::Copy, d).unwrap();

    let mut shard1 = shard.clone();
    shard1.truncate(Relation::Descendants, a).unwrap();
    assert_eq!(
        shard1.changes(),
        &HashMap::from([
            (a, shard.changes().get(a).unwrap().clone())
        ])
    );
    
    let mut shard2 = shard.clone();
    shard2.truncate(Relation::Ancestors, d).unwrap();

    let mut shard3 = shard.clone();
}