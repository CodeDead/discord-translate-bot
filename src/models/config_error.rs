use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ConfigError {
    pub message: String,
}

impl ConfigError {
    /// Create a new ConfigError instance
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    ///
    /// # Returns
    ///
    /// A new ConfigError instance
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for ConfigError {
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

impl Error for ConfigError {}
