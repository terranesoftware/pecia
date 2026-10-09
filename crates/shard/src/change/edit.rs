pub struct Edit {
    at: usize,
    replaced: Vec<u8>,
    replacement: Vec<u8>,
}

impl Edit {
    pub fn new(
        at: usize,
        replaced: Vec<u8>,
        replacement: Vec<u8>,
    ) -> Self {
        Self {
            at,
            replaced,
            replacement,
        }
    }
    
    pub fn at(&self) -> usize {
        self.at
    }

    pub fn replaced(&self) -> &[u8] {
        &self.replaced
    }

    pub fn replacement(&self) -> &[u8] {
        &self.replacement
    }
}