use tokio::io;

#[derive(Debug)]
pub struct Error {
    source: io::Error,
    text: String,
}

impl Error {
    pub fn new(text: &str, source: io::Error) -> Self {
        return Self {
            text: text.to_string(),
            source,
        };
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}\n", self.text)
    }
}
