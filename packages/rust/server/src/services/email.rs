use sendgrid::SGClient;
use sendgrid::SendgridError;
use sendgrid::{Destination, Mail};

#[derive(Default, Clone, Copy)]
pub struct EmailService {}

impl EmailService {
    pub fn send(&self, _email: &str) -> Result<(), SendgridError> {
        let mut env_vars = std::env::vars();
        let api_key_check = env_vars.find(|var| var.0 == "SENDGRID_API_KEY"); // pull api key from env. variables
        let sender_email = match std::env::var("sender_email") {
            Ok(value) => value,
            Err(_) => return Err(SendgridError::InvalidFilename),
        };
        let api_key: String = match api_key_check {
            // make sure pulled value is valid
            Some(key) => key.1,
            None => return Err(SendgridError::InvalidFilename),
        };

        // create mail object and add sender, recipient data
        let mail_info = Mail::new()
            .add_to(Destination {
                address: "branh@umich.edu",
                name: "Brandon",
            })
            .add_from(&sender_email)
            .add_from_name("Bubble")
            .add_subject("Test Email")
            .add_text(_email);

        // do stuff to send
        let client = SGClient::new(api_key);
        client.send(mail_info)?;

        Ok(())
    }
}
