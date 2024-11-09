#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use crate::s3wrapper::S3Location;
use anyhow::anyhow;
use clap::{arg, Arg, Command};
use log::{debug, error, warn};
use std::error::Error;
use std::process;
use tempfile::NamedTempFile;

mod helper;
mod s3wrapper;

#[tokio::main]
async fn main() {
    let s3edit = S3Edit::new().await;
    let rs = s3edit.run().await;
    if rs.is_err() {
        error!("error:{}", rs.unwrap_err());
    }
}

struct S3Edit {
    client: s3wrapper::S3Wrapper,
    s3location: Option<S3Location>,
    editor: Option<String>,
}

impl S3Edit {
    async fn new() -> Self {
        let client = s3wrapper::S3Wrapper::new().await;
        S3Edit {
            client: client,
            s3location: None,
            editor: None,
        }
    }
    fn init_command(&self) -> Command {
        let cmd = Command::new("s3-edit")
            .author("vanessa")
            .version("0.1.0")
            .about("A tool to edit s3 object")
            .arg(Arg::new("s3-url").short('u').long("s3-url").required(true))
            .arg(
                Arg::new("editor")
                    .short('e')
                    .long("editor")
                    .default_value("vi"),
            );
        cmd
    }
    async fn run(&self) -> anyhow::Result<()> {
        let matches = self.init_command().get_matches();

        let url = matches
            .get_one::<String>("s3-url")
            .ok_or(anyhow!("s3-url is required"))?;
        let editor = matches.get_one::<String>("editor").unwrap();
        debug!("url:{} ,editor:{}", url, editor);
        if !helper::check_command_exist(editor).await {
            return Err(anyhow!("editor {} not found", editor));
        }

        let temp_file = self.download(&url).await?;
        let path = temp_file.path().to_str().unwrap();
        let rs = helper::normal_exec_cmd(editor.as_str(), vec![path]);
        debug!("rs:{:?}", temp_file);
        Ok(())
    }

    async fn download(&self, url: &str) -> anyhow::Result<NamedTempFile> {
        let rs = self.client.get_object_to_temp_file(url).await?;
        Ok(rs)
    }
}
