use crate::text::{observation::{buffer::Buffer, history::History}, transformation::Transformation};

impl Transformation {
    pub fn insert(
        at: usize,
        buffer: &mut Buffer,
        history: &mut History,
        insertion: &[u8]
    ) {
        assert!(
            buffer.region().range().start <= at
            &&
            buffer.region().range().end >= at,
            "Insertion point is not contained by the region this buffer spans"
        );
        
        let normalized = at - buffer.region().range().start;
        let implementation = buffer.implementation_mut();
        implementation.insert(normalized, insertion);
        
        let range = buffer.region_mut().range_mut();
        range.end = range.end.strict_add(insertion.len());
    }
}