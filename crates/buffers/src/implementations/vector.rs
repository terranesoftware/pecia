use std::ops::RangeBounds;

use crate::{BufferError, implementations::normalize};

pub struct Vector(Vec<u8>);

impl Vector {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }
    
    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }
    
    pub(crate) fn edit<R: RangeBounds<usize>>(
        &mut self,
        range: R,
        replacement: &[u8]
    ) -> Result<(), BufferError> {
        let range = normalize(self.len(), range)?;
        
        if range.start > range.end {
            return Err(BufferError::ReversedBounds);
        }
        else if range.end > self.len() {
            return Err(BufferError::OutOfBounds);
        }
        
        self.0.splice(range, replacement.iter().copied());

        Ok(())
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn read<R: RangeBounds<usize>>(
        &self,
        range: R
    ) -> Result<&[u8], BufferError> {
        let range = normalize(self.len(), range)?;
        
        if range.start > range.end {
            return Err(BufferError::ReversedBounds);
        }
        else if range.end > self.len() {
            return Err(BufferError::OutOfBounds);
        }
        
        Ok(&self.0[range])
    }
}