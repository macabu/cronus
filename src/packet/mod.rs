mod ca_login;
pub use ca_login::*;

mod ac_accept_login;
pub use ac_accept_login::*;

mod ac_refuse_login;
pub use ac_refuse_login::*;

pub trait PacketHeader {
    const HEADER: i16;
}

pub trait Packet: PacketHeader {}

// PacketReader should implement a method to convert from &[u8] into a struct.
pub trait PacketReader: Packet + for<'a> TryFrom<&'a [u8]> {}

// PacketWriter should implement a method to convert to a Vec<u8> for writing it back to the buffer.
pub trait PacketWriter: Packet + Into<Vec<u8>> {}
