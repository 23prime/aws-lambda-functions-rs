use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GokabotLambdaError {
    content: String,
}

impl Display for GokabotLambdaError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "content = [{}]", self.content);
    }
}

impl Error for GokabotLambdaError {}

pub fn from(str: String) -> GokabotLambdaError {
    return GokabotLambdaError { content: str };
}
