use std::range::Range;

pub enum Buffer {
    
}

// Placeholders for now
impl Buffer {
    pub fn insert(&mut self, at: usize, insertion: &[u8]) {
        
    }

    pub fn len(&self) -> usize {
        0
    }

    pub fn remove(&mut self, range: Range<usize>) {
        
    }

    pub fn replace(&mut self, range: Range<usize>, replacement: &[u8]) {
        
    }
}