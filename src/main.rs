#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use anyhow::anyhow;
use aws_sdk_s3::types::S3Location;
use clap::builder::Str;
use clap::{arg, Arg, Command};
use inquire::{required, Confirm};
use log::{debug, error, warn};
use std::error::Error;
use std::process;
use tempfile::NamedTempFile;

mod helper;
mod s3wrapper;

#[tokio::main]
async fn main() {
    env_logger::init();
    let s3edit = S3Edit::new().await;
    let rs = s3edit.run().await;
    if rs.is_err() {
        error!("error:{}", rs.unwrap_err());
    }
}

struct S3Edit {
    client: s3wrapper::S3Wrapper,
    url: String,
    editor: String,
}

fn init_command() -> Command {
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
        )
        .arg(Arg::new("region").short('r').long("region"));
    cmd
}

impl S3Edit {
    async fn new() -> Self {
        let matches = init_command().get_matches();
        let region = matches.get_one::<String>("region").map(|x| x.clone());
        let editor = matches.get_one::<String>("editor").unwrap().clone();
        let url = matches.get_one::<String>("s3-url").unwrap().clone();
        let client = s3wrapper::S3Wrapper::new(region).await;
        S3Edit {
            client,
            url,
            editor,
        }
    }
    async fn run(&self) -> anyhow::Result<()> {
        if !helper::check_command_exist(self.editor.as_str()).await {
            return Err(anyhow!("editor {} not found", self.editor));
        }

        let temp_file = self.download(self.url.as_str()).await?;
        let path = temp_file.path().to_str().unwrap();
        let path_edited = format!("{}.edited", path);
        let cp_rs = helper::exec_cmd_sucess("cp", vec![path, path_edited.as_str()]).await;
        if !cp_rs {
            return Err(anyhow!("copy failed"));
        }
        let git_exist = helper::check_command_exist("git").await;
        loop {
            let rs = helper::normal_exec_cmd(self.editor.as_str(), vec![path_edited.as_str()]);
            let ans = helper::answer_confirm("Do you finish editing?", true);
            if !ans {
                continue;
            }
            if git_exist {
                let git_diff = helper::exec_cmd(
                    "git",
                    vec![
                        "diff",
                        "--ignore-space-change",
                        "--color",
                        path,
                        path_edited.as_str(),
                    ],
                )
                .await?;
                println!("git diff:{}", String::from_utf8(git_diff.stdout)?);
                let ans = helper::answer_confirm("Confirm your edits and commit?", true);
                if ans {
                    break;
                }
            }
        }
        self.client
            .put_object_from_file(self.url.as_str(), &path_edited)
            .await?;
        Ok(())
    }

    async fn download(&self, url: &str) -> anyhow::Result<NamedTempFile> {
        let rs = self.client.get_object_to_temp_file(url).await?;
        Ok(rs)
    }
}
