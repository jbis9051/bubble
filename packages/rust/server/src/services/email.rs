use crate::config::CONFIG;
use sendgrid::SGClient;
use sendgrid::SendgridError;
use sendgrid::{Destination, Mail};


#[derive(Default, Clone, Copy)]
pub struct EmailService {}
pub struct MockEmailService {}

pub trait Email_ {
    fn send(&self,
            _email: &str,
            _subject: &str,
            _recipient: &Vec<(String, String)>,
            _hasHTML: bool,
            _html_content: &str) -> Result<(), SendgridError>;
}

impl Email_ for EmailService{
    fn send(
        &self,
        _email: &str,
        _subject: &str,
        _recipient: &Vec<(String, String)>,
        _hasHTML: bool,
        _html_content: &str,
    ) -> Result<(), SendgridError> {
        let api_key = &CONFIG.api_key_check; // pull api key

        // create mail object and add sender, recipient data
        let mut mail_info = Mail::new()
            .add_from(&CONFIG.sender_email)
            .add_from_name("Bubble")
            .add_subject(_subject)
            .add_text(_email);

        for tuple in _recipient {
            mail_info = mail_info.add_to(Destination {
                address: &tuple.0,
                name: &tuple.1,
            })
        }
        if _hasHTML {
            mail_info = mail_info.add_html(_html_content);
        }
        // do stuff to send
        let client = SGClient::new(api_key);
        client.send(mail_info)?;

        Ok(())
    }
}

impl Email_ for MockEmailService {
    fn send(&self,
            _email: &str,
            _subject: &str,
            _recipients: &Vec<(String, String)>,
            _hasHTML: bool,
            _html_content: &str) -> Result<(), SendgridError> {

        let api_key = &CONFIG.api_key_check;
        let mut mail_info = Mail::new()
            .add_from(&CONFIG.sender_email)
            .add_from_name("Bubble")
            .add_subject(_subject)
            .add_text(_email);

        for tuple in _recipients {
            mail_info = mail_info.add_to(Destination {
                address: &tuple.0,
                name: &tuple.1,
            })
        }

        if _hasHTML {
            mail_info = mail_info.add_html(_html_content);
        }


        let actual = String::from(format!("Mock Email sent to: {}, Subject: {}, Body: {}, HTML: {}", mail_info.to[0].name, mail_info.subject,
            mail_info.text, mail_info.html));
        let expected =  String::from(format!("Mock Email sent to: {}, Subject: {}, Body: {}, HTML: {}", _recipients[0].1, _subject,
                                             _email, _html_content));
        assert_eq!(actual, expected);

        Ok(())
    }
}








