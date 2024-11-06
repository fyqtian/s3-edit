pub async fn aws_config() -> aws_config::SdkConfig {
    let config = aws_config::from_env().load().await;
    config
}

pub fn exit_with_error(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}