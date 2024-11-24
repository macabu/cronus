mod ca_login;
pub use ca_login::*;

mod ac_accept_login;
pub use ac_accept_login::*;

pub trait PacketHeader {
    const HEADER: u16;
}

pub trait Packet: PacketHeader {}
