/// A modification made to text.
#[derive(Clone, Debug, PartialEq)]
pub struct Edit {
    pub at: usize,
    pub replaced: Vec<u8>,
    pub replacement: Vec<u8>
}