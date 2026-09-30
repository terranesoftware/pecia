pub mod encoding;
pub mod region;
pub mod resource;

use crate::text::observation::buffer::{encoding::Encoding, region::Region, resource::Resource};

pub struct Buffer {
    content: Vec<u8>,
    encoding: Encoding,
    region: Region,
    resource: Resource
}

impl Buffer {
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    pub fn region(&self) -> Region {
        self.region
    }

    pub fn resource(&self) -> &Resource {
        &self.resource
    }
}