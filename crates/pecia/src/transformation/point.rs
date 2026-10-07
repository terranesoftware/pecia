use crate::{observation::{Observation, buffers::BufferKey, resource::Resource}, transformation::Transformation};

impl Transformation {
    pub fn point(
        key: BufferKey,
        observation: &mut Observation,
        resource: Resource
    ) {
        let buffer = observation.buffers_mut().map_mut().get_mut(key).expect("Buffer has already been closed");
        
        *buffer.resource_mut() = resource;
    }
}