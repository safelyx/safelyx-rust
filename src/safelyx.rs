use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainAnalysis {
    pub domain_reputation: String,
    pub source_code: String,
    pub anti_virus: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafeLinkResponse {
    pub url: String,
    pub result: i32,
    pub result_text: String,
    pub date: String,
    pub analysis: DomainAnalysis,
    pub checks_remaining: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAnalysis {
    pub address: String,
    pub domain_reputation: String,
    pub mx_records: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafeEmailResponse {
    pub email: String,
    pub result: i32,
    pub result_text: String,
    pub date: String,
    pub analysis: EmailAnalysis,
    pub checks_remaining: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageLinkAnalysis {
    pub url: String,
    pub result: i32,
    pub date: String,
    pub analysis: DomainAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEmailAnalysis {
    pub email: String,
    pub result: i32,
    pub date: String,
    pub analysis: EmailAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageAnalysis {
    pub content: String,
    pub sentiment: String,
    pub links: Vec<MessageLinkAnalysis>,
    pub emails: Vec<MessageEmailAnalysis>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafeMessageResponse {
    pub message: String,
    pub result: i32,
    pub result_text: String,
    pub date: String,
    pub analysis: MessageAnalysis,
    pub checks_remaining: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageAnalysis {
    pub description: String,
    pub link: MessageLinkAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafeImageResponse {
    pub image_url: String,
    pub result: i32,
    pub result_text: String,
    pub date: String,
    pub analysis: ImageAnalysis,
    pub checks_remaining: i32,
}

fn create_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    headers
}

/// Securely checks if a link is safe to click or visit.
pub async fn check_link(
    link: &str,
    key_code: Option<&str>,
) -> Result<SafeLinkResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut payload = HashMap::new();
    payload.insert("link", link);
    if let Some(code) = key_code {
        payload.insert("key_code", code);
    }

    let response = client
        .post("https://safelyx.com/safe-link-checker")
        .headers(create_headers())
        .json(&payload)
        .send()
        .await?;

    response.json::<SafeLinkResponse>().await
}

/// Securely checks if an email address is legitimate.
pub async fn check_email(
    email: &str,
    key_code: Option<&str>,
) -> Result<SafeEmailResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut payload = HashMap::new();
    payload.insert("email", email);
    if let Some(code) = key_code {
        payload.insert("key_code", code);
    }

    let response = client
        .post("https://safelyx.com/safe-email-checker")
        .headers(create_headers())
        .json(&payload)
        .send()
        .await?;

    response.json::<SafeEmailResponse>().await
}

/// Securely checks if a message's content is safe.
pub async fn check_message(
    message: &str,
    skip_link_and_email_checks: bool,
    key_code: Option<&str>,
) -> Result<SafeMessageResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut payload = HashMap::new();
    let skip_link_and_email_checks_string = &skip_link_and_email_checks.to_string();
    payload.insert("message", message);
    payload.insert(
        "skip_link_and_email_checks",
        skip_link_and_email_checks_string,
    );
    if let Some(code) = key_code {
        payload.insert("key_code", code);
    }

    let response = client
        .post("https://safelyx.com/safe-message-checker")
        .headers(create_headers())
        .json(&payload)
        .send()
        .await?;

    response.json::<SafeMessageResponse>().await
}

/// Securely checks if an image is safe.
pub async fn check_image(
    image_url: &str,
    key_code: Option<&str>,
) -> Result<SafeImageResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut payload = HashMap::new();
    payload.insert("image_url", image_url);
    if let Some(code) = key_code {
        payload.insert("key_code", code);
    }

    let response = client
        .post("https://safelyx.com/safe-image-checker")
        .headers(create_headers())
        .json(&payload)
        .send()
        .await?;

    response.json::<SafeImageResponse>().await
}
