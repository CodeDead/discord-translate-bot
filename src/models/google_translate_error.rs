use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct GoogleTranslateError {
    pub message: String,
}

impl GoogleTranslateError {
    /// Create a new GoogleTranslateError instance
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    ///
    /// # Returns
    ///
    /// A new GoogleTranslateError instance
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for GoogleTranslateError {
    /// Display the error message
    ///
    /// # Arguments
    ///
    /// * `f` - Formatter
    ///
    /// # Returns
    ///
    /// A formatted string
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GoogleTranslateError {}
