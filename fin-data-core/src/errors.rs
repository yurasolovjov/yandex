use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("CSV parse error: {0}")]
    Csv(#[from] csv::Error),
    #[error("MT940 parse error: {0}")]
    Mt940(String),
    #[error("Json parse error: {0}")]
    Camt053(#[from] serde_json::Error),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum SerializeError {
    #[error("CSV serialize error: {0}")]
    Csv(#[from] csv::Error),
    #[error("JSON serialize error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}