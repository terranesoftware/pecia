use blake3::Hash;

use crate::observation::{Observation, scope::Scope};

pub(crate) fn ancestors(
    observation: &mut Observation,
    child: (&Scope, &Hash)
) {
    
}