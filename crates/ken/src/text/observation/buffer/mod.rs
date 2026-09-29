pub mod encoding;
pub mod resource;

use std::range::RangeInclusive;

use crate::text::observation::buffer::{encoding::Encoding, resource::Resource};

pub struct Buffer {
    content: Vec<u8>,
    encoding: Encoding,
    region: RangeInclusive<usize>,
    resource: Resource
}

impl Buffer {
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut [u8] {
        &mut self.content
    }

    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    pub fn region(&self) -> RangeInclusive<usize> {
        self.region
    }

    pub fn resource(&self) -> &Resource {
        &self.resource
    }

    pub fn resource_mut(&mut self) -> &mut Resource {
        &mut self.resource
    }
}