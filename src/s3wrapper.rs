// https://github.com/awsdocs/aws-doc-sdk-examples
use crate::helper;


use aws_sdk_s3::Client;
use std::error::Error;
pub struct s3_wrapper {
    client: Client,
}
impl s3_wrapper {
    pub async fn new() -> Self {
        let config = helper::aws_config().await;
        let client = Client::new(&config);
        s3_wrapper { client }
    }
}
