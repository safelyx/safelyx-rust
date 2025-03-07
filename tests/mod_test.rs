use dotenv::dotenv;
use std::env;

// Since this is an integration test, we import from the crate root
use safelyx::{check_email, check_image, check_link, check_message};

fn setup() -> String {
    dotenv().ok();
    env::var("TEST_KEY_CODE").expect("TEST_KEY_CODE must be set")
}

#[tokio::test]
async fn test_check_link() {
    let test_key_code = setup();

    let test_url = "example.com";
    let parsed_url = check_link(test_url, Some(&test_key_code))
        .await
        .expect("Failed to check link");

    assert_eq!(parsed_url.url, "https://example.com");
    assert!(parsed_url.result == -2 || (0..=10).contains(&parsed_url.result));
    assert!(!parsed_url.result_text.is_empty());
    assert!(!parsed_url.date.is_empty());
    assert!(!parsed_url.analysis.domain_reputation.is_empty());
    assert!(!parsed_url.analysis.source_code.is_empty());
    assert!(!parsed_url.analysis.anti_virus.is_empty());
    assert!(parsed_url.checks_remaining >= 0);
}

#[tokio::test]
async fn test_check_email() {
    let test_key_code = setup();

    let test_email = "help@safelyx.com";
    let parsed_email = check_email(test_email, Some(&test_key_code))
        .await
        .expect("Failed to check email");

    assert_eq!(parsed_email.email, "help@safelyx.com");
    assert!(parsed_email.result == -2 || (8..=10).contains(&parsed_email.result));
    assert!(!parsed_email.result_text.is_empty());
    assert!(!parsed_email.date.is_empty());
    assert!(!parsed_email.analysis.address.is_empty());
    assert!(!parsed_email.analysis.domain_reputation.is_empty());
    assert!(!parsed_email.analysis.mx_records.is_empty());
    assert!(parsed_email.checks_remaining >= 0);
}

#[tokio::test]
async fn test_check_message() {
    let test_key_code = setup();

    let test_message = "Hello, world!";
    let parsed_message = check_message(test_message, false, Some(&test_key_code))
        .await
        .expect("Failed to check message");

    assert_eq!(parsed_message.message, "Hello, world!");
    assert!(parsed_message.result == -2 || (8..=10).contains(&parsed_message.result));
    assert!(!parsed_message.result_text.is_empty());
    assert!(!parsed_message.date.is_empty());
    assert!(!parsed_message.analysis.content.is_empty());
    assert!(!parsed_message.analysis.sentiment.is_empty());
    assert!(parsed_message.checks_remaining >= 0);
}

#[tokio::test]
async fn test_check_image() {
    let test_key_code = setup();

    let test_image_url =
        "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png";
    let parsed_image = check_image(test_image_url, Some(&test_key_code))
        .await
        .expect("Failed to check image");

    assert_eq!(parsed_image.image_url, test_image_url);
    assert!(parsed_image.result == -2 || (8..=10).contains(&parsed_image.result));
    assert!(!parsed_image.result_text.is_empty());
    assert!(!parsed_image.date.is_empty());
    assert!(!parsed_image.analysis.description.is_empty());
    assert!(!parsed_image.analysis.link.url.is_empty());
    assert!(
        parsed_image.analysis.link.result == -2
            || parsed_image.analysis.link.result == -1
            || (8..=10).contains(&parsed_image.analysis.link.result)
    );
    assert!(!parsed_image.analysis.link.date.is_empty());
    assert!(
        !parsed_image
            .analysis
            .link
            .analysis
            .domain_reputation
            .is_empty()
    );
    assert!(!parsed_image.analysis.link.analysis.source_code.is_empty());
    assert!(!parsed_image.analysis.link.analysis.anti_virus.is_empty());
    assert!(parsed_image.checks_remaining >= 0);
}
