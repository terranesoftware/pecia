pub mod spec;

use crate::language::transformation::spec::TransformationSpec;

/// Entities that contribute to the manipulation of a language.
pub struct Transformation<T: TransformationSpec> {
    completions: Vec<T::Completion>,
    formatter: T::Formatter,
    propagator: T::Propagator,
    suggestions: Vec<T::Suggestion>,
}

impl<T: TransformationSpec> Transformation<T> {
    pub fn completions(&self) -> &[T::Completion] {
        &self.completions
    }

    pub fn completions_mut(&mut self) -> &mut [T::Completion] {
        &mut self.completions
    }

    pub fn formatter(&self) -> &T::Formatter {
        &self.formatter
    }

    pub fn formatter_mut(&mut self) -> &mut T::Formatter {
        &mut self.formatter
    }

    pub fn propagator(&self) -> &T::Propagator {
        &self.propagator
    }

    pub fn propagator_mut(&mut self) -> &mut T::Propagator {
        &mut self.propagator
    }

    pub fn suggestions(&self) -> &[T::Suggestion] {
        &self.suggestions
    }

    pub fn suggestions_mut(&mut self) -> &mut [T::Suggestion] {
        &mut self.suggestions
    }
}