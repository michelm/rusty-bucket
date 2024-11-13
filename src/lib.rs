use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::delete::DeleteObjectRequest;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::list::ListObjectsRequest;
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

/// Upload a file to a Google Cloud Storage bucket and returns the size of the uploaded file in bytes
///
/// Arguments:
///
/// - bucket: Name of the google cloud storage (bucket)
/// - source: File name and path to be uploaded
/// - destination: File name and path in the bucket
///
pub async fn upload(bucket: String, source: &Path, destination: String) -> Result<usize> {
    let client = get_client().await?;
    let content: Vec<u8> = fs::read(source)?;
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

/// Download a file from a Google Cloud Storage bucket and returns the size of the downloaded file in bytes
///
/// Arguments:
///
/// - bucket: Name of the google cloud storage (bucket)
/// - destination: File name and path where downloaded file will be saved
/// - source: File name and path in the bucket to be downloaded
///
pub async fn download(bucket: String, destination: &Path, source: String) -> Result<usize> {
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
    if let Some(parent) = destination.parent() {
        async_fs::create_dir_all(parent).await?;
    }

    fs::write(destination, data)?;

    Ok(size)
}

/// Delete a file(object) from a Google Cloud Storage bucket
///
/// Arguments:
///
/// - bucket: Name of the google cloud storage (bucket)
/// - object: File name and path in the bucket to be deleted
///
pub async fn delete(bucket: String, object: String) -> Result<()> {
    let client = get_client().await?;

    client
        .delete_object(&DeleteObjectRequest {
            bucket,
            object,
            ..Default::default()
        })
        .await?;

    Ok(())
}

/// List files(objects) in a Google Cloud Storage bucket
///
/// Arguments:
///
/// - bucket: Name of the google cloud storage (bucket)
///
/// Returns:
/// - A vector of file names in the bucket
///
pub async fn list_objects(bucket: String) -> Result<Vec<String>> {
    let client = get_client().await?;

    let objects = client
        .list_objects(&ListObjectsRequest {
            bucket,
            ..Default::default()
        })
        .await?;

    let mut result: Vec<String> = Vec::new();

    if let Some(items) = objects.items {
        for object in items {
            result.push(object.name.clone());
        }
    }

    Ok(result)
}
