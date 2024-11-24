package packet

import (
	"io"

	"github.com/macabu/cronus/pkg/xbinary"
)

//	struct PACKET_AC_ACCEPT_LOGIN2 {
//	  /* this+0x0 */ short PacketType
//	  /* this+0x2 */ short PacketLength
//	  /* this+0x4 */ int AuthCode
//	  /* this+0x8 */ unsigned long AID
//	  /* this+0xc */ unsigned long userLevel
//	  /* this+0x10 */ unsigned long lastLoginIP
//	  /* this+0x14 */ char lastLoginTime[26]
//	  /* this+0x2e */ unsigned char Sex
//	  /* this+0x2f */ char iAccountSID[16]
//	  /* this+0x3f */ unsigned char twitterFlag
//	  /* this+0x2f */ struct SERVER_ADDR ServerList[...] { // Size 32
//	    /* this+0x0 */ unsigned long ip
//	    /* this+0x4 */ short port
//	    /* this+0x6 */ unsigned char name[20]
//	    /* this+0x1a */ unsigned short usercount
//	    /* this+0x1c */ unsigned short state
//	    /* this+0x1e */ unsigned short property
//	    /* this+0x20 */ unsigned char unknown[128]
//	  }
//	}
type PACKET_AC_ACCEPT_LOGIN2 struct {
	PacketType    int16
	PacketLength  int16
	AuthCode      int32
	AccountID     uint32
	UserLevel     uint32
	LastLoginIP   uint32   // use proper ip type?
	LastLoginTime [26]byte // modify these with proper types and then we have a func tha dictates how we marshal it to the wire
	Sex           byte
	AuthToken     [16]byte
	TwitterFlag   byte
	ServerList    []ServerList
}

type ServerList struct {
	IP        uint32
	Port      int16
	Name      string
	UserCount uint16
	State     uint16
	Property  uint16
	Unknown2  [128]byte
}

func (p *PACKET_AC_ACCEPT_LOGIN2) Size() int {
	return 43 + len(p.ServerList)*208 // TODO: not right size
}

func (p *PACKET_AC_ACCEPT_LOGIN2) WriteTo(w io.Writer) (int64, error) {
	xbinary.Write(w, int16(p.Size()))
	xbinary.Write(w, p.AuthCode)
	xbinary.Write(w, p.AccountID)
	xbinary.Write(w, p.UserLevel)
	xbinary.Write(w, p.LastLoginIP)
	xbinary.Write(w, p.LastLoginTime)
	xbinary.Write(w, p.Sex)

	for _, server := range p.ServerList {
		xbinary.Write(w, server.IP)
		xbinary.Write(w, server.Port)
		xbinary.WriteString(w, server.Name, 20)
		xbinary.Write(w, server.UserCount)
		xbinary.Write(w, server.State)
		xbinary.Write(w, server.Property)
		xbinary.Write(w, server.Unknown2)
	}

	return int64(p.Size()), nil
}
