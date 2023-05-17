use crate::config::CONFIG;
use sendgrid::SGClient;
use sendgrid::SendgridError;
use sendgrid::{Destination, Mail};

#[derive(Default, Clone, Copy)]
pub struct EmailService {}

impl EmailService {
    pub fn send(
        &self,
        _email: &str,
        _subject: &str,
        _recipient: (&str, &str),
        _hasHTML: bool,
        _html_content: &str,
    ) -> Result<(), SendgridError> {
        let api_key = &CONFIG.api_key_check; // pull api key

        // create mail object and add sender, recipient data
        let mut mail_info = Mail::new()
            .add_to(Destination {
                address: _recipient.0,
                name: _recipient.1,
            })
            .add_from(&CONFIG.sender_email)
            .add_from_name("Bubble")
            .add_subject(_subject)
            .add_text(_email);
        if _hasHTML {
            mail_info = mail_info.add_html(_html_content);
        }
        // do stuff to send
        let client = SGClient::new(api_key);
        client.send(mail_info)?;

        Ok(())
    }
}
