use crate::{observation::{Observation, buffers::BufferKey, Resource}, transformation::Transformation};

impl Transformation {
    /// Targets the buffer at the specified `Resource`.
    pub fn point(
        buffer: BufferKey,
        observation: &mut Observation,
        resource: Resource
    ) {
        let buffer = &mut observation.buffers_mut().map_mut().get_mut(buffer).expect("Buffer has already been closed");
        assert!(buffer.0 == true, "Buffer has already been closed");
        
        *buffer.1.resource_mut() = resource;
    }
}