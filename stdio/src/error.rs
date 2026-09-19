use std::fmt;
use tokio::io::Error;

#[derive(Debug)]
pub enum StdioErrorKind {
    Stdin,
    Stdout,
}

#[derive(Debug)]
pub struct StdioError {
    text: String,
    source: Error,
}

impl StdioError {
    pub fn new(text: &str, source: Error) -> Self {
        return Self {
            text: text.to_string(),
            source,
        };
    }
}

impl std::error::Error for StdioError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl fmt::Display for StdioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StdioError: {}\n", self.text)
    }
}

