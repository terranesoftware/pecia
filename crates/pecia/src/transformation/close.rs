use crate::{observation::{Observation, buffers::BufferKey}, transformation::Transformation};

impl Transformation {
    pub fn close(
        key: BufferKey,
        observation: &mut Observation
    ) {
        observation.buffers_mut().close(key);
    }
}