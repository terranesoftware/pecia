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

impl<C: ComprehensionSpec> Comprehension<C> {
    pub fn descriptions(&self) -> &[C::Description] {
        &self.descriptions
    }

    pub fn descriptions_mut(&mut self) -> &mut [C::Description] {
        &mut self.descriptions
    }

    pub fn diagnostics(&self) -> &[C::Diagnostic] {
        &self.diagnostics
    }

    pub fn diagnostics_mut(&mut self) -> &mut [C::Diagnostic] {
        &mut self.diagnostics
    }

    pub fn highlights(&self) -> &[C::Highlight] {
        &self.highlights
    }

    pub fn highlights_mut(&mut self) -> &mut [C::Highlight] {
        &mut self.highlights
    }
}