use clap::arg;
use inquire::Confirm;
use log::debug;
use std::env::args;
use std::ffi::OsStr;
use std::fmt::{Debug, Display};
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::{Command, ExitCode, ExitStatus, Output};
use tempfile::NamedTempFile;
use tokio::io::AsyncReadExt;

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

pub async fn exec_cmd_sucess<T: AsRef<OsStr>>(cmd: T, args: Vec<T>) -> bool {
    let output = exec_cmd(cmd, args).await;
    if output.is_err() {
        return false;
    }
    output.unwrap().status.success()
}
pub async fn check_command_exist(cmd: &str) -> bool {
    let output = exec_cmd("which", vec![cmd]).await;
    if output.is_err() {
        return false;
    }
    output.unwrap().status.success()
}

pub fn normal_exec_cmd<T: AsRef<OsStr>>(cmd: T, args: Vec<T>) -> io::Result<ExitStatus> {
    let mut exec_cmd = Command::new(cmd);
    exec_cmd.args(args);
    debug!("exec_cmd: {:?}", exec_cmd);
    let r = exec_cmd.status()?;
    Ok(r)
}

pub fn answer_confirm(msg: &str, exit: bool) -> bool {
    let ans = Confirm::new(msg).with_default(false).prompt();
    if ans.is_err() && exit {
        println!("{}", "");
        exit_with_error("ctrl-c exit");
    }
    ans.unwrap()
}

pub async fn read_file<T: AsRef<Path>>(path: T) -> io::Result<Vec<u8>> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(buffer)
}
