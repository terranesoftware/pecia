use std::assert_matches;

use crate::{observation::{Observation, buffers::BufferKey, resource::Resource}, transformation::Transformation};

impl Transformation {
    pub fn persist(
        key: BufferKey,
        observation: &mut Observation,
        resource: Resource
    ) {
        assert_matches!(resource, Resource::Memory(_) | Resource::Stdin, "Cannot persist to an ephemeral resource");
        
        let buffer = observation.buffers_mut().map_mut().get_mut(key).expect("Buffer has already been closed");
        assert_matches!(buffer.resource(), Resource::Memory(_) | Resource::Stdin, "Buffer is already persisted");
        
        *buffer.resource_mut() = resource;
    }
}