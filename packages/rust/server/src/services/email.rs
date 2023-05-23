use crate::config::CONFIG;
use sendgrid::SGClient;
use sendgrid::SendgridError;
use sendgrid::{Destination, Mail};

#[derive(Default, Clone, Copy)]
pub struct SendGridEmailService {}
#[derive(Default, Clone, Copy)]
pub struct PrinterEmailService {}

#[derive(Debug)]
pub struct Recipient {
    pub address: String,
    pub name: String,
}

pub trait EmailService {
    fn send(
        &self,
        subject: &str,
        recipients: &[Recipient],
        text_content: Option<&str>,
        html_content: Option<&str>,
    ) -> Result<(), SendgridError>;
}

impl EmailService for SendGridEmailService {
    fn send(
        &self,
        subject: &str,
        recipients: &[Recipient],
        text_content: Option<&str>,
        html_content: Option<&str>,
    ) -> Result<(), SendgridError> {
        let api_key = &CONFIG.api_key_check;

        // create mail object and add sender, recipient data
        let mut mail_info = Mail::new()
            .add_from(&CONFIG.sender_email)
            .add_from_name("Bubble")
            .add_subject(subject);

        if let Some(text_content) = text_content {
            mail_info = mail_info.add_text(text_content);
        }

        if let Some(html_content) = html_content {
            mail_info = mail_info.add_html(html_content);
        }

        for recipient in recipients {
            mail_info = mail_info.add_to(Destination {
                address: &recipient.address,
                name: &recipient.name,
            })
        }

        let client = SGClient::new(api_key);

        // TODO async
        client.send(mail_info)?;

        Ok(())
    }
}

impl EmailService for PrinterEmailService {
    fn send(
        &self,
        subject: &str,
        recipients: &[Recipient],
        text_content: Option<&str>,
        html_content: Option<&str>,
    ) -> Result<(), SendgridError> {
        println!(
            "Mock Email sent to: {:?}, Subject: {}, Body: {:?}, HTML: {:?}",
            recipients, subject, text_content, html_content
        );
        Ok(())
    }
}
