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