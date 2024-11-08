use clap::arg;
use std::env::args;
use std::ffi::OsStr;
use std::fmt::{Debug, Display};
use std::io;
use std::process::{Command, ExitStatus, Output};
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
    let temp_file = tempfile::NamedTempFile::new()?;
    Ok(temp_file)
}

pub async fn exec_cmd<T: AsRef<OsStr>>(cmd: T, args: Vec<T>) -> io::Result<Output> {
    let output = tokio::process::Command::new(cmd)
        .args(args)
        .output()
        .await?;
    Ok(output)
}

pub async fn check_command_exist(cmd: &str) -> bool {
    let output = exec_cmd("which", vec![cmd]).await;
    if output.is_err() {
        return false;
    }
    output.unwrap().status.success()
}

pub fn normal_exec_cmd<T: AsRef<OsStr>>(cmd: T, args: Vec<T>) -> io::Result<Output> {
    for i in args.iter() {
        println!("arg:{}", i.as_ref().to_str().unwrap());
    }

    let r = Command::new("sh").args(args).output()?;
    Ok(r)
}
