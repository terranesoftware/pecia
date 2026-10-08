use std::range::Range;

pub struct Vector(Vec<u8>);

impl Vector {
    pub fn new() -> Self {
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
        
    }
}