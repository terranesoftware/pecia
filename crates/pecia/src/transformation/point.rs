use crate::{observation::{Observation, buffers::BufferKey, Resource}, transformation::Transformation};

impl Transformation {
    pub fn point(
        key: BufferKey,
        observation: &mut Observation,
        resource: Resource
    ) {
        let buffer = &mut observation.buffers_mut().map_mut().get_mut(key).expect("Buffer has already been closed").1;
        
        *buffer.resource_mut() = resource;
    }
}