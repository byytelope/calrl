#[derive(Debug)]
pub enum CalrlError {
    // TODO: Handle direct in
    // InputError,
    UnexpectedChar(char, usize),
}

impl std::fmt::Display for CalrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // CalrlError::InputError => write!(f, "Error while reading input..."),
            CalrlError::UnexpectedChar(c, s) => write!(f, "Unexpected character read: {c}@{s}"),
        }
    }
}
