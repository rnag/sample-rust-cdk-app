use base64::write::EncoderWriter as Base64Encoder;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, ClientBuilder};
use std::fmt;
use std::io::Write;

use crate::{Admin, AwsErrorContext, Result, CONTENT_TYPE_JSON};

/// Returns the value to set in the `AUTHORIZATION` header for a request.
pub fn auth_token(token: &str) -> String {
    // "Bearer ".len() == 7
    let mut bearer_token = String::with_capacity(7 + token.len());
    bearer_token.push_str("Bearer ");
    bearer_token.push_str(token);

    bearer_token
}

// this code is mostly taken from the `reqwest::basic_auth` implementation
pub fn auth_header_value<S>(user: S, password: S) -> HeaderValue
where
    S: fmt::Display,
{
    let mut auth_value = b"Basic ".to_vec();
    {
        let mut encoder = Base64Encoder::new(&mut auth_value, base64::STANDARD);
        // The unwrap here is fine because Vec::write* is infallible.
        write!(encoder, "{}:{}", user, password).unwrap();
    }
    let mut header_value = HeaderValue::from_bytes(&auth_value).unwrap();
    header_value.set_sensitive(true);

    header_value
}

pub fn reqwest_client(admin: Admin) -> Result<Client> {
    let mut headers = HeaderMap::with_capacity(2);
    headers.insert(
        AUTHORIZATION,
        auth_header_value(admin.username, admin.password),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static(CONTENT_TYPE_JSON));

    let builder = ClientBuilder::new().default_headers(headers);
    let client = builder.build().context("couldn't build client")?;

    Ok(client)
}
