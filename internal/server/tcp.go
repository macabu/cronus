package server

import (
	"bufio"
	"context"
	"encoding/binary"
	"fmt"
	"log/slog"
	"net"
	"reflect"

	"github.com/macabu/cronus/internal/packet"
)

type Server struct {
	packetHandler packet.PacketHandler
	address       string
}

func NewServer(address string, packetHandler packet.PacketHandler) *Server {
	return &Server{
		address:       address,
		packetHandler: packetHandler,
	}
}

func (s *Server) ListenAndServe(ctx context.Context) error {
	listener, err := net.Listen("tcp", s.address)
	if err != nil {
		return fmt.Errorf("tcp listen: %w", err)
	}

	defer listener.Close()

	go func() {
		<-ctx.Done()
		listener.Close()
	}()

	slog.InfoContext(ctx, "Listening for connections", slog.String("address", listener.Addr().String()))

	for {
		conn, err := listener.Accept()
		if err != nil {
			return fmt.Errorf("tcp accept connection: %w", err)
		}

		go s.handleConnection(ctx, conn)
	}
}

// TODO: make this return err so we dont log so much
func (s *Server) handleConnection(ctx context.Context, conn net.Conn) {
	defer conn.Close()

	reader := bufio.NewReader(conn)
	writer := bufio.NewWriter(conn)

	logger := slog.With(slog.String("remote_addr", conn.RemoteAddr().String()))

	logger.InfoContext(ctx, "Accepted connection")

	for {
		// Check if the context has been cancelled.
		if ctx.Err() != nil {
			logger.ErrorContext(ctx, "Closing connection", slog.String("error", ctx.Err().Error()))
			return
		}

		// Read the packet header.
		var packetHeader uint16
		if err := binary.Read(reader, binary.LittleEndian, &packetHeader); err != nil {
			logger.ErrorContext(ctx, "Reading packet header", slog.String("error", err.Error()))
			return
		}

		logger.InfoContext(ctx, "<- Received packet", slog.String("header", fmt.Sprintf("0x%x", packetHeader)))

		// See if there is a packet registered for that header.
		packetReader, ok := packet.PacketFactory.Reader[packetHeader]
		if !ok {
			logger.ErrorContext(ctx, "No factory struct for packet reader")
			return
		}

		// Decode the raw bytes into a struct.
		if _, err := packetReader.ReadFrom(reader); err != nil {
			logger.ErrorContext(ctx, "Decoding raw packet", slog.String("error", err.Error()))
			return
		}

		// Handle the packet use-case.
		// Do we always respond? If so, is it always req-res? What about packets that need to be broadcasted?
		packetWriter, err := s.packetHandler.Handle(packetReader)
		if err != nil {
			logger.ErrorContext(ctx, "Handling packet", slog.String("error", err.Error()))
			return
		}

		// Find the header for the response packet.
		writerHeader, ok := packet.PacketFactory.Writer[reflect.TypeOf(packetWriter)]
		if !ok {
			logger.ErrorContext(ctx, "No factory struct for packet writer")
			return
		}

		// Write header. Because it is buffered, may be okay to do this here? Would allow supporting N packet versions.
		if err := binary.Write(writer, binary.LittleEndian, writerHeader); err != nil {
			logger.ErrorContext(ctx, "Writing packet header", slog.String("error", err.Error()))
			return
		}

		// Write the payload of the packet intot the buffer.
		if _, err := packetWriter.WriteTo(writer); err != nil {
			logger.ErrorContext(ctx, "Writing packet payload", slog.String("error", err.Error()))
			return
		}

		logger.InfoContext(ctx, "-> Sent packet", slog.String("header", fmt.Sprintf("0x%x", writerHeader)))
	}
}
