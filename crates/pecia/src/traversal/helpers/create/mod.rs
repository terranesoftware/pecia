mod apply;
mod insert;

use crate::{observation::{buffers::buffer::region::Region, shard::change::edit::Edit}, traversal::{delta::{Delta, diff::Diff}, helpers::create::{apply::apply, insert::insert}}};

pub fn create(
    deltas: &mut Vec<Delta>,
    edit: &Edit
) {
    // Find if there are already diffs for the edit's resource
    if let Some(delta) = deltas.iter_mut().find(|delta| delta.resource() == edit.resource()) {
        let diffs = delta.diffs_mut();
        
        let start = edit.at();
        let end = edit.at() + edit.replaced().len();
        
        // Flag to check if an edit's region overlapped with the region of an existing diff
        let mut overlapped = false;
        
        // Create a peekable iterator with the diffs that overlap with the edit
        // The diffs are guaranteed not to overlap due to the coalesce at the end
        let targeted = diffs.iter_mut().filter(|diff| {
            let range = diff.region().range();
            
            start < range.end && end > range.start
        })
        .peekable();
        
        // See if we can apply this edit to a diff/diffs
        apply(edit, end, &mut overlapped, start, targeted);
        
        // If no overlap, just create a new diff and insert it
        if !overlapped {
            let diff = Diff::new(
                Region::new(edit.at(), edit.at() + edit.replaced().len()),
                edit.replacement().to_vec()
            );

            insert(diff, diffs);
        }
    }
    else {
        // If no resource, create a new diff, wrap it in a Vec, and insert it as the first for its resource
        let diff = Diff::new(
            Region::new(edit.at(), edit.at() + edit.replaced().len()),
            edit.replacement().to_vec()
        );
        
        deltas.push(Delta::new(vec![diff], edit.resource().clone()));
    }
}