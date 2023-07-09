use std::fmt;

/// An Error type for the dhcp lib.
#[derive(Debug)]
pub enum DhcpError {
    ParsingError(String),
}

impl fmt::Display for DhcpError {
    /// Display a DhcpError.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DhcpError::ParsingError(message) => write!(f, "Parsing Error: {}", message),
        }
    }
}

impl std::error::Error for DhcpError {}
