use std::fmt;

#[derive(Debug)]
pub enum AnalyzeError {
    Other(String),
}

pub type AnalyzeResult<T> = Result<T, AnalyzeError>;

impl fmt::Display for AnalyzeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalyzeError::Other(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for AnalyzeError {}

impl From<String> for AnalyzeError {
    fn from(s: String) -> AnalyzeError {
        AnalyzeError::Other(s)
    }
}

impl From<&'static str> for AnalyzeError {
    fn from(s: &'static str) -> AnalyzeError {
        AnalyzeError::Other(s.to_string())
    }
}
