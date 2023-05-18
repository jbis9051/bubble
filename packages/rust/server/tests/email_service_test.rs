#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use bubble::services::email::EmailService;
    use bubble::services::email::Email_;
    use mockall::{automock, predicate::*};

    #[test]
    fn real_email_test() {
        let email_service = EmailService::default();

        let email = "Whats uppppp";
        let has_html = false;
        let subject = "TEST email";
        let recipient = vec!((String::from("branh@umich.edu"), String::from("Brandon")));
        let html_content = "";

        let sent = email_service.send(email, subject, &recipient, has_html, html_content);

        assert!(sent.is_ok());
    }
    #[test]
    fn mock_email_test() {

        let mut mock_email = EmailService::default();

        let email = "Whats uppppp";
        let has_html = false;
        let subject = "TEST email";
        let recipient = vec!((String::from("branh@umich.edu"), String::from("Brandon")));
        let html_content = "";




        let result = mock_email.send("test@bitches.com", "Testing bitch",
                                        &recipient, false, "");

        assert!(result.is_ok());

    }
}



