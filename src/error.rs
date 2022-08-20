use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IOError: {cause:?}")]
    IOError { cause: std::io::Error },
    #[error("CloudStorageError: {cause:?}")]
    CloudStorageError { cause: cloud_storage::Error },
}

impl From<std::io::Error> for Error {
    fn from(cause: std::io::Error) -> Self {
        Self::IOError { cause }
    }
}

impl From<cloud_storage::Error> for Error {
    fn from(cause: cloud_storage::Error) -> Self {
        Self::CloudStorageError { cause }
    }
}
