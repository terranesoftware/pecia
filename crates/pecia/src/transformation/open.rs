use buffers::{Buffer as BufferImpl, implementation::Implementation};

use crate::{observation::{Observation, buffers::{BufferKey, buffer::{Buffer, encoding::Encoding, region::Region}}, Resource}, transformation::Transformation};

impl Transformation {
    pub fn open(
        observation: &mut Observation,
        encoding: Encoding,
        implementation: Implementation,
        region: Region,
        resource: Resource
    ) -> BufferKey {
        let buffers = observation.buffers().map();
        buffers.iter().find(|buffer| {
            buffer.1.0 == false
            &&
            buffer.1.1.encoding() == encoding
            &&
            // Have to do a match here
            // buffer.1.1.implementation()
            true
            &&
            // Have to do a containment check here
            buffer.1.1.region() == region
            &&
            buffer.1.1.resource() == &resource
        });
        
        // Instead of creating a new one, would always try to search for a free one first
        let buffer = match implementation {
            Implementation::Vector => Buffer::new(encoding, BufferImpl::vector(), region, resource)
        };
        
        observation.buffers_mut().map_mut().insert((true, buffer))
    }
}