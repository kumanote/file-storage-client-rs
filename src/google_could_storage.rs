use crate::Result;
use cloud_storage::Client;
use futures::{Stream, StreamExt};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use tokio::fs::File;

const DEFAULT_MIME_TYPE: &'static str = "application/octet-stream";

pub(crate) async fn put(
    local_filepath: &PathBuf,
    bucket_name: &str,
    filename: &str,
    mime_type: Option<&str>,
) -> Result<()> {
    let client = Client::default();
    let file = File::open(local_filepath).await?;
    let stream = tokio_util::io::ReaderStream::new(file);
    let _object = client
        .object()
        .create_streamed(
            bucket_name,
            stream,
            None,
            filename,
            mime_type.unwrap_or(DEFAULT_MIME_TYPE),
        )
        .await?;
    Ok(())
}

pub(crate) async fn put_data(
    data: Vec<u8>,
    bucket_name: &str,
    filename: &str,
    mime_type: Option<&str>,
) -> Result<()> {
    let client = Client::default();
    let _object = client
        .object()
        .create(
            bucket_name,
            data,
            filename,
            mime_type.unwrap_or(DEFAULT_MIME_TYPE),
        )
        .await?;
    Ok(())
}

pub(crate) async fn get(
    bucket_name: &str,
    filename: &str,
    download_dir: &PathBuf,
) -> Result<PathBuf> {
    let client = Client::default();
    let mut stream = client
        .object()
        .download_streamed(bucket_name, filename)
        .await?;
    let download_filepath = PathBuf::from(download_dir);
    let download_filepath = download_filepath.join(filename);
    let mut file = BufWriter::new(std::fs::File::create(&download_filepath).unwrap());
    while let Some(byte) = stream.next().await {
        file.write_all(&[byte.unwrap()]).unwrap();
    }
    Ok(download_filepath)
}

pub(crate) async fn get_data(
    bucket_name: &str,
    filename: &str,
) -> Result<impl Stream<Item = Result<u8>> + Unpin> {
    let client = Client::default();
    let stream = client
        .object()
        .download_streamed(bucket_name, filename)
        .await?;
    Ok(stream.map(|item| item.map_err(Into::into)))
}
