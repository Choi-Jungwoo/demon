use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum DownloadError {
    ConnectionError,
    GetLengthError,
    DownlaodingError,
}

impl Display for DownloadError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let display = match self {
            DownloadError::ConnectionError => "Connection error.".to_string(),
            DownloadError::GetLengthError => "Get length error.".to_string(),
            DownloadError::DownlaodingError => "Downlaoding error.".to_string(),
        };

        fmt.write_str(&display)
    }
}

impl Error for DownloadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DownloadError::ConnectionError => None,
            DownloadError::GetLengthError => None,
            DownloadError::DownlaodingError => None,
        }
    }
}
