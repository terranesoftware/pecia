pub mod description;
pub mod diagnostic;
pub mod highlight;

use crate::traits::comprehension::{description::Description, diagnostic::Diagnostic, highlight::Highlight};

pub trait Comprehension {
    type Description: Description;
    type Diagnostic: Diagnostic;
    type Highlight: Highlight;
}