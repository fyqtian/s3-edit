use clap::{arg, command, Arg, Command, Parser};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::process::Output;

mod error;
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

    let client = s3wrapper::S3Wrapper::new().await;

    let rs = client.get_object_to_temp_file(url).await;
    if rs.is_err() {
        let m = rs.err().unwrap();
        helper::exit_with_error("download object failed");
    }
    let mut temp_file = rs.unwrap();
    let args = vec![temp_file.path().to_str().unwrap(), "test"];
    let output = helper::exec_cmd("cp", args).await;
    if output.is_err() {
        let m = output.err().unwrap();
        helper::exit_with_error("exec cmd failed");
    }
}

fn init_command() -> Command {
    let cmd = Command::new("s3-edit")
        .author("vanessa")
        .version("0.1.0")
        .about("A tool to edit s3 object")
        .arg(Arg::new("s3-url").short('u').long("s3-url").required(true));
    cmd
}
