pub mod comprehension;
pub mod transformation;
pub mod traversal;

use crate::traits::{comprehension::Comprehension, transformation::Transformation, traversal::Traversal};

pub trait Language: Comprehension + Transformation + Traversal {}