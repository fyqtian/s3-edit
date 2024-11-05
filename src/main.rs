use clap::{Arg, Command, Parser};
use std::error::Error;
mod helper;
mod s3wrapper;

#[tokio::main]

async fn main() {
    run().await
}

async fn run() {
    let cmd = init_command();
    let matches = cmd.get_matches();
    let url = matches.get_one::<String>("s3-url").unwrap();
    if let Ok(s3_location) = parse_s3_url(&url) {
        println!("bucket: {}, key: {}", s3_location.bucket, s3_location.key);
    } else {
        println!("Invalid s3 url");
        return;
    }
    let client = s3wrapper::s3_wrapper::new().await;
}
#[derive(Debug)]
struct s3_location {
    bucket: String,
    key: String,
}

impl s3_location {
    fn new(bucket: String, key: String) -> Self {
        s3_location { bucket, key }
    }
}

impl Into<String> for s3_location {
    fn into(self) -> String {
        format!("s3://{}/{}", self.bucket, self.key)
    }
}

impl From<String> for s3_location {
    fn from(url: String) -> Self {
        parse_s3_url(&url).unwrap()
    }
}

fn parse_s3_url(url: &str) -> Result<s3_location, String> {
    let err = "Invalid s3 url".to_string();
    if !url.starts_with("s3://") {
        return Err(err);
    }
    let raw_url = url.trim_start_matches("s3://");
    let parts: Vec<&str> = raw_url.split('/').collect();
    if parts.len() != 2 {
        return Err(err);
    }
    Ok(s3_location {
        bucket: parts[0].to_string(),
        key: parts[1].to_string(),
    })
}

fn init_command() -> Command {
    let cmd = Command::new("s3-edit")
        .author("vanessa")
        .version("0.1.0")
        .about("A tool to edit s3 object")
        .arg(Arg::new("s3-url").short('u').long("s3-url").required(true));
    cmd
}
