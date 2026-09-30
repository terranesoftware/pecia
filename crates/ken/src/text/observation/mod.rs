pub mod buffer;
pub mod history;

use crate::text::observation::{buffer::Buffer, history::History};

/// Entities that contributes to the inspection of text.
pub struct Observation {
    buffers: Vec<Buffer>,
    history: History
}

impl Observation {
    pub fn buffers(&self) -> &[Buffer] {
        &self.buffers
    }

    pub fn history(&self) -> &History {
        &self.history
    }
}