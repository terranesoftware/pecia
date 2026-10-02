use buffers::{Buffer as BufferImpl, kind::BufferKind};

use crate::text::observation::{buffers::{BufferKey, Buffers, buffer::{Buffer, encoding::Encoding, region::Region}}, resource::Resource};

impl Buffers {
    pub(crate) fn close(&mut self, id: BufferKey) {
        
    }

    pub(crate) fn open(
        &mut self,
        encoding: Encoding,
        implementation: BufferKind,
        region: Region,
        resource: Resource
    ) -> BufferKey {
        // Instead of creating a new one, would always try to search for a free one first
        let buffer = match implementation {
            BufferKind::Placeholder => Buffer::new(encoding, BufferImpl::new(), region, resource)
        };
        
        self.map.insert(buffer)
    }
}