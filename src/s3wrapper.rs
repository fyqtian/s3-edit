// https://github.com/awsdocs/aws-doc-sdk-examples
use crate::helper;
use anyhow::Result;
use anyhow::{anyhow, Context};

use aws_sdk_s3::Client;
use std::error::Error;
pub struct S3Wrapper {
    client: Client,
}
impl S3Wrapper {
    pub async fn new() -> Self {
        let config = helper::aws_config().await;
        let client = Client::new(&config);
        S3Wrapper { client }
    }
    pub async fn get_object(
        &self,
        url: &str,
    ) -> Result<aws_sdk_s3::operation::get_object::GetObjectOutput> {
        let location = S3Location::new_from_url(url)?;

        let obj = self
            .client
            .get_object()
            .bucket(location.bucket)
            .key(location.key)
            .send()
            .await?;
        Ok(obj)
    }
}

#[derive(Debug)]
pub struct S3Location {
    bucket: String,
    key: String,
}

impl S3Location {
    fn new(bucket: String, key: String) -> Self {
        S3Location { bucket, key }
    }
    fn new_from_url(url: &str) -> Result<Self> {
        parse_s3_url(url).context("passed invalid s3 url")
    }
}

impl Into<String> for S3Location {
    fn into(self) -> String {
        format!("s3://{}/{}", self.bucket, self.key)
    }
}

impl From<String> for S3Location {
    fn from(url: String) -> Self {
        parse_s3_url(&url).unwrap()
    }
}

fn parse_s3_url(url: &str) -> Result<S3Location> {
    if !url.starts_with("s3://") {
        return Err(anyhow!("Invalid s3 url, must start with s3://"));
    }

    let raw_url = url.trim_start_matches("s3://");
    let parts: Vec<&str> = raw_url.split('/').collect();

    if parts.len() != 2 {
        return Err(anyhow!("Invalid s3 url, must have exactly full path"));
    }
    Ok(S3Location {
        bucket: parts[0].to_string(),
        key: parts[1].to_string(),
    })
}
