package account

import (
	"fmt"

	"github.com/macabu/cronus/internal/packet"
)

type PacketHandler struct{}

func (h *PacketHandler) Handle(packetReader packet.PacketReader) (packet.PacketWriter, error) {
	switch p := packetReader.(type) {
	case *packet.PACKET_CA_LOGIN:
		packetWriter, err := h.requestLogin(p)
		if err != nil {
			return nil, fmt.Errorf("request login handler: %w", err)
		}

		return packetWriter, nil
	}

	return nil, fmt.Errorf("unreachable? %T", packetReader)
}

func (h *PacketHandler) requestLogin(p *packet.PACKET_CA_LOGIN) (packet.PacketWriter, error) {
	_ = p

	// returning interface we can return any packet that is outgoing.
	return &packet.PACKET_AC_ACCEPT_LOGIN2{}, nil
}
