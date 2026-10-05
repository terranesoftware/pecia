use crate::observation::resource::Resource;

pub struct Edit {
    at: usize,
    replaced: Vec<u8>,
    replacement: Vec<u8>,
    resource: Resource
}

impl Edit {
    pub fn new(
        at: usize,
        replaced: Vec<u8>,
        replacement: Vec<u8>,
        resource: Resource
    ) -> Self {
        Self {
            at,
            replaced,
            replacement,
            resource
        }
    }
    
    pub fn at(&self) -> usize {
        self.at
    }

    pub fn replaced(&self) -> &[u8] {
        &self.replaced
    }

    pub fn replacement(&self) -> &[u8] {
        &self.replacement
    }

    pub fn resource(&self) -> &Resource {
        &self.resource
    }
}