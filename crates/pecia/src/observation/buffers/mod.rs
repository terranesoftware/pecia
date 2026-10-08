pub mod buffer;

use slotmap::{SlotMap, new_key_type};

use crate::observation::buffers::buffer::Buffer;

new_key_type! { pub struct BufferKey; }

pub struct Buffers {
    current: usize,
    map: SlotMap<BufferKey, (bool, Buffer)>,
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

    pub(crate) fn current_mut(&mut self) -> &mut usize {
        &mut self.current
    }

    pub fn max(&self) -> usize {
        self.max
    }

    pub fn map(&self) -> &SlotMap<BufferKey, (bool, Buffer)> {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut SlotMap<BufferKey, (bool, Buffer)> {
        &mut self.map
    }
}