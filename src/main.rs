use clap::Parser;
use std::path::Path;

/// Test application for up- and downloading files to Google Cloud Storage
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Bucket name
    #[arg(short, long, default_value = "gcpee-bucket")]
    bucket: String,

    /// Path of local file; source when uploading and destination when downloading
    #[arg(short, long)]
    local: String,

    /// Path of remote file in the bucket; destination when uploading and source when downloading
    #[arg(short, long)]
    remote: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    let bucket = args.bucket;
    let local = Path::new(args.local.as_str());
    let remote: &'static str = Box::leak(args.remote.into_boxed_str());
    println!("cp {:?} -> gs://{}/{}", local, bucket, remote);

    match rusty_bucket::upload(bucket, local, remote).await {
        Ok(size) => println!("\x1b[92mok\x1b[0m: uploaded {} bytes", size),
        Err(e) => eprintln!("\x1b[91mfailed\x1b[0m: {}", e),
    }
}
