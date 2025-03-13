use csv::Error as CsvError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),

    #[error("Reader has no permission to call this method")]
    ReadDataError(String),

    #[error("csv error")]
    CsvError(#[from] CsvError),

    #[error("poly error: {0}")]
    PolyError(String),
}
