# file-storage-client-rs

> Rust implemented simple file storage client.

## Features

- Local Storage i.e. Local attached disk / NFS(Network File System)
  - save file
  - read file
    - read stream
    - return file path
- Google Cloud Storage
  - put object
  - get object
    - read object stream
    - download object and return the downloaded file path
- AWS S3 (**TODO - NOT SUPPORTED**)

## Prerequisite

- [Rust with Cargo](http://rust-lang.org)
  - There is no specific `MSRV(Minimum Supported Rust Version)`
  - Only tested with the latest stable version Rust compiler (older/nightly builds may work...)

## Usage

**Cargo.toml**

Please install this packages as follows.

```toml
[dependencies]
file-storage-client = { version = "0.1.0", git = "https://github.com/kumanote/file-storage-client-rs", branch = "main" }
```

**rust files**

```rust
use file_storage_client::FileStorageClient;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let file_content = b"This is test.";
  let local_file_storage_dir = {
    let mut result = std::env::current_dir().unwrap();
    result.push("tests");
    result.push("tmp");
    result
  };
  let client = FileStorageClient::new_local_storage(&local_file_storage_dir);

  // save as ./tmp/test.txt
  client
          .put_data(file_content.to_vec(), "test.txt", None)
          .await?;

  // read saved file content
  let saved_file_path = {
    let mut result = local_file_storage_dir.clone();
    result.push("test.txt");
    result
  };
  let save_filename = saved_file_path.into_os_string().into_string().unwrap();
  let mut stream = client.get_data(&save_filename).await?; // you can map this stream and then hand it to http response steam.
  let mut content = vec![];
  while let Some(Ok(b)) = stream.next().await {
    content.push(b);
  }
  assert_eq!(content, file_content);
  Ok(())
}
```

### Google Cloud Storage

Authorization can be granted using the `SERVICE_ACCOUNT` or `GOOGLE_APPLICATION_CREDENTIALS` environment variable, 
which should contain path to the `service-account-*******.json` file that contains the Google credentials. 
Alternatively, the service account credentials can be provided as JSON directly through the `SERVICE_ACCOUNT_JSON` 
or `GOOGLE_APPLICATION_CREDENTIALS_JSON` environment variable, which is useful when providing secrets in CI or k8s.

The service account should also have the roles `Service Account Token Creator` (for generating access tokens) and 
`Storage Object Admin` (for generating sign urls to download the files).
