pub mod dictionary;

use crate::language::traversal::dictionary::Dictionary;

/// Entities that contribute to the navigation of a language.
pub struct Traversal {
    dictionaries: Vec<Dictionary>
}