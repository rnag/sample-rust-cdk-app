use std::time::Instant;

use aws_sdk_sesv2::model::EmailTemplateContent;
use aws_sdk_sesv2::types::SdkError;
use aws_sdk_sesv2::Client;
use aws_types::SdkConfig;

use sample_rust_cdk_app::*;

pub struct SESHelper {
    client: Client,
}

impl From<&SdkConfig> for SESHelper {
    fn from(config: &SdkConfig) -> Self {
        let client = Client::new(config);
        Self { client }
    }
}

impl SESHelper {
    /// Create a new `SESHelper`, with an SES Client constructed with AWS
    /// credentials retrieved from the environment.
    pub async fn new() -> SESHelper {
        Self::from(&aws_config::load_from_env().await)
    }

    /// Send a [SES API v2 email] using the AWS SDK
    ///
    /// [SES API v2 email]: https://docs.aws.amazon.com/ses/latest/dg/example_sesv2_SendEmail_section.html
    pub async fn update_template(
        &self,
        name: &str,
        subject: impl Into<Option<&str>>,
        html_content: &str,
    ) -> CustomResult<()> {
        debug!("Saving SES template: [[ {} ]]", name);

        let start = Instant::now();

        let subject = subject.into().unwrap_or("{{subject}}");

        let content = EmailTemplateContent::builder()
            .subject(subject)
            .html(html_content)
            .build();

        let resp = match self
            .client
            .update_email_template()
            .template_name(name)
            .template_content(content.clone())
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match e {
                SdkError::ServiceError { err, .. } => {
                    if err.is_not_found_exception() {
                        match self
                            .client
                            .create_email_template()
                            .template_name(name)
                            .template_content(content)
                            .send()
                            .await
                            .map_context(|err| {
                                error!("Failed to create SES template: {:?}", err);
                            }) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                error!("error: {e:?}");
                                Err(CustomError::new_response())
                            }
                        }
                    } else {
                        error!("error: {err:?}");
                        Err(CustomError::new_response())
                    }
                }
                _ => {
                    error!("error: {e:?}");
                    Err(CustomError::new_response())
                }
            },
        };

        trace!("Saved SES template in {:.2?}", start.elapsed());

        resp
    }
}
