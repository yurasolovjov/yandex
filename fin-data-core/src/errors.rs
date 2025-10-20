use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("CSV parse error: {0}")]
    Csv(#[from] csv::Error),
    #[error("MT940 parse error: {0}")]
    Mt940(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum SerializeError {
    #[error("CSV serialize error: {0}")]
    Csv(#[from] csv::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}