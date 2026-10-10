use std::ops::RangeBounds;

use crate::{Buffer, BufferError};

impl Buffer {
    /// Clears the buffer.
    pub fn clear(&mut self) {
        match self {
            Buffer::Vector(vector) => vector.clear()
        }
    }
    
    /// Applies an edit to a buffer.
    pub fn edit<R: RangeBounds<usize>>(
        &mut self,
        range: R,
        replacement: &[u8]
    ) -> Result<(), BufferError> {
        match self {
            Buffer::Vector(vector) => vector.edit(range, replacement)
        }
    }
    
    /// Returns the current length of a buffer.
    pub fn len(&self) -> usize {
        match self {
            Buffer::Vector(vector) => vector.len()
        }
    }

    /// Reads the contents of a buffer.
    pub fn read<R: RangeBounds<usize>>(
        &self,
        range: R
    ) -> Result<&[u8], BufferError> {
        match self {
            Buffer::Vector(vector) => vector.read(range)
        }
    }
}