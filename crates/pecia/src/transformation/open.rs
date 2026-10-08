use buffers::{Buffer as BufferImpl, implementation::Implementation};

use crate::{observation::{Observation, buffers::{BufferKey, buffer::{Buffer, encoding::Encoding, region::Region}}, resource::Resource}, transformation::Transformation};

impl Transformation {
    pub fn open(
        observation: &mut Observation,
        encoding: Encoding,
        implementation: Implementation,
        region: Region,
        resource: Resource
    ) -> BufferKey {
        // Instead of creating a new one, would always try to search for a free one first
        let buffer = match implementation {
            Implementation::Vector => Buffer::new(encoding, BufferImpl::vector(), region, resource)
        };
        
        observation.buffers_mut().map_mut().insert(buffer)
    }
}