use pecia::{Observation, Transformation, observation::{BufferKey, buffers::buffer::region::Region}};

use crate::observation;

#[test]
fn test_edit() {
    let mut observation = observation();
    
    // edit(&mut observation);
}

fn edit(
    buffer: BufferKey,
    observation: &mut Observation,
    region: Region,
    replacement: &[u8]
) {
    Transformation::edit(buffer, observation, region, replacement);
}