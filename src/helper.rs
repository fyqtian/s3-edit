use anyhow::anyhow;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io;
use std::path::Path;
use tempfile::NamedTempFile;

pub async fn aws_config() -> aws_config::SdkConfig {
    let config = aws_config::from_env().load().await;
    config
}

pub fn exit_with_error(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}

pub async fn create_temp_file() -> io::Result<File> {
    let mut temp_file = tempfile::tempfile()?;
    Ok(temp_file)
}
