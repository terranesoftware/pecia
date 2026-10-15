mod helpers;

mod relation;
pub use relation::Relation;

use blake3::Hash;

use crate::{Shard, ShardError, operations::truncate::helpers::{ancestors::ancestors, descendants::descendants}};

impl Shard {
    /// Modifies the shard's change tree.
    pub fn truncate(
        &mut self,
        relation: Relation,
        target: &Hash
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
    // Change tree:
    //    A
    //  /  \
    // B    C
    //  \  /
    //   D
    
    let shard = Shard::new();
}