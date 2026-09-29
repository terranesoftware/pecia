pub trait ComprehensionSpec {
    type Description;
    type Diagnostic;
    type Highlight;
}

/// Entities that contribute to the understanding of a language.
pub struct Comprehension<C: ComprehensionSpec> {
    descriptions: Vec<C::Description>,
    diagnostics: Vec<C::Diagnostic>,
    highlights: Vec<C::Highlight>
}