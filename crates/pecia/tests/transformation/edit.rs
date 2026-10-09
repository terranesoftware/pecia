use pecia::{Observation, Transformation, observation::{BufferKey, buffers::buffer::region::Region}};

pub(crate) fn edit(
    buffer: BufferKey,
    observation: &mut Observation,
    region: Region,
    replacement: &[u8]
) {
    Transformation::edit(buffer, observation, region, replacement);
}