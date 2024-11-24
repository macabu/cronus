mod login;
pub use login::*;

mod register;
pub use register::*;

pub trait CommandHandler {
    type Input;
    type Output;
    type Error;

    fn handle(&self, input: &Self::Input) -> Result<Self::Output, Self::Error>;
}
