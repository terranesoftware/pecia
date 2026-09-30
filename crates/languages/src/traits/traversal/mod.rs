pub mod grammar;
pub mod index;

use crate::traits::traversal::{grammar::Grammar, index::Index};

pub trait Traversal {
    type Grammar: Grammar;
    type Index: Index;
}