/// Options that are passed to the [`crate::tokenizer::Tokenizer`].
/// This set of rules configures the shape of the resulting output
/// string.
pub struct FormattingOptions {
    /// Will be appended to each token expect for the last.
    pub separator: Option<String>,
    /// Formats each generated token in the specified [`Case`].
    pub case: Option<Case>,
}

pub enum Case {
    Lower,
    Upper,
    //TODO: snake case, kebab case etc.
}

impl FormattingOptions {
    /// For now the current iteration value `i` and the number
    /// of total iterations of the run `t` are passed into the
    /// formatting function but not used. This is done, because
    /// some formatting rules might depend on this information.
    pub fn apply_formatting(&self, s: &mut String, _i: u32, _t: u32) {
        if let Some(case) = &self.case {
            match case {
                Case::Lower => *s = s.to_lowercase(),
                Case::Upper => *s = s.to_uppercase(),
            }
        }
    }
}
