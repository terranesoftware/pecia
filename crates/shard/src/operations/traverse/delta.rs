use std::range::Range;

/// A unit of accumulated change.
pub struct Delta {
    pub(super) region: Range<usize>,
    pub(super) replacement: Vec<u8>
}

impl Delta {
    /// Returns a copy of the contained region.
    pub fn region(&self) -> Range<usize> {
        self.region
    }

    /// Returns a reference to the contained replacement.
    pub fn replacement(&self) -> &[u8] {
        &self.replacement
    }
}