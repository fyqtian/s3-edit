#[allow(dead_code)]
// https://github.com/awsdocs/aws-doc-sdk-examples
use crate::helper;
use anyhow::Result;
use anyhow::{anyhow, Context};
use aws_sdk_s3::Client;
use reqwest::Proxy;
use std::fs::File;
use std::io::{Read, Write};
use tempfile::NamedTempFile;

pub struct S3Wrapper {
    client: Client,
}
impl S3Wrapper {
    pub async fn new(region: Option<String>) -> Self {
        let config = helper::aws_config(region).await;
        let client = Client::new(&config);
        S3Wrapper { client }
    }

    pub async fn get_object(
        &self,
        url: &str,
    ) -> Result<aws_sdk_s3::operation::get_object::GetObjectOutput> {
        let location = parse_s3_url(url)?;
        let obj = self
            .client
            .get_object()
            .bucket(location.bucket)
            .key(location.key)
            .send()
            .await?;
        Ok(obj)
    }
    pub async fn get_object_body(&self, url: &str) -> Result<Vec<u8>> {
        let mut object = self.get_object(url).await?;
        let mut store: Vec<u8> = vec![];
        while let Some(chunk) = object.body.try_next().await? {
            let tmp: Vec<u8> = chunk.clone().into();
            store.extend(tmp);
        }
        Ok(store)
    }
    pub async fn get_object_to_file(&self, url: &str, file: &mut File) -> Result<()> {
        let mut object = self.get_object(url).await?;
        while let Some(chunk) = object.body.try_next().await? {
            file.write_all(&chunk).context("failed to write to file")?;
        }
        Ok(())
    }
    pub async fn get_object_to_temp_file(&self, url: &str) -> Result<NamedTempFile> {
        let mut temp_file = helper::create_temp_file().await?;
        let f = self
            .get_object_to_file(url, temp_file.as_file_mut())
            .await?;
        Ok(temp_file)
    }
    pub async fn put_object_from_file(&self, url: &str, path: &str) -> Result<()> {
        let location = parse_s3_url(url)?;
        let buffer = helper::read_file(path).await?;
        self.client
            .put_object()
            .bucket(location.bucket)
            .key(location.key)
            .body(buffer.into())
            .send()
            .await?;
        Ok(())
    }
}

fn parse_s3_url(url: &str) -> Result<S3Location> {
    if !url.starts_with("s3://") {
        return Err(anyhow!("Invalid s3 url, must start with s3://"));
    }

    let raw_url = url.trim_start_matches("s3://");
    let parts: Vec<&str> = raw_url.split('/').collect();

    if parts.len() != 2 {
        return Err(anyhow!(
            "Invalid s3 url, must have exactly full path example s3://bucket/key"
        ));
    }
    Ok(S3Location::new(parts[0].to_string(), parts[1].to_string()))
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
