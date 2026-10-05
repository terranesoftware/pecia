mod apply;
mod insert;

use crate::text::{observation::{buffers::buffer::region::Region, resource::Resource, shard::change::edit::Edit}, traversal::{diff::Diff, helpers::create::{apply::apply, insert::insert}}};

pub fn create(
    deltas: &mut Vec<(Resource, Vec<Diff>)>,
    edit: &Edit
) {
    // Find if there are already diffs for the edit's resource
    if let Some((_, diffs)) = deltas.iter_mut().find(|delta| &delta.0 == edit.resource()) {
        let start = edit.at();
        let end = edit.at() + edit.replaced().len();
        
        // Flag to check if an edit's region overlapped with the region of an existing diff
        let mut overlapped = false;
        
        // Create a peekable iterator with the diffs that overlap with the edit
        // The diffs are guaranteed not to overlap due to the coalesce at the end
        let targeted = diffs.iter_mut().filter(|diff| {
            let range = diff.region().range();
            
            start < range.end && end > range.start
        }).peekable();
        
        apply(edit, end, &mut overlapped, start, targeted);
        
        if !overlapped {
            let diff = Diff::new(
                Region::new(edit.at(), edit.at() + edit.replaced().len()),
                edit.replacement().to_vec()
            );

            insert(diff, diffs);
        }
    }
    else {
        let diff = Diff::new(
            Region::new(edit.at(), edit.at() + edit.replaced().len()),
            edit.replacement().to_vec()
        );
        
        deltas.push((edit.resource().clone(), vec![diff]));
    }
}