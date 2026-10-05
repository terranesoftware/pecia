use crate::observation::buffers::buffer::Buffer;

impl Buffer {
    pub fn insert(
        &mut self,
        at: usize,
        insertion: &[u8]
    ) {
        assert!(
            self.region().range().start <= at
            &&
            self.region().range().end >= at,
            "Insertion point is not contained by the region this buffer spans"
        );
        
        let normalized = at - self.region().range().start;
        let implementation = self.implementation_mut();
        implementation.insert(normalized, insertion);
        
        let range = self.region_mut().range_mut();
        range.end = range.end.strict_add(insertion.len());
    }
}