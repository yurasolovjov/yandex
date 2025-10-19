use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    Csv(String),
    Mt940(String),
    InvalidFormat(String),
    Io(std::io::Error),
}

#[derive(Debug)]
pub enum SerializeError {
    Csv(String),
    Xml(String),
    Io( std::io::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           ParseError::Csv(msg) => write!(f, "{}", msg),
           ParseError::Mt940(msg) => write!(f, "{}", msg),
           ParseError::InvalidFormat(msg) => write!(f, "{}", msg),
           ParseError::Io(err) => write!(f, "{}", err),
       }
    }
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializeError::Csv(msg) => write!(f, "{}", msg),
            SerializeError::Xml(msg) => write!(f, "{}", msg),
            SerializeError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl Error for SerializeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl From<std::io::Error> for ParseError  {
    fn from(err: std::io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

impl From<std::io::Error> for SerializeError  {
    fn from(err: std::io::Error) -> SerializeError {
        SerializeError::Io(err)
    }
}