use std::fmt;

pub mod client;
pub mod util;

#[derive(Debug, Clone)]
pub enum CFToolError {
    FailedRequest,
}

impl fmt::Display for CFToolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: More output
        write!(f, "CFToolError")
    }
}
