use super::DownloadError;
use super::Scheme;
use bytes::{BufMut, BytesMut};
use reqwest::{self, header};
use std::fmt::{self, Display};
use std::path::PathBuf;
use reqwest::blocking::Client;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Http{
    url: String,
    client: Client
}

impl Http {
    pub fn new(url: String, client: Client) -> Http {
        Http { url, client }
    }
}

impl Display for Http {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("URL: {}", self.url))
    }
}

impl<C: Into<Client>, S: Into<String>> From<(S, C)> for Http {
    fn from(v: (S, C)) -> Self {
        Self::new(v.0.into(), v.1.into())
    }
}

impl Scheme for Http {
    fn get_length(&self) -> Result<usize, DownloadError> {
        let res = match self.client
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

    fn download(&self, start: usize, end: usize) -> Result<BytesMut, DownloadError> {
        let mut buf = BytesMut::with_capacity(end - start);

        let res = match self.client
            .get(&self.url)
            .header(header::RANGE, &format!("bytes={}-{}", start, end))
            .send()
        {
            Ok(value) => value,
            _ => return Err(DownloadError::ConnectionError),
        };

        buf.put(match res.bytes() {
            Ok(value) => value,
            Err(err) => {
                println!("err: {}", err);
                return Err(DownloadError::DownlaodingError);
            }
        });

        Ok(buf)
    }

    fn get_file_name(&self) -> String {
        let path = PathBuf::from(&self.url);
        format!("{}", path.file_name().unwrap().to_str().unwrap())
    }
}
