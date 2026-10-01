use std::range::Range;

use crate::text::observation::buffers::buffer::{Buffer, region::Region};

impl Buffer {
    pub fn remove(
        &mut self,
        region: Region
    ) {
        assert!(
            self.region().range().start <= region.range().start
            &&
            self.region().range().end >= region.range().end,
            "Region to edit is not a subset of the region this buffer spans"
        );
        
        let normalized = Range {
            start: region.range().start - self.region().range().start,
            end: region.range().end - self.region().range().start
        };
        let implementation = self.implementation_mut();
        implementation.remove(normalized);
        
        let range = self.region_mut().range_mut();
        let removed = region.range().end - region.range().start;
        range.end = range.end.strict_sub(removed)
    }
}