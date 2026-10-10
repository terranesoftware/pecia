mod apply;
use apply::apply;

mod insert;
use insert::insert;

use std::range::Range;

use crate::{change::Edit, operations::traverse::Delta};

pub fn create(
    deltas: &mut Vec<Delta>,
    edit: &Edit
) {
    let start = edit.at;
    let end = edit.at + edit.replaced.len();
    
    // Flag to check if an edit's region overlapped with the region of an existing delta
    let mut overlapped = false;
    
    // Create a peekable iterator with the deltas that overlap with the edit
    // The deltas are guaranteed not to overlap since each iteration "glues" them together with each edit applied
    let targeted = deltas.iter_mut().filter(|delta| {
        let region = delta.region;
        
        start < region.end && end > region.start
    }).peekable();
    
    // See if we can apply this edit to a delta/deltas
    apply(edit, end, &mut overlapped, start, targeted);
    
    // If no overlap, just create a new delta and insert it
    if !overlapped {
        let delta = Delta {
            region: Range {
                start: edit.at,
                end: edit.at + edit.replaced.len()
            },
            replacement: edit.replacement.to_vec()
        };
        
        insert(delta, deltas);
    }
}