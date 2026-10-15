mod edit;
pub use edit::Edit;

use time::Timestamp;

/// A group of modifications that happened in the same instant.
#[derive(Clone, Debug, PartialEq)]
pub struct Change {
    pub edits: Vec<Edit>,
    pub timestamp: Timestamp
}