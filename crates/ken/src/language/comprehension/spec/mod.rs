pub mod description;
pub mod diagnostic;
pub mod highlight;

use crate::language::comprehension::spec::{description::Description, diagnostic::Diagnostic, highlight::Highlight};

pub trait ComprehensionSpec {
    type Description: Description;
    type Diagnostic: Diagnostic;
    type Highlight: Highlight;
}