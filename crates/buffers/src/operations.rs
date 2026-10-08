use std::range::Range;

use crate::Buffer;

impl Buffer {
    pub fn clear(&mut self) {
        match self {
            Buffer::Vector(vector) => vector.clear()
        }
    }
    
    pub fn edit(
        &mut self,
        range: Range<usize>,
        replacement: &[u8]
    ) {
        match self {
            Buffer::Vector(vector) => vector.edit(range, replacement)
        }
    }
    
    pub fn len(&self) -> usize {
        0
    }
}