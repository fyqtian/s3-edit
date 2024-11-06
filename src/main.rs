use clap::{Arg, Command, Parser};
use std::error::Error;
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

    let rs = client.get_object_body(url).await;
    if rs.is_err() {
        let m = rs.err().unwrap();
        helper::exit_with_error("download object failed");
    }
    let body = rs.unwrap();
    println!("{}", String::from_utf8(body).unwrap());
}

fn init_command() -> Command {
    let cmd = Command::new("s3-edit")
        .author("vanessa")
        .version("0.1.0")
        .about("A tool to edit s3 object")
        .arg(Arg::new("s3-url").short('u').long("s3-url").required(true));
    cmd
}
