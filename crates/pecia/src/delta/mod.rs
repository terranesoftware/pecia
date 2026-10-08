pub mod diff;

use crate::{delta::diff::Diff, observation::Resource};

pub struct Delta {
    diffs: Vec<Diff>,
    resource: Resource
}

impl Delta {
    pub fn new(
        diffs: Vec<Diff>,
        resource: Resource
    ) -> Self {
        Self {
            diffs,
            resource
        }
    }
    
    pub fn diffs(&self) -> &[Diff] {
        &self.diffs
    }

    pub(super) fn diffs_mut(&mut self) -> &mut Vec<Diff> {
        &mut self.diffs
    }
    
    pub fn resource(&self) -> &Resource {
        &self.resource
    }
}