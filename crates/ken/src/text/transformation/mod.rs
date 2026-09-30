use std::range::Range;

use crate::text::observation::{buffer::{Buffer, region::Region}, history::History};

/// Entity that contributes to the manipulation of text.
pub struct Transformation;

impl Transformation {
    pub fn insert(
        at: usize,
        buffer: &mut Buffer,
        history: &mut History,
        insertion: &[u8]
    ) {
        assert!(buffer.region().range().start <= at && buffer.region().range().end >= at, "Insertion point is not contained by the region this buffer spans");

        let normalized = at - buffer.region().range().start;
        let implementation = buffer.implementation_mut();
        implementation.insert(normalized, insertion);

        let range = buffer.region_mut().range_mut();
        range.end = range.end.strict_add(insertion.len());
    }

    pub fn remove(
        buffer: &mut Buffer,
        history: &mut History,
        region: Region
    ) {
        assert!(
            buffer.region().range().contains(&region.range().start)
            &&
            buffer.region().range().contains(&region.range().end),
            "Region to edit is not a subset of the region this buffer spans"
        );

        let normalized = Range {
            start: region.range().start - buffer.region().range().start,
            end: region.range().end - buffer.region().range().start
        };
        let implementation = buffer.implementation_mut();
        implementation.remove(normalized);
        
        let removed = region.range().end - region.range().start + 1;
        let range = buffer.region_mut().range_mut();
        range.end = range.end.strict_sub(removed)
    }

    pub fn replace(
        buffer: &mut Buffer,
        history: &mut History,
        region: Region,
        replacement: &[u8]
    ) {
        assert!(
            buffer.region().range().contains(&region.range().start)
            &&
            buffer.region().range().contains(&region.range().end),
            "Region to edit is not a subset of the region this buffer spans"
        );

        let normalized = Range {
            start: region.range().start - buffer.region().range().start,
            end: region.range().end - buffer.region().range().start
        };
        let implementation = buffer.implementation_mut();
        let old_len = implementation.len();
        implementation.replace(normalized, replacement);
        let new_len = implementation.len();

        let replaced = new_len as isize - old_len as isize;
        let range = buffer.region_mut().range_mut();
        range.end = range.end.strict_add_signed(replaced);
    }
}