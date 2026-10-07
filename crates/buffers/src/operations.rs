use std::range::Range;

use crate::Buffer;

impl Buffer {
    pub fn len(&self) -> usize {
        0
    }

    pub fn edit(
        &mut self,
        range: Range<usize>,
        replacement: &[u8]
    ) {
        
    }
}