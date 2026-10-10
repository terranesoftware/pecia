use std::{ops::{Bound, RangeBounds}, range::Range};

use crate::BufferError;

pub struct Vector(Vec<u8>);

impl Vector {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }
    
    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }
    
    pub(crate) fn edit(
        &mut self,
        range: Range<usize>,
        replacement: &[u8]
    ) -> Result<(), BufferError> {
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
        let start = match range.start_bound() {
            Bound::Excluded(bound) => bound.checked_add(1).ok_or(BufferError::OutOfBounds)?,
            Bound::Included(bound) => *bound,
            Bound::Unbounded => 0
        };

        let end = match range.end_bound() {
            Bound::Excluded(bound) => *bound,
            Bound::Included(bound) => bound.checked_add(1).ok_or(BufferError::OutOfBounds)?,
            Bound::Unbounded => self.len()
        };
        
        if start > end {
            return Err(BufferError::ReversedBounds);
        }
        else if end > self.len() {
            return Err(BufferError::OutOfBounds);
        }
        
        Ok(&self.0[start..end])
    }
}