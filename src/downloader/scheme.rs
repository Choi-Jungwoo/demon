pub mod http;

use super::errors::DownloadError;
use bytes::BytesMut;
use std::fmt::{Debug, Display};

pub trait Scheme: Debug + Display + Clone + Send + Sync {
    fn get_length(&self) -> Result<usize, DownloadError>;

    fn download(&self, start: usize, end: usize) -> Result<BytesMut, DownloadError>;

    fn get_file_name(&self) -> String;
}
