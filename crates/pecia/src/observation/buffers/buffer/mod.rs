pub mod encoding;
pub mod region;

use buffers::Buffer as BufferImpl;

use crate::observation::{buffers::buffer::{encoding::Encoding, region::Region}, resource::Resource};

pub struct Buffer {
    encoding: Encoding,
    implementation: BufferImpl,
    region: Region,
    resource: Resource
}

impl Buffer {
    pub fn new(
        encoding: Encoding,
        implementation: BufferImpl,
        region: Region,
        resource: Resource
    ) -> Self {
        Self {
            encoding,
            implementation,
            region,
            resource
        }
    }
    
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    pub(crate) fn encoding_mut(&mut self) -> &mut Encoding {
        &mut self.encoding
    }

    pub fn implementation(&self) -> &BufferImpl {
        &self.implementation
    }
    
    pub(crate) fn implementation_mut(&mut self) -> &mut BufferImpl {
        &mut self.implementation
    }

    pub fn region(&self) -> Region {
        self.region
    }

    pub(crate) fn region_mut(&mut self) -> &mut Region {
        &mut self.region
    }

    pub fn resource(&self) -> &Resource {
        &self.resource
    }

    pub(crate) fn resource_mut(&mut self) -> &mut Resource {
        &mut self.resource
    }
}