use buffers::implementation::Implementation;
use pecia::{Observation, Transformation, observation::{Resource, buffers::buffer::{encoding::Encoding, region::Region}}};

use crate::observation;

#[test]
fn test_open() {
    let mut observation = observation();

    // open(observation);
}

fn open(
    encoding: Encoding,
    implementation: Implementation,
    observation: &mut Observation,
    region: Region,
    resource: Resource
) {
    Transformation::open(encoding, implementation, observation, region, resource);
}