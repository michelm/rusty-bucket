use clap::{builder::PossibleValuesParser, Parser};
use std::path::Path;

/// Test application for up- and downloading files to Google Cloud Storage
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File action to be performed: upload or download
    #[arg(short, long, default_value = "upload", value_parser = PossibleValuesParser::new(["ls", "list", "put", "up", "upload", "get", "down", "download"]))]
    action: String,

    /// Bucket name
    #[arg(short, long, default_value = "gcpee-bucket")]
    bucket: String,

    /// Path of local file; source when uploading and destination when downloading
    #[arg(short, long, default_value = "tests/data/lorem-ipsum.txt")]
    local: String,

    /// Path of remote file in the bucket; destination when uploading and source when downloading
    #[arg(short, long, default_value = "data/test.txt")]
    remote: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();

    let action = args.action.as_str();
    let bucket = args.bucket;
    let local = Path::new(args.local.as_str());
    let remote = args.remote;

    println!("action: {}", action);
    println!("bucket: {}", bucket);
    println!("local : {:?}", local);
    println!("remote: {}\n", remote);

    match action {
        "put" | "up" | "upload" => put_file(bucket, local, remote).await,
        "get" | "down" | "download" => get_file(bucket, local, remote).await,
        _ => eprintln!("\x1b[91merror\x1b[0m: action '{}' is not supported", action),
    }
}

async fn put_file(bucket: String, local: &Path, remote: String) {
    println!("cp {:?} -> gs://{}/{}", local, bucket, remote);

    match rusty_bucket::upload(bucket, local, remote).await {
        Ok(size) => println!("\x1b[92mok\x1b[0m: uploaded {} bytes", size),
        Err(e) => eprintln!("\x1b[91mfailed\x1b[0m: {}", e),
    }
}

async fn get_file(bucket: String, local: &Path, remote: String) {
    println!("cp gs://{}/{} -> {:?}", bucket, remote, local);

    match rusty_bucket::download(bucket, local, remote).await {
        Ok(size) => println!("\x1b[92mok\x1b[0m: downloaded {} bytes", size),
        Err(e) => eprintln!("\x1b[91mfailed\x1b[0m: {}", e),
    }
}
