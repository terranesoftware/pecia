use std::range::RangeInclusive;

#[derive(Clone, Copy)]
pub struct Region(RangeInclusive<usize>);

impl Region {
    pub fn new(start: usize, last: usize) -> Self {
        assert!(start > last, "An empty region is illogical");

        Self(RangeInclusive { start, last })
    }

    pub fn range(&self) -> RangeInclusive<usize> {
        self.0
    }
}