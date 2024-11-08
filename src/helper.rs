use std::env::args;
use std::ffi::OsStr;
use std::fmt::{Debug, Display};
use std::io;
use std::process::Output;
use tempfile::NamedTempFile;

pub async fn aws_config() -> aws_config::SdkConfig {
    let config = aws_config::from_env().load().await;
    config
}

pub fn exit_with_error(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}

pub async fn create_temp_file() -> io::Result<NamedTempFile> {
    let mut temp_file = tempfile::NamedTempFile::new()?;
    Ok(temp_file)
}

pub async fn exec_cmd<T: AsRef<OsStr>>(cmd: &str, args: Vec<T>) -> io::Result<Output> {
    let output = tokio::process::Command::new(cmd)
        .args(args)
        .output()
        .await?;
    Ok(output)
}
