use buffers::{Buffer as BufferImpl, implementation::Implementation};

use crate::{observation::{Observation, buffers::{BufferKey, buffer::{Buffer, encoding::Encoding, region::Region}}, Resource}, transformation::Transformation};

impl Transformation {
    /// Opens a buffer with the specified arguments.
    pub fn open(
        encoding: Encoding,
        implementation: Implementation,
        observation: &mut Observation,
        region: Region,
        resource: Resource
    ) -> BufferKey {
        let buffers = observation.buffers_mut().map_mut();
        
        // See if there is a free buffer matching the requirements
        if let Some(buffer) = buffers.iter_mut().find(|(_, (in_use, buffer))| {
            // Make sure its not in use
            *in_use == false
            &&
            buffer.implementation() == &implementation
            &&
            buffer.resource() == &resource
        }) {
            // Change the encoding and region to what was requested
            *buffer.1.1.encoding_mut() = encoding;
            *buffer.1.1.region_mut() = region;
            
            return buffer.0
        };
        
        // If not, create a new one and insert it into `Buffers`
        let buffer = match implementation {
            Implementation::Vector => Buffer::new(encoding, BufferImpl::vector(), region, resource)
        };
        let key = observation.buffers_mut().map_mut().insert((true, buffer));

        key
    }
}