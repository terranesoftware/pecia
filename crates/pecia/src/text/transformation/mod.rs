use blake3::Hash;

use crate::text::{observation::{Observation, buffers::buffer::region::Region, resource::Resource}, traversal::Delta};

/// The entity that contributes to the manipulation of text.
pub struct Transformation;

impl Transformation {
    pub fn checkout(
        delta: Delta,
        destination: Hash,
        observation: &mut Observation
    ) {
        
    }
    
    pub fn close(
        observation: &mut Observation,
        region: Region,
        resource: Resource
    ) {
        
    }

    pub fn edit(
        observation: &mut Observation,
        region: Region,
        replacement: &[u8],
        resource: Resource
    ) {
        
    }

    pub fn reset(
        delta: Delta,
        destination: Hash,
        observation: &mut Observation
    ) {
        
    }

    pub fn save(observation: &mut Observation, resource: Resource) {
        
    }

    pub fn squash(observation: &mut Observation) {
        
    }
}