use serde::Serialize;

/// A modification made to text.
#[derive(Clone, Serialize)]
pub struct Edit {
    pub at: usize,
    pub replaced: Vec<u8>,
    pub replacement: Vec<u8>
}