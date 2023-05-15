use bubble::services::email::EmailService;

#[test]
fn email_test() {
    let email_service = EmailService::default();

    let email = "Hi whats up";

    let sent = email_service.send(email);

    assert!(sent.is_ok());
}
