use aws_config::SdkConfig;
use aws_sdk_s3::output::PutObjectOutput;
use aws_sdk_s3::Client;

use crate::{AwsErrorContext, CustomResult};

pub struct S3Helper {
    client: Client,
}

impl From<&SdkConfig> for S3Helper {
    fn from(config: &SdkConfig) -> Self {
        let s3_client = Client::new(config);
        Self { client: s3_client }
    }
}

impl S3Helper {
    #[allow(dead_code)]
    pub async fn new() -> Self {
        Self::from(&aws_config::load_from_env().await)
    }

    pub async fn put_object<'a>(
        &self,
        bucket_name: &str,
        filename: impl AsRef<str>,
        object_bytes: &[u8],
    ) -> CustomResult<PutObjectOutput> {
        self.client
            .put_object()
            .bucket(bucket_name)
            .body(object_bytes.to_owned().into())
            .key(filename.as_ref())
            .content_type("text/plain")
            .send()
            .await
            .map_context(|err| {
                error!(
                    bucket = bucket_name,
                    file = filename.as_ref(),
                    "failed to upload file to S3 with error: {err}"
                );
            })
    }
}
