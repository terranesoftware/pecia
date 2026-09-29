pub trait TransformationSpec {
    type Completion;
    type Formatter;
    type Propagation;
    type Suggestion;
}

/// Entities that contribute to the manipulation of a language.
pub struct Transformation<T: TransformationSpec> {
    completions: Vec<T::Completion>,
    formatter: T::Formatter,
    propagation: T::Propagation,
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

    pub fn propagation(&self) -> &T::Propagation {
        &self.propagation
    }

    pub fn propagation_mut(&mut self) -> &mut T::Propagation {
        &mut self.propagation
    }

    pub fn suggestions(&self) -> &[T::Suggestion] {
        &self.suggestions
    }

    pub fn suggestions_mut(&mut self) -> &mut [T::Suggestion] {
        &mut self.suggestions
    }
}