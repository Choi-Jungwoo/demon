use super::DownloadError;
use super::Scheme;
use bytes::{BufMut, BytesMut};
use reqwest::{self, header};
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct Http {
    url: String,
}

impl Http {
    pub fn new(url: String) -> Http {
        Http { url }
    }
}

impl From<&str> for Http {
    fn from(value: &str) -> Self {
        Http {
            url: value.to_string(),
        }
    }
}

impl Display for Http {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("URL: {}", self.url))
    }
}

impl Scheme for Http {
    fn get_length(&self) -> Result<usize, DownloadError> {
        let client = reqwest::blocking::Client::new();
        let res = match client
            .get(&self.url)
            .header(header::RANGE, "bytes=0-0")
            .send()
        {
            Ok(value) => value,
            _ => return Err(DownloadError::ConnectionError),
        };

        let range = match res.headers().get("content-range") {
            Some(value) => value,
            _ => return Err(DownloadError::GetLengthError),
        };

        let length = match range.to_str() {
            Ok(value) => match value.split("/").collect::<Vec<&str>>()[1].parse::<usize>() {
                Ok(value) => value,
                _ => return Err(DownloadError::GetLengthError),
            },
            _ => return Err(DownloadError::GetLengthError),
        };

        Ok(length)
    }

    fn download(&self, buf: &mut BytesMut, start: usize, end: usize) -> Result<(), DownloadError> {
        let client = reqwest::blocking::Client::new();
        let res = match client
            .get(&self.url)
            .header(header::RANGE, &format!("bytes={}-{}", start, end))
            .send()
        {
            Ok(value) => value,
            _ => return Err(DownloadError::ConnectionError),
        };

        buf.put(match res.bytes() {
            Ok(value) => value,
            _ => return Err(DownloadError::DownlaodingError),
        });

        Ok(())
    }
}
