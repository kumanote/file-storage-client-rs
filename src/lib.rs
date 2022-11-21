mod error;
mod google_could_storage;
mod local_storage;

use futures::Stream;
use std::path::{Path, PathBuf};
use std::pin::Pin;

pub use error::Error;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum FileStorageClient {
    LocalStorage {
        directory: PathBuf,
    },
    GoogleCloudStorage {
        bucket_name: String,
        download_dir: PathBuf,
    },
}

impl FileStorageClient {
    pub fn new_local_storage<P: AsRef<Path>>(directory: P) -> Self {
        Self::LocalStorage {
            directory: directory.as_ref().to_path_buf(),
        }
    }

    pub fn new_google_cloud_storage<S: Into<String>, P: AsRef<Path>>(
        bucket_name: S,
        download_dir: P,
    ) -> Self {
        Self::GoogleCloudStorage {
            bucket_name: bucket_name.into(),
            download_dir: download_dir.as_ref().to_path_buf(),
        }
    }

    pub async fn put(
        &self,
        local_filepath: &PathBuf,
        filename: &str,
        mime_type: Option<&str>,
    ) -> Result<()> {
        match self {
            Self::LocalStorage { directory } => {
                local_storage::put(local_filepath, directory, filename)
            }
            Self::GoogleCloudStorage {
                bucket_name,
                download_dir: _,
            } => {
                google_could_storage::put(local_filepath, bucket_name.as_str(), filename, mime_type)
                    .await
            }
        }
    }

    pub async fn put_data(
        &self,
        data: Vec<u8>,
        filename: &str,
        mime_type: Option<&str>,
    ) -> Result<()> {
        match self {
            Self::LocalStorage { directory } => local_storage::put_data(data, directory, filename),
            Self::GoogleCloudStorage {
                bucket_name,
                download_dir: _,
            } => {
                google_could_storage::put_data(data, bucket_name.as_str(), filename, mime_type)
                    .await
            }
        }
    }

    pub async fn get(&self, filename: &str) -> Result<PathBuf> {
        match self {
            Self::LocalStorage { directory } => Ok(local_storage::get(directory, filename)),
            Self::GoogleCloudStorage {
                bucket_name,
                download_dir,
            } => google_could_storage::get(bucket_name.as_str(), filename, download_dir).await,
        }
    }

    pub async fn get_data(
        &self,
        filename: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<u8>> + Unpin>>> {
        match self {
            Self::LocalStorage { directory } => {
                let stream = local_storage::get_data(directory, filename).await?;
                Ok(Box::pin(stream))
            }
            Self::GoogleCloudStorage {
                bucket_name,
                download_dir: _,
            } => {
                let stream = google_could_storage::get_data(bucket_name.as_str(), filename).await?;
                Ok(Box::pin(stream))
            }
        }
    }
}
