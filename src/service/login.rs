use std::error::Error;

use either::Either;

use crate::packet;

#[derive(Debug, Clone)]
pub struct LoginService {}

impl LoginService {
    pub fn login(
        &self,
        input: &packet::PACKET_CA_LOGIN,
    ) -> Result<
        Either<packet::PACKET_AC_ACCEPT_LOGIN, packet::PACKET_AC_REFUSE_LOGIN>,
        Box<dyn Error>,
    > {
        // Call registration handler.
        if input.id.ends_with("_M") || input.id.ends_with("_F") {
            let _ = self.register(input)?;
        }

        // Check if user exists in DB, fetch it.
        if false {
            return Ok(either::Right(packet::PACKET_AC_REFUSE_LOGIN::new(
                packet::RefuseLoginErrorCode::BlockedByGM,
                "xxx",
            )));
        }

        // Check if password matches.
        // Append to login_activity table.
        // Respond to client.

        Ok(either::Left(packet::PACKET_AC_ACCEPT_LOGIN {
            packet_type: todo!(),
            packet_length: todo!(),
            auth_code: todo!(),
            aid: todo!(),
            user_level: todo!(),
            last_login_ip: todo!(),
            last_login_time: todo!(),
            sex: todo!(),
            server_list: todo!(),
        }))
    }

    pub fn register(&self, input: &packet::PACKET_CA_LOGIN) -> Result<(), Box<dyn Error>> {
        // Check if the username already exists.
        // Populate information (id, email, use argon2 for password) and store in DB.
        // What to return? Maybe nothing.
        Ok(())
    }
}
