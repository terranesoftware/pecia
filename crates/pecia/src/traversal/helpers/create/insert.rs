use crate::delta::diff::Diff;

pub(super) fn insert(
    diff: Diff,
    diffs: &mut Vec<Diff>
) {
    // Binary search to maintain sort by region start
    let index = diffs.binary_search_by_key(
        &diff.region().range().start,
        |diff| diff.region().range().start
    )
    .unwrap_or_else(|index| index);
    
    diffs.insert(index, diff);
}