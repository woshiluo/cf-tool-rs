use std::fmt;

pub mod client;
pub mod config;
pub mod util;

#[derive(Debug, Clone)]
pub enum CFToolError {
    FailedRequest,
    FailedParseRespone,
    FailedTerminalOutput,
    WrongRespone(u16),
    NotLogin,
}

impl fmt::Display for CFToolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: More output
        write!(f, "CFToolError")
    }
}

impl From<crossterm::ErrorKind> for CFToolError {
    fn from(_: crossterm::ErrorKind) -> CFToolError {
        CFToolError::FailedTerminalOutput
    }
}
