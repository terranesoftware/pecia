use crate::operations::traverse::Delta;

pub(super) fn insert(
    delta: Delta,
    deltas: &mut Vec<Delta>
) {
    // Binary search to maintain sort by region start
    let index = deltas.binary_search_by_key(
        &delta.region.start,
        |diff| diff.region.start
    )
    .unwrap_or_else(|index| index);
    
    deltas.insert(index, delta);
}