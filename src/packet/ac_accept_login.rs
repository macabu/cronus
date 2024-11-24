#![allow(non_camel_case_types)]
use super::PacketHeader;

/*
// packet 0x69
struct PACKET_AC_ACCEPT_LOGIN {
  /* this+0x0 */ short PacketType
  /* this+0x2 */ short PacketLength
  /* this+0x4 */ int AuthCode
  /* this+0x8 */ unsigned long AID
  /* this+0xc */ unsigned long userLevel
  /* this+0x10 */ unsigned long lastLoginIP
  /* this+0x14 */ char lastLoginTime[26]
  /* this+0x2e */ unsigned char Sex
  /* this+0x2f */ struct SERVER_ADDR ServerList[...] { // Size 32
    /* this+0x0 */ unsigned long ip
    /* this+0x4 */ short port
    /* this+0x6 */ unsigned char name[20]
    /* this+0x1a */ unsigned short usercount
    /* this+0x1c */ unsigned short state
    /* this+0x1e */ unsigned short property
  }
}
*/

#[derive(Debug, Clone)]
pub struct PACKET_AC_ACCEPT_LOGIN {
    pub packet_type: i16,
    pub packet_length: i16,
    pub auth_code: i32,
    pub aid: u32,
    pub user_level: u32,
    pub last_login_ip: u32,
    pub last_login_time: [u8; 26],
    pub sex: u8, // char
    pub server_list: Vec<SERVER_ADDR>,
}

#[derive(Debug, Clone)]
pub struct SERVER_ADDR {
    pub ip: u32,
    pub port: i16,
    pub name: String,
    pub user_count: u16,
    pub state: u16,
    pub property: u16,
}

impl PacketHeader for PACKET_AC_ACCEPT_LOGIN {
    const HEADER: u16 = 0x69;
}

impl From<PACKET_AC_ACCEPT_LOGIN> for Vec<u8> {
    // TODO: use different type, not Vec<u8>
    fn from(value: PACKET_AC_ACCEPT_LOGIN) -> Self {
        todo!()
    }
}
