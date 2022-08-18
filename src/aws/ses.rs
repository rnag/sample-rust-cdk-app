use crate::*;

use aws_sdk_sesv2::model::{Destination, EmailContent, Template};
use aws_sdk_sesv2::Client;
use aws_types::SdkConfig;
use lambda_runtime::Context;
use serde::Serialize;
use serde_json::to_string;

pub const SES_TEMPLATE_NAME: &str = "test-failure"; // TODO
pub(crate) const AWS_ROOT: &str = "https://console.aws.amazon.com";

/// Get the AWS Account Id from a Lambda Function ARN
pub fn get_account_id<'a>(function_arn: impl Into<String>) -> String {
    function_arn.into().split(':').nth(4).unwrap().to_owned()
}

pub async fn send_failure_email_to_admins(
    config: &SdkConfig,
    ctx: Context,
    secret_name: &str,
    errors: Vec<RequestError>,
) -> Result<()> {
    let from = SES_SENDER.unwrap_or("my-default-sender@email.com");
    let to = ADMIN_EMAILS.map(|s| s.to_owned()).to_vec();

    let account_name = ACCOUNT_NAME.unwrap_or("aws-test-account");
    let account_id = get_account_id(ctx.invoked_function_arn);

    let fn_name = ctx.env_config.function_name;
    let log_group_name = ctx.env_config.log_group;
    let log_stream_name = ctx.env_config.log_stream;
    let subject = "[ACTION NEEDED] My Test";

    // AWS Region, should be automatically set for AWS Lambda functions
    #[allow(unused, non_snake_case)]
    let AWS_REGION = env_or_default("AWS_REGION", "us-east-1");

    trace!("sending email to: {:?}", to);

    // Build account info
    let account = AccountInfo {
        name: account_name,
        id: account_id.as_str(),
    };

    // Build links
    let mut links = Vec::with_capacity(3);
    if let Some(source_code) = SOURCE_CODE {
        links.push(Link {
            text: "Link to Source",
            location: source_code.to_owned(),
        });
    }
    links.push(Link {
        text: "Link to Lambda",
        location: format!("{AWS_ROOT}/lambda/home?region={AWS_REGION}#/functions/{fn_name}"),
    });
    links.push(Link{
        text: "Link to Logs",
        location: format!("{AWS_ROOT}/cloudwatch/home?region={AWS_REGION}#logEventViewer:group={log_group_name};stream={log_stream_name}"),
    });

    // Construct SES template data
    let data = TemplateData {
        subject: &subject,
        function_name: fn_name.as_str(),
        secret_name,
        errors,
        account,
        links,
    };

    let ses = SESHelper::from(config);
    ses.send_outlook_email(to, from, data).await
}

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
    pub async fn send_templated_email(
        &self,
        to: Vec<String>,
        from: &str,
        name: impl Into<String>,
        data: impl Into<String>,
    ) -> Result<()> {
        debug!("Sending templated email to: {:?}", to);

        let dest = Destination::builder().set_to_addresses(Some(to)).build();

        let template = Template::builder()
            .template_name(name)
            .template_data(data)
            .build();

        let email_content = EmailContent::builder().template(template).build();

        self.client
            .send_email()
            .from_email_address(from)
            .destination(dest)
            .content(email_content)
            .send()
            .await
            .map_context(|err| {
                error!("Unable to send templated email: {:?}", err);
            })?;

        Ok(())
    }

    // Test rendering an SES template.
    pub async fn test_render_template(
        &self,
        name: impl Into<String>,
        data: impl Into<String>,
    ) -> CustomResult<String> {
        let template = self
            .client
            .test_render_email_template()
            .template_name(name)
            .template_data(data)
            .send()
            .await
            .context("failed to render template")?;

        match template.rendered_template {
            Some(result) => Ok(result),
            None => {
                error!("received an empty template");
                Err(CustomError::new_response())
            }
        }
    }

    /// Sends a Lambda Failure email formatted for MS Outlook
    #[allow(unused)]
    async fn send_outlook_email<D: Serialize>(
        &self,
        to: Vec<String>,
        from: &str,
        data: D,
    ) -> Result<()> {
        let data = to_string(&data).context("couldn't deserialize data")?;
        self.send_templated_email(to, from, SES_TEMPLATE_NAME, data)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::get_account_id;

    #[test]
    fn test_get_account_id() {
        let function_arn = "arn:aws:lambda:us-east-1:1234567890:function:my-lambda-function-123";
        assert_eq!("1234567890", get_account_id(function_arn.to_owned()));
    }
}
