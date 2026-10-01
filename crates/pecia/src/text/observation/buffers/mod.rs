pub mod buffer;
pub mod operations;

use slotmap::{SlotMap, new_key_type};

use crate::text::observation::buffers::buffer::Buffer;

new_key_type! { pub struct BufferKey; }

pub struct Buffers {
    current: usize,
    map: SlotMap<BufferKey, Buffer>,
    max: usize
}

impl Buffers {
    /// Constructs a `Buffers`, taking in the maximum number of bytes the buffer implementations should hold before trimming the arena.
    pub fn new(max: usize) -> Self {
        Self {
            current: 0,
            max,
            map: SlotMap::with_key()
        }
    }
    
    pub fn current(&self) -> usize {
        self.current
    }

    pub fn max(&self) -> usize {
        self.max
    }

    pub fn map(&self) -> &SlotMap<BufferKey, Buffer> {
        &self.map
    }
}