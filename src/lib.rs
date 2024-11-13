use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use std::error;
use std::fmt;
use std::fs;
use std::path::Path;
use tokio::fs as async_fs;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

async fn get_client() -> Result<Client> {
    let config = ClientConfig::default().with_auth().await?;
    Ok(Client::new(config))
}

pub async fn upload(bucket: String, path: &Path, destination: String) -> Result<usize> {
    let client = get_client().await?;
    let content: Vec<u8> = fs::read(path)?;
    let destination: &'static str = Box::leak(destination.into_boxed_str());

    let object_type = UploadType::Simple(Media::new(destination));
    let object = client
        .upload_object(
            &UploadObjectRequest {
                bucket,
                ..Default::default()
            },
            content,
            &object_type,
        )
        .await?;

    Ok(object.size as usize)
}

pub async fn download(bucket: String, path: &Path, source: String) -> Result<usize> {
    let client = get_client().await?;

    let data = client
        .download_object(
            &GetObjectRequest {
                bucket,
                object: source,
                ..Default::default()
            },
            &Range::default(),
        )
        .await?;

    let size = data.len();

    // Ensure the parent directory exists
    if let Some(parent) = path.parent() {
        async_fs::create_dir_all(parent).await?;
    }

    fs::write(path, data)?;

    Ok(size)
}
