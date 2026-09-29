pub mod diagnostic;
pub mod highlight;
pub mod hover;
pub mod recognition;

use crate::language::comprehension::{diagnostic::Diagnostic, highlight::Highlight, hover::Hover, recognition::Recognition};

pub struct Comprehension {
    diagnostics: Vec<Diagnostic>,
    highlights: Vec<Highlight>,
    hovers: Vec<Hover>,
    recognitions: Vec<Recognition>
}