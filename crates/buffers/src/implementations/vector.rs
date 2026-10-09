use std::range::Range;

pub struct Vector(Vec<u8>);

impl Vector {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }
    
    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }
    
    pub(crate) fn edit(
        &mut self,
        range: Range<usize>,
        replacement: &[u8]
    ) {
        self.0.splice(range, replacement.iter().copied());
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn read(
        &self,
        range: Range<usize>
    ) -> &[u8] {
        &self.0[range]
    }
}