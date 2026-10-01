use std::range::Range;

#[derive(Clone, Copy)]
pub struct Region(Range<usize>);

impl Region {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "A reversed region is illogical");

        Self(Range { start, end })
    }

    pub fn range(&self) -> Range<usize> {
        self.0
    }

    pub fn range_mut(&mut self) -> &mut Range<usize> {
        &mut self.0
    }
}