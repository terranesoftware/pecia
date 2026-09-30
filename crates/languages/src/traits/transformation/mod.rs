pub mod completion;
pub mod formatter;
pub mod propagator;
pub mod suggestion;

use crate::traits::transformation::{completion::Completion, formatter::Formatter, propagator::Propagator, suggestion::Suggestion};

pub trait Transformation {
    type Completion: Completion;
    type Formatter: Formatter;
    type Propagator: Propagator;
    type Suggestion: Suggestion;
}