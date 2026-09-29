pub mod completion;
pub mod formatter;
pub mod propagation;
pub mod suggestion;

use crate::language::transformation::{completion::Completion, formatter::Formatter, propagation::Propagation, suggestion::Suggestion};

/// Entities that contribute to the manipulation of a language.
pub struct Transformation {
    completions: Vec<Completion>,
    formatter: Formatter,
    propagation: Propagation,
    suggestions: Vec<Suggestion>,
}