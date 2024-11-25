#![allow(non_camel_case_types)]

use super::PacketHeader;

/*
// packet 0x6a
struct PACKET_AC_REFUSE_LOGIN {
  /* this+0x0 */ short PacketType
  /* this+0x2 */ unsigned char ErrorCode
  /* this+0x3 */ char blockDate[20]
}
*/

// From https://github.com/eathena/eathena/blob/master/src/login/login.c#L918
#[derive(Debug)]
pub enum RefuseLoginErrorCode {
    UnregisteredID = 0,
    IncorrectPassword = 1,
    AccountExpired = 2,
    RejectedFromServer = 3,
    BlockedByGM = 4,
    NotLatestGameExe = 5,
    Banned = 6,
    ServerOverPopulation = 7,
    AccountLimitFromCompany = 8,
    BanByDBA = 9,
    EmailNotConfirmed = 10,
    BanByGM = 11,
    WorkingInDB = 12,
    SelfLock = 13,
    NotPermittedGroup = 14,
    NotPermittedGroup2 = 15,
    AccountGone = 99,
    LoginInfoRemains = 100,
    HackingInvestigation = 101,
    BugInvestigation = 102,
    DeletingChar = 103,
    DeletingSpouseChar = 104,
    UnknownError = 255,
}

impl From<RefuseLoginErrorCode> for u8 {
    fn from(value: RefuseLoginErrorCode) -> Self {
        value as u8
    }
}

#[derive(Debug, Clone)]
pub struct PACKET_AC_REFUSE_LOGIN {
    pub packet_type: i16,
    pub error_code: u8,
    pub block_date: [u8; 20],
}

impl PACKET_AC_REFUSE_LOGIN {
    pub fn new(error_code: RefuseLoginErrorCode, block_date: &str) -> Self {
        PACKET_AC_REFUSE_LOGIN {
            packet_type: PACKET_AC_REFUSE_LOGIN::HEADER,
            error_code: u8::from(error_code),
            block_date: todo!(),
        }
    }
}

impl PacketHeader for PACKET_AC_REFUSE_LOGIN {
    const HEADER: i16 = 0x6a;
}

impl From<PACKET_AC_REFUSE_LOGIN> for Vec<u8> {
    fn from(packet: PACKET_AC_REFUSE_LOGIN) -> Self {
        let mut buf = Vec::with_capacity(std::mem::size_of_val(&packet));

        buf.extend(&packet.packet_type.to_le_bytes());
        buf.push(packet.error_code);
        buf.extend(&packet.block_date);

        buf
    }
}
