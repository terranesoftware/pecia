pub mod completion;
pub mod formatter;
pub mod propagator;
pub mod suggestion;

use crate::language::transformation::spec::{completion::Completion, formatter::Formatter, propagator::Propagator, suggestion::Suggestion};

pub trait TransformationSpec {
    type Completion: Completion;
    type Formatter: Formatter;
    type Propagator: Propagator;
    type Suggestion: Suggestion;
}