use std::range::Range;

use crate::{observation::{Observation, buffers::{BufferKey, buffer::region::Region}}, transformation::Transformation};

impl Transformation {
    /// Modifies the payload of a `Buffer`.
    pub fn edit(
        buffer: BufferKey,
        observation: &mut Observation,
        region: Region,
        replacement: &[u8]
    ) {
        // Get buffer
        let buffer = &mut observation.buffers_mut().map_mut().get_mut(buffer).expect("Buffer has already been closed");
        assert!(buffer.0 == true, "Buffer has already been closed");
        
        // Make sure target region is within the buffer
        assert!(
            buffer.1.region().range().start <= region.range().start
            &&
            buffer.1.region().range().end >= region.range().end,
            "Region to edit is not a subset of the region this buffer spans"
        );
        
        // Normalize the target region against the buffer region and then edit
        let normalized = Range {
            start: region.range().start - buffer.1.region().range().start,
            end: region.range().end - buffer.1.region().range().start
        };
        buffer.1.implementation_mut().edit(normalized, replacement);
        
        // If the replacement was an "addition", add
        // If the replacement was a "subtraction", subtract
        let removed = region.range().end - region.range().start;
        let range = buffer.1.region_mut().range_mut();
        if replacement.len() >= removed {
            range.end = range.end.strict_add(replacement.len() - removed);
        }
        else {
            range.end = range.end.strict_sub(removed - replacement.len());
        }
    }
}