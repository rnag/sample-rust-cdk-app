// SES - Sender (from) email, must be verified in the SES console
pub const SES_SENDER: Option<&str> = option_env!("SES_SENDER");

// Repository URL to the project in version control (defined in `package.json`)
pub const SOURCE_CODE: Option<&str> = option_env!("SOURCE_CODE");

// AWS Account Name
pub const ACCOUNT_NAME: Option<&str> = option_env!("ACCOUNT_NAME");

// A list of admin emails to notify if any errors occur
pub const ADMIN_EMAILS: [&str; 1] = ["some-dev@org.com"];
