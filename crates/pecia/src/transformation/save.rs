use crate::{observation::{Observation, buffers::BufferKey}, transformation::Transformation};

impl Transformation {
    pub fn save(
        key: BufferKey,
        observation: &mut Observation
    ) {
        let buffer = observation.buffers().map().get(key).expect("Buffer has already been closed");
        // Need some type of save operation
        observation.buffers_mut().close(key);
    }
}