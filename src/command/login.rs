use either::Either;

use crate::packet::{self, RefuseLoginErrorCode, PACKET_AC_ACCEPT_LOGIN, PACKET_AC_REFUSE_LOGIN};

use super::{CommandHandler, RegistrationHandler};

#[derive(Debug, Clone)]
pub struct LoginHandler<'a> {
    pub registration_handler: &'a RegistrationHandler,
}

impl CommandHandler for LoginHandler<'_> {
    type Input = packet::PACKET_CA_LOGIN;
    type Output = Either<PACKET_AC_ACCEPT_LOGIN, PACKET_AC_REFUSE_LOGIN>;
    type Error = Box<dyn std::error::Error>;

    fn handle(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        // Call registration handler.
        if input.id.ends_with("_M") || input.id.ends_with("_F") {
            self.registration_handler.handle(input)?;
        }

        // Check if user exists in DB, fetch it.
        if false {
            return Ok(either::Right(PACKET_AC_REFUSE_LOGIN::new(
                RefuseLoginErrorCode::BlockedByGM,
                "xxx",
            )));
        }

        // Check if password matches.
        // Append to login_activity table.
        // Respond to client.

        Ok(either::Left(PACKET_AC_ACCEPT_LOGIN {
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
}
