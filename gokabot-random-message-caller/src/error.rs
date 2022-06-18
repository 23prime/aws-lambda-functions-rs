use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LambdaGeneralError {
    content: Option<String>,
}

impl Display for LambdaGeneralError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return match self.content.clone() {
            Some(inner) => write!(f, "content: {}", inner),
            None => write!(f, "content: None"),
        };
    }
}

impl Error for LambdaGeneralError {}

impl LambdaGeneralError {
    pub fn new(str: String) -> Self {
        return Self { content: Some(str) };
    }

    pub fn none() -> LambdaGeneralError {
        return Self { content: None };
    }
}
