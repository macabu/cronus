use crate::packet;

use super::CommandHandler;

#[derive(Debug, Clone)]
pub struct RegistrationHandler {}

impl CommandHandler for RegistrationHandler {
    type Input = packet::PACKET_CA_LOGIN;
    type Output = (); // TODO
    type Error = Box<dyn std::error::Error>;

    fn handle(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        // Check if the username already exists.
        // Populate information (id, email, use argon2 for password) and store in DB.
        // What to return? Maybe nothing.
        Ok(())
    }
}
