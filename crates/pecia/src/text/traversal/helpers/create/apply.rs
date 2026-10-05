use std::range::Range;

use crate::text::{observation::shard::change::edit::Edit, traversal::diff::Diff};

pub(super) fn apply<'a>(
    edit: &Edit,
    end: usize,
    overlapped: &mut bool,
    start: usize,
    mut targeted: impl Iterator<Item = &'a mut Diff>
) {
    // STILL BROKEN, HAVE TO FIX EDITS SPANNING THE DIFFS, PEEK TO THE NEXT DIFF TO CHECK ITS STARTING POINT
    while let Some(diff) = targeted.next() {
        *overlapped = true;
        
        let range = diff.region().range();
        let intersection_start = start.saturating_sub(range.start);
        let intersection_end = end.saturating_sub(range.start);
        
        let prefix = &diff.replacement()[..intersection_start];
        let suffix = if intersection_end < diff.replacement().len() {
            &diff.replacement()[intersection_end..]
        }
        else {
            &[]
        };
        let mut intersection: Vec<u8> = Vec::with_capacity(prefix.len() + edit.replacement().len() + suffix.len());
        intersection.extend(prefix);
        intersection.extend(edit.replacement());
        intersection.extend(suffix);
        
        *diff.region_mut().range_mut() = Range {
            start: start.min(range.start),
            end: end.max(range.end)
        };
        *diff.replacement_mut() = intersection;
    }
}