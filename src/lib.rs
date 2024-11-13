use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use std::error;
use std::fmt;
use std::fs;
use std::path::Path;

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

pub async fn upload(bucket: String, path: &Path, destination: &'static str) -> Result<i64> {
    // Get the google storage client
    let client = get_client().await?;

    // Open the file and read its contents
    let content: Vec<u8> = fs::read(path)?;

    // Upload the file
    let upload_type = UploadType::Simple(Media::new(destination));
    let uploaded = client
        .upload_object(
            &UploadObjectRequest {
                bucket,
                ..Default::default()
            },
            content,
            &upload_type,
        )
        .await?;

    Ok(uploaded.size)
}
