#[derive(Default, Clone, Copy)]
pub struct EmailService {}

impl EmailService {
    pub fn send(&self, email: &str) -> Result<(), ()> {
        // TODO: implement
        println!("Sending email: {}", email);
        Ok(())
    }
}
