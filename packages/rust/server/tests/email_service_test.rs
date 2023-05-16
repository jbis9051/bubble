use bubble::services::email::EmailService;

#[test]
fn email_test() {
    let email_service = EmailService::default();

    let email = "Whats uppppp";
    let has_html = false;
    let subject = "TEST email";
    let recipient = ("branh@umich.edu", "Brandon");
    let html_content = "";

    let sent = email_service.send(email, subject, recipient, has_html, html_content);

    assert!(sent.is_ok());
}
