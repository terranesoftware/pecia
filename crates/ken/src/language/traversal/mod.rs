pub mod dictionary;

use crate::language::traversal::dictionary::Dictionary;

/// Entities that contribute to the navigation of the text.
pub struct Traversal {
    dictionaries: Vec<Dictionary>
}