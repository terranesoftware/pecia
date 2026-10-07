use buffers::kind::BufferKind;

use crate::{observation::{Observation, buffers::{BufferKey, buffer::{encoding::Encoding, region::Region}}, resource::Resource}, transformation::Transformation};

impl Transformation {
    pub fn open(
        observation: &mut Observation,
        encoding: Encoding,
        implementation: BufferKind,
        region: Region,
        resource: Resource
    ) -> BufferKey {
        observation.buffers_mut().open(encoding, implementation, region, resource)
    }
}