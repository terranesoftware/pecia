pub mod encoding;
pub mod operations;
pub mod region;

use buffers::Buffer as BufferImpl;

use crate::text::observation::{buffers::buffer::{encoding::Encoding, region::Region}, resource::Resource};

pub struct Buffer {
    implementation: BufferImpl,
    encoding: Encoding,
    region: Region,
    resource: Resource
}

impl Buffer {
    pub fn implementation(&self) -> &BufferImpl {
        &self.implementation
    }

    pub fn implementation_mut(&mut self) -> &mut BufferImpl {
        &mut self.implementation
    }

    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    pub fn region(&self) -> Region {
        self.region
    }

    pub fn region_mut(&mut self) -> &mut Region {
        &mut self.region
    }

    pub fn resource(&self) -> &Resource {
        &self.resource
    }
}