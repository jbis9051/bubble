use crate::config::CONFIG;
use sendgrid::SGClient;
use sendgrid::SendgridError;
use sendgrid::{Destination, Mail};

#[derive(Default, Clone, Copy)]
pub struct SendGridEmailService {}
#[derive(Default, Clone, Copy)]
pub struct PrinterEmailService {}

pub trait EmailService {
    fn send(
        &self,
        email: &str,
        subject: &str,
        recipient: &[(String, String)],
        has_html: bool,
        _html_content: &str,
    ) -> Result<(), SendgridError>;
}

impl EmailService for SendGridEmailService {
    fn send(
        &self,
        email: &str,
        subject: &str,
        recipients: &[(String, String)],
        has_html: bool,
        html_content: &str,
    ) -> Result<(), SendgridError> {
        let api_key = &CONFIG.api_key_check; // pull api key

        // create mail object and add sender, recipient data
        let mut mail_info = Mail::new()
            .add_from(&CONFIG.sender_email)
            .add_from_name("Bubble")
            .add_subject(subject)
            .add_text(email);

        for tuple in recipients {
            mail_info = mail_info.add_to(Destination {
                address: &tuple.0,
                name: &tuple.1,
            })
        }
        if has_html {
            mail_info = mail_info.add_html(html_content);
        }
        // creates a client to send
        let client = SGClient::new(api_key);
        client.send(mail_info)?;

        Ok(())
    }
}

impl EmailService for PrinterEmailService {
    fn send(
        &self,
        email: &str,
        subjects: &str,
        recipients: &[(String, String)],
        _has_html: bool,
        html_content: &str,
    ) -> Result<(), SendgridError> {
        println!(
            "Mock Email sent to: {}, Subject: {}, Body: {}, HTML: {}",
            recipients[0].1, subjects, email, html_content
        );
        Ok(())
    }
}
