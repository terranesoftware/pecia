use crate::observation::buffers::buffer::region::Region;

pub struct Diff {
    region: Region,
    replacement: Vec<u8>
}

impl Diff {
    pub fn new(
        region: Region,
        replacement: Vec<u8>
    ) -> Self {
        Self {
            region,
            replacement
        }
    }

    pub fn region(&self) -> Region {
        self.region
    }

    pub(crate) fn region_mut(&mut self) -> &mut Region {
        &mut self.region
    }

    pub fn replacement(&self) -> &[u8] {
        &self.replacement
    }

    pub(crate) fn replacement_mut(&mut self) -> &mut Vec<u8> {
        &mut self.replacement
    }
}