use async_trait::async_trait;
use aws_sdk_secretsmanager::model::Tag;
use aws_sdk_secretsmanager::output::TagResourceOutput;
use aws_sdk_secretsmanager::Client;
use aws_types::SdkConfig;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use tokio::join;

use crate::{AwsErrorContext, CustomResult, Secrets};

/// Admin User.
#[derive(Debug, Deserialize)]
pub struct Admin {
    pub username: String,
    pub password: String,
}

/// Dummy credentials.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DummyCredentials {
    pub access_key_id: String,
    pub password: String,
}

impl<'a> Secrets<'a> {
    /// Retrieves the list of secret(s) concurrently, from AWS Secrets Manager.
    pub async fn fetch(
        self,
        shared_config: &'a SdkConfig,
    ) -> CustomResult<(Admin, DummyCredentials)> {
        let (admin_result, creds_result) = join!(
            self.admin_user.get_secret::<Admin>(shared_config),
            self.creds.get_secret::<DummyCredentials>(shared_config),
        );

        Ok((admin_result?, creds_result?))
    }
}

#[async_trait]
pub trait SecretsExt {
    async fn get_secret<T: DeserializeOwned>(self, config: &SdkConfig) -> CustomResult<T>;
    async fn set_tag(
        self,
        config: &SdkConfig,
        key: &str,
        value: &str,
    ) -> CustomResult<TagResourceOutput>;
}

#[async_trait]
impl SecretsExt for &str {
    /// Retrieves and de-serializes a secret from AWS Secrets Manager;
    /// taken from the [Get Secret Value] example.
    ///
    /// [Get Secret Value]: https://github.com/awslabs/aws-sdk-rust/blob/main/examples/secretsmanager/src/bin/get-secret-value.rs
    async fn get_secret<T: DeserializeOwned>(self, config: &SdkConfig) -> CustomResult<T> {
        let client = Client::new(config);

        let resp = client
            .get_secret_value()
            .secret_id(self)
            .send()
            .await
            .context(format_args!("[{}] couldn't read secret", self))?;

        let secret_str = resp.secret_string().unwrap();

        serde_json::from_str(secret_str).context("couldn't deserialize secret string")
    }

    /// Set or update the value of a **tag** on a secret that lives in
    /// AWS Secrets Manager.
    async fn set_tag(
        self,
        config: &SdkConfig,
        key: &str,
        value: &str,
    ) -> CustomResult<TagResourceOutput> {
        let client = Client::new(config);

        let tag = Tag::builder().key(key).value(value).build();

        client
            .tag_resource()
            .secret_id(self)
            .tags(tag)
            .send()
            .await
            .context("couldn't set tags")
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use super::*;

    #[test]
    fn test_deserialize_secret() {
        let secret_string = r#"
        {
          "UserName":"my-user",
          "AccessKeyId":"ABC321",
          "Status":"Active",
          "SecretAccessKey":"XYZ123",
          "CreateDate":"2022-07-11T09:00:12.000Z",
          "Password":"<PASSWORD>"
        }
        "#;

        trace!(
            "Result: {:#?}",
            from_str::<DummyCredentials>(secret_string).unwrap()
        );
    }
}
