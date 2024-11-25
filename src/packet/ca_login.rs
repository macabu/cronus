#![allow(non_camel_case_types)]
use super::PacketHeader;

/*
// packet 0x64
struct PACKET_CA_LOGIN {
  /* this+0x0 */ short PacketType
  /* this+0x2 */ unsigned long Version
  /* this+0x6 */ unsigned char ID[24]
  /* this+0x1e */ unsigned char Passwd[24]
  /* this+0x36 */ unsigned char clienttype
}
*/
#[derive(Debug, Clone)]
pub struct PACKET_CA_LOGIN {
    pub version: u32,
    pub id: String,
    pub password: String,
    pub clienttype: u8,
}

impl PacketHeader for PACKET_CA_LOGIN {
    const HEADER: i16 = 0x64;
}

impl TryFrom<&[u8]> for PACKET_CA_LOGIN {
    type Error = Box<dyn std::error::Error>;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let mut offset = 2;

        let version = u32::from_le_bytes(buf[offset..4 + offset].try_into()?);
        offset += 4;

        let id = String::from_utf8_lossy(&buf[offset..24 + offset]).into_owned();
        offset += 24;

        let password = String::from_utf8_lossy(&buf[offset..24 + offset]).into_owned();
        offset += 24;

        let clienttype = buf[offset..1 + offset][0];

        Ok(PACKET_CA_LOGIN {
            version,
            id,
            password,
            clienttype,
        })
    }
}
