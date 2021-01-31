use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SearchError {
    details: String,
}

impl SearchError {
    pub fn new(details: &str) -> SearchError {
        SearchError {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SearchError {
    fn description(&self) -> &str {
        &self.details
    }
}
