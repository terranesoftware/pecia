use std::{ops::{Bound, RangeBounds}, range::Range};

use crate::BufferError;

pub(super) fn normalize<R: RangeBounds<usize>>(
    len: usize,
    range: R
) -> Result<Range<usize>, BufferError> {
    let start = match range.start_bound() {
        Bound::Excluded(bound) => bound.checked_add(1).ok_or(BufferError::OutOfBounds)?,
        Bound::Included(bound) => *bound,
        Bound::Unbounded => 0
    };
    
    let end = match range.end_bound() {
        Bound::Excluded(bound) => *bound,
        Bound::Included(bound) => bound.checked_add(1).ok_or(BufferError::OutOfBounds)?,
        Bound::Unbounded => len
    };

    Ok((start..end).into())
}