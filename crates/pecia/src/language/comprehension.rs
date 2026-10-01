use languages::traits::comprehension::Comprehension as ComprehensionSpec;

/// Entities that contribute to the understanding of a language.
pub struct Comprehension<C: ComprehensionSpec> {
    descriptions: Vec<C::Description>,
    diagnostics: Vec<C::Diagnostic>,
    highlights: Vec<C::Highlight>
}

impl<C: ComprehensionSpec> Comprehension<C> {
    pub fn descriptions(&self) -> &[C::Description] {
        &self.descriptions
    }

    pub fn diagnostics(&self) -> &[C::Diagnostic] {
        &self.diagnostics
    }

    pub fn highlights(&self) -> &[C::Highlight] {
        &self.highlights
    }
}