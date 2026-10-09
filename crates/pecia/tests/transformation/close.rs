use pecia::{Observation, Transformation, observation::BufferKey};

use crate::observation;

#[test]
fn test_close() {
    let mut observation = observation();
    
    // close(&mut observation);
}

fn close(
    key: BufferKey,
    observation: &mut Observation
) {
    Transformation::close(key, observation);
}