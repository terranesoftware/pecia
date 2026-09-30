use std::range::Range;

use crate::text::{observation::{buffer::{Buffer, region::Region}, history::History}, transformation::Transformation};

impl Transformation {
    pub fn remove(
        buffer: &mut Buffer,
        history: &mut History,
        region: Region
    ) {
        assert!(
            buffer.region().range().start <= region.range().start
            &&
            buffer.region().range().end >= region.range().end,
            "Region to edit is not a subset of the region this buffer spans"
        );
        
        let normalized = Range {
            start: region.range().start - buffer.region().range().start,
            end: region.range().end - buffer.region().range().start
        };
        let implementation = buffer.implementation_mut();
        implementation.remove(normalized);
        
        let range = buffer.region_mut().range_mut();
        let removed = region.range().end - region.range().start;
        range.end = range.end.strict_sub(removed)
    }
}