/// A modification made to text.
pub struct Edit {
    pub at: usize,
    pub replaced: Vec<u8>,
    pub replacement: Vec<u8>
}