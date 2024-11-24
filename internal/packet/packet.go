package packet

import (
	"io"
	"reflect"
)

type PacketReader interface {
	io.ReaderFrom
	PacketSize
}

type PacketWriter interface {
	io.WriterTo
	PacketSize
}

type PacketSize interface {
	Size() int
}

type PacketHandler interface {
	Handle(packet PacketReader) (PacketWriter, error)
}

// TODO: Have a factory for each server?
var PacketFactory = struct {
	Reader map[uint16]PacketReader
	Writer map[reflect.Type]uint16
}{
	Reader: map[uint16]PacketReader{
		0x0064: &PACKET_CA_LOGIN{},
	},
	Writer: map[reflect.Type]uint16{
		reflect.TypeOf(&PACKET_AC_ACCEPT_LOGIN2{}): 0x0AC4,
	},
}
