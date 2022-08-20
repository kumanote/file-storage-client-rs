use crate::Result;
use futures::{Stream, StreamExt, TryStreamExt};
use std::io::Write;
use std::path::PathBuf;
use tokio::fs::File;

pub(crate) fn put(local_filepath: &PathBuf, to_directory: &PathBuf, filename: &str) -> Result<()> {
    let copy_to = to_directory.join(filename);
    if let Some(parent_dir) = copy_to.parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)?;
        }
    }
    std::fs::copy(local_filepath, &copy_to)?;
    Ok(())
}

pub(crate) fn put_data(data: Vec<u8>, to_directory: &PathBuf, filename: &str) -> Result<()> {
    let destination = to_directory.join(filename);
    if let Some(parent_dir) = destination.parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)?;
        }
    }
    // write data to target
    let mut file = std::fs::File::create(&destination)?;
    file.write_all(data.as_slice())?;
    Ok(())
}

pub(crate) fn get(directory: &PathBuf, filename: &str) -> PathBuf {
    directory.join(filename)
}

pub(crate) async fn get_data(
    directory: &PathBuf,
    filename: &str,
) -> Result<impl Stream<Item = Result<u8>> + Unpin> {
    let local_filepath = directory.join(filename);
    let file = File::open(&local_filepath).await?;
    let bytes_stream = tokio_util::io::ReaderStream::new(file);
    Ok(bytes_stream
        .map(|chunk| chunk.map(|c| futures_util::stream::iter(c.into_iter().map(Ok))))
        .try_flatten())
}
