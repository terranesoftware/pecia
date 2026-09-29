pub mod buffer;

use crate::text::observation::buffer::Buffer;

/// Entity that contributes to the inspection of text.
pub struct Observation(Vec<Buffer>);

impl Observation {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    
    pub fn buffers(&self) -> &[Buffer] {
        &self.0
    }

    pub fn buffers_mut(&mut self) -> &mut [Buffer] {
        &mut self.0
    }
}