package packet

import (
	"fmt"
	"io"

	"github.com/macabu/cronus/pkg/xbinary"
)

type PACKET_CA_LOGIN struct {
	// /* this+0x0 */ short PacketType
	version    uint32 // /* this+0x2 */ unsigned long Version
	id         string // /* this+0x6 */ unsigned char ID[24]
	password   string // /* this+0x1e */ unsigned char Passwd[24]
	clienttype uint8  // /* this+0x36 */ unsigned char clienttype
}

func (p *PACKET_CA_LOGIN) Size() int {
	return 55
}

func (p *PACKET_CA_LOGIN) ReadFrom(r io.Reader) (int64, error) {
	if err := xbinary.Read(r, &p.version); err != nil {
		return 0, fmt.Errorf("reading version: %w", err)
	}

	if err := xbinary.ReadString(r, &p.id, 24); err != nil {
		return 0, fmt.Errorf("reading id: %w", err)
	}

	if err := xbinary.ReadString(r, &p.password, 24); err != nil {
		return 0, fmt.Errorf("reading password: %w", err)
	}

	if err := xbinary.Read(r, &p.clienttype); err != nil {
		return 0, fmt.Errorf("reading clienttype: %w", err)
	}

	return int64(p.Size()), nil
}
