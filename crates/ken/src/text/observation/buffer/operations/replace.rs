use std::range::Range;

use crate::text::observation::buffer::{Buffer, region::Region};

impl Buffer {
    pub fn replace(
        &mut self,
        region: Region,
        replacement: &[u8]
    ) {
        assert!(
            self.region().range().start <= region.range().start
            &&
            self.region().range().end >= region.range().end,
            "Region to edit is not a subset of the region this self spans"
        );
        
        let normalized = Range {
            start: region.range().start - self.region().range().start,
            end: region.range().end - self.region().range().start
        };
        let implementation = self.implementation_mut();
        implementation.replace(normalized, replacement);
        
        let range = self.region_mut().range_mut();
        let removed = region.range().end - region.range().start;
        if replacement.len() >= removed {
            range.end = range.end.strict_add(replacement.len() - removed);
        }
        else {
            range.end = range.end.strict_sub(removed - replacement.len())
        }
    }
}