use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LambdaGeneralError {
    content: String,
}

impl Display for LambdaGeneralError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "content = [{}]", self.content);
    }
}

impl Error for LambdaGeneralError {}

impl LambdaGeneralError {
    pub fn new(str: String) -> LambdaGeneralError {
        return LambdaGeneralError { content: str };
    }
}
