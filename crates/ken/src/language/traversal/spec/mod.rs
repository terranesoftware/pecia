pub mod grammar;
pub mod index;

use crate::language::traversal::spec::{grammar::Grammar, index::Index};

pub trait TraversalSpec {
    type Grammar: Grammar;
    type Index: Index;
}