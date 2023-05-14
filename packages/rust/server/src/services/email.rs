use sendgrid::SGClient;
use sendgrid::SendgridError;
use sendgrid::{Destination, Mail};

#[derive(Default, Clone, Copy)]
pub struct EmailService {}

impl EmailService {
    pub fn send(&self, _email: &str) -> Result<(), SendgridError> {
        let mut env_vars = std::env::vars();
        let api_key_check = env_vars.find(|var| var.0 == "SENDGRID_API_KEY"); // pull api key from env. variables
        let api_key: String = match api_key_check {
            // make sure pulled value is valid
            Some(key) => key.1,
            None => return Err(SendgridError::InvalidFilename),
        };

        // create mail object and add sender, recipient data
        let mail_info = Mail::new()
            .add_to(Destination {
                address: "jimlin@umich.edu",
                name: "Jimmy",
            })
            .add_from("branh@umich.edu")
            .add_from_name("Brandon")
            .add_subject("Test Email")
            .add_text("I wanna sleep")
            .add_html("<h1>Testing the code</h1>");

        // do stuff to send
        let client = SGClient::new(api_key);
        client.send(mail_info)?;

        Ok(())
    }
}
